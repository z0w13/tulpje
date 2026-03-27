use std::collections::{HashMap, HashSet};

use pkrs_fork::model::Member;
use pkrs_fork::{client::PkClient, model::PkId};
use reqwest::StatusCode;
use tracing::debug;
use tulpje_cache::Cache;
use twilight_http::Client;
use twilight_model::guild::{Guild, Role};
use twilight_model::id::Id;
use twilight_model::id::marker::{GuildMarker, RoleMarker, UserMarker};

use tulpje_framework::Error;

use crate::context::CommandContext;
use crate::modules::pk::util::SystemRef;
use crate::modules::pk::{
    db::get_guild_settings_for_id,
    util::{get_member_name, pk_color_to_discord},
};
use crate::responses;

// Discord's role limit
// see https://support.discord.com/hc/en-us/articles/33694251638295-Discord-Account-Caps-Server-Caps-and-More
const DISCORD_ROLE_LIMIT: usize = 250;
const ROLE_BUFFER: usize = 25;

fn role_limit_message(member_count: usize) -> String {
    format!(
        "### Error\n\
        Can't create member roles, discord has a role limit of {DISCORD_ROLE_LIMIT} \
        and your system has {member_count} (visible) members\n\n\
        Additionally we keep a buffer of {ROLE_BUFFER} just in case"
    )
}

async fn handle_update_success_message(
    ctx: &CommandContext,
    created: u16,
    deleted: u16,
    updated: u16,
    assigned: u16,
) -> Result<(), Error> {
    if created + deleted + updated + assigned == 0 {
        responses::info(ctx, "Member roles are already up-to-date").await?;
        return Ok(());
    }

    // all this code is just to get messages to look like
    //   1 role created, 2 updated, 1 assigned
    //   2 roles updated, 1 assigned
    //   etc
    let mut parts = Vec::<(u16, &'static str)>::new();
    if created > 0 {
        parts.push((created, "created"));
    }
    if updated > 0 {
        parts.push((updated, "updated"));
    }
    if deleted > 0 {
        parts.push((deleted, "deleted"));
    }
    if assigned > 0 {
        parts.push((assigned, "assigned"));
    }

    responses::success(
        ctx,
        &format!(
            "### Member Roles Updated\n{}",
            parts
                .into_iter()
                .enumerate()
                .map(|(idx, (count, verb))| {
                    if idx == 0 {
                        let noun = if count == 1 { "role" } else { "roles" };
                        format!("{count} {noun} {verb}")
                    } else {
                        format!("{count} {verb}")
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        ),
    )
    .await?;

    Ok(())
}

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    ctx.defer_ephemeral().await?; // delay responding and make reply ephemeral

    let Some(gs) = get_guild_settings_for_id(&ctx.services.db, guild.id).await? else {
        responses::error(
            &ctx,
            "### Error\nPluralKit module not set-up, please run `/pk setup`",
        )
        .await?;
        return Ok(());
    };
    let system_ref = SystemRef::Id(gs.system_id);
    let token = ctx.get_arg_string_optional("token")?;

    // fetch members from PluralKit
    let Some(members) = handle_get_system_members(
        &ctx,
        &ctx.services.pk.with_token(token.unwrap_or_default()),
        system_ref,
    )
    .await?
    else {
        return Ok(());
    };

    // TODO: Actually check based on the number of roles in the server
    let member_count = members.len();
    if member_count > DISCORD_ROLE_LIMIT.saturating_sub(ROLE_BUFFER) {
        responses::error(&ctx, &role_limit_message(member_count)).await?;
        return Ok(());
    }

    // get current and desired server roles
    let current_role_map = get_current_roles(&guild);
    let desired_role_map = get_desired_roles(&members);

    // get current and desired assigned roles for user
    let current_user_roles =
        get_user_roles(&ctx.client, &ctx.services.cache, *gs.guild_id, *gs.user_id).await?;

    let current_user_role_names: HashSet<_> = current_user_roles
        .iter()
        .filter(|r| r.name.ends_with("(Alter)"))
        .map(|r| r.name.clone())
        .collect();
    let desired_user_role_names: HashSet<_> = desired_role_map.keys().cloned().collect();
    let missing_user_role_names: Vec<_> = desired_user_role_names
        .difference(&current_user_role_names)
        .collect();

    let ops = get_role_ops(&current_role_map, &desired_role_map);

    let mut role_name_id_map: HashMap<String, Id<RoleMarker>> = current_role_map
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                v.id.expect("get_current_roles always assigns id: Some(...)"),
            )
        })
        .collect();

    // TODO: actually handle errors
    // TODO: set mention permissions?
    for op in &ops {
        match op {
            ChangeOperation::Update { id, name, color } => {
                ctx.client
                    .update_role(guild.id, *id)
                    .color(Some(*color))
                    .await
                    .map_err(|err| format!("error updating role {name} ({id}): {err}"))?;

                debug!(
                    guild_id = guild.id.get(),
                    guild_name = guild.name,
                    "updated role: {}",
                    name,
                );
            }
            ChangeOperation::Create { name, color } => {
                let role = ctx
                    .client
                    .create_role(guild.id)
                    .name(name)
                    .color(*color)
                    .await
                    .map_err(|err| format!("error creating role {name}: {err}"))?
                    .model()
                    .await
                    .map_err(|err| format!("error parsing role {name}: {err}"))?;

                role_name_id_map.insert(name.clone(), role.id);

                debug!(
                    guild_id = guild.id.get(),
                    guild_name = guild.name,
                    "created role: {}",
                    name
                );
            }
            ChangeOperation::Delete { id, name } => {
                ctx.client
                    .delete_role(guild.id, *id)
                    .await
                    .map_err(|err| format!("error deleting role {name} ({id}): {err}"))?;

                debug!(
                    guild_id = guild.id.get(),
                    guild_name = guild.name,
                    "deleted_role: {}",
                    name
                );
            }
        };
    }

    for missing_role_name in &missing_user_role_names {
        let Some(role_id) = role_name_id_map.get(*missing_role_name) else {
            tracing::warn!("couldn't get role id from `role_name_id_map` for {missing_role_name}");
            continue;
        };

        ctx.client
            .add_guild_member_role(*gs.guild_id, *gs.user_id, *role_id)
            .await
            .map_err(|err| {
                format!("error assigning role {missing_role_name} ({role_id}): {err}")
            })?;

        debug!(
            guild_id = guild.id.get(),
            guild_name = guild.name,
            "assigned role: {missing_role_name}"
        );
    }

    // aggregate stats
    let (created, deleted, updated) =
        ops.into_iter()
            .fold((0, 0, 0), |(created, deleted, updated), op| match op {
                ChangeOperation::Create { .. } => (created + 1, deleted, updated),
                ChangeOperation::Delete { .. } => (created, deleted + 1, updated),
                ChangeOperation::Update { .. } => (created, deleted, updated + 1),
            });

    // send success message to user
    handle_update_success_message(
        &ctx,
        created,
        deleted,
        updated,
        missing_user_role_names.len() as u16,
    )
    .await?;

    Ok(())
}

async fn handle_get_system_members(
    ctx: &CommandContext,
    client: &PkClient,
    system_ref: SystemRef,
) -> Result<Option<Vec<Member>>, Error> {
    match client
        .get_system_members(&PkId(system_ref.clone().into()))
        .await
    {
        Ok(members) => Ok(Some(members)),
        // private member list
        Err(err)
            if err
                .status()
                .is_some_and(|status| status == StatusCode::FORBIDDEN) =>
        {
            responses::error(
                ctx,
                &format!("### Error\nMember list for `{system_ref}` is private"),
            )
            .await?;
            Ok(None)
        }
        // missing system
        Err(err)
            if err
                .status()
                .is_some_and(|status| status == StatusCode::NOT_FOUND) =>
        {
            responses::error(
                    ctx,
                    &format!("### Error\nPluralKit returned a `404 Not Found` error, does `{system_ref}` exist?"),
                )
                .await?;
            Ok(None)
        }
        // miscellaneous errors
        Err(err) => Err(err.into()),
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct MemberRole {
    id: Option<Id<RoleMarker>>,
    name: String,
    color: u32,
}

enum ChangeOperation {
    Create {
        name: String,
        color: u32,
    },
    Delete {
        id: Id<RoleMarker>,
        name: String,
    },
    Update {
        id: Id<RoleMarker>,
        name: String,
        color: u32,
    },
}

// TODO: Persist updated info in the cache
async fn get_user_roles(
    client: &Client,
    cache: &Cache,
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
) -> Result<Vec<Role>, Error> {
    let member_roles = if let Some(member) = cache.members.get(&(guild_id, user_id)).await? {
        member.roles
    } else {
        client
            .guild_member(guild_id, user_id)
            .await?
            .model()
            .await?
            .roles
    };

    let mut roles = Vec::new();
    for role_id in member_roles {
        roles.push(if let Some(role) = cache.roles.get(&role_id).await? {
            role.inner()
        } else {
            client.role(guild_id, role_id).await?.model().await?
        });
    }

    Ok(roles)
}

fn get_desired_roles(members: &[Member]) -> HashMap<String, MemberRole> {
    members
        .iter()
        .map(|m| MemberRole {
            id: None,
            name: format!(
                "{} (Alter)",
                get_member_name(m)
                    .split(" (") // Remove parenthesised pronouns ' (she/her)' and such
                    .next() // get the first part of the split string
                    .unwrap()
            ),
            color: pk_color_to_discord(m.color.clone()),
        })
        .map(|r| (r.name.clone(), r))
        .collect()
}

fn get_current_roles(guild: &Guild) -> HashMap<String, MemberRole> {
    guild
        .roles
        .iter()
        .filter(|v| v.name.ends_with(" (Alter)"))
        .map(|v| MemberRole {
            id: Some(v.id),
            name: v.name.clone(),
            color: v.colors.primary_color,
        })
        .map(|v| (v.name.clone(), v))
        .collect()
}

fn get_role_ops(
    current_roles: &HashMap<String, MemberRole>,
    desired_roles: &HashMap<String, MemberRole>,
) -> Vec<ChangeOperation> {
    let all_roles: HashSet<&String> = current_roles.keys().chain(desired_roles.keys()).collect();

    all_roles
        .into_iter()
        .filter_map(|role| {
            match (current_roles.get(role), desired_roles.get(role)) {
                // Update, only if color changed
                (Some(current), Some(desired)) => {
                    (current.color != desired.color).then(|| ChangeOperation::Update {
                        id: current.id.unwrap(),
                        name: current.name.clone(),
                        color: desired.color,
                    })
                }
                // Create
                (None, Some(desired)) => Some(ChangeOperation::Create {
                    name: desired.name.clone(),
                    color: desired.color,
                }),
                // Delete
                (Some(current), None) => Some(ChangeOperation::Delete {
                    id: current.id.unwrap(),
                    name: current.name.clone(),
                }),
                // Shit got fucked up aaaa
                (None, None) => panic!("current and desired are both None, shouldn't happen"),
            }
        })
        .collect()
}
