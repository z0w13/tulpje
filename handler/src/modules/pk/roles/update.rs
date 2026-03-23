use std::collections::{HashMap, HashSet};

use pkrs_fork::{client::PkClient, model::PkId};
use tracing::debug;
use twilight_model::guild::Guild;
use twilight_model::id::Id;
use twilight_model::id::marker::RoleMarker;

use tulpje_framework::Error;

use crate::context::CommandContext;
use crate::modules::pk::{
    db::get_guild_settings_for_id,
    util::{get_member_name, pk_color_to_discord},
};
use crate::util::error_response;

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

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    ctx.defer_ephemeral().await?; // delay responding and make reply ephemeral

    let Some(gs) = get_guild_settings_for_id(&ctx.services.db, guild.id).await? else {
        ctx.update("PluralKit module not set-up, please run /pk setup")
            .await?;
        return Ok(());
    };
    let system_id = PkId(gs.system_id);

    // TODO: Actually check based on the number of roles in the server
    let member_count = ctx.services.pk.get_system_members(&system_id).await?.len();
    if member_count > DISCORD_ROLE_LIMIT.saturating_sub(ROLE_BUFFER) {
        error_response(&ctx, &role_limit_message(member_count)).await?;
        return Ok(());
    }

    let token = ctx.get_arg_string_optional("token")?;
    let current_role_map = get_current_roles(guild.clone());
    let desired_role_map =
        get_desired_roles(&ctx.services.pk, &system_id, token.unwrap_or_default()).await?;
    let ops = get_ops(&current_role_map, &desired_role_map);

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
                ctx.client
                    .create_role(guild.id)
                    .name(name)
                    .color(*color)
                    .await
                    .map_err(|err| format!("error creating role {name}: {err}"))?;

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

    // aggregate stats
    let (created, deleted, updated) =
        ops.into_iter()
            .fold((0, 0, 0), |(created, deleted, updated), op| match op {
                ChangeOperation::Create { .. } => (created + 1, deleted, updated),
                ChangeOperation::Delete { .. } => (created, deleted + 1, updated),
                ChangeOperation::Update { .. } => (created, deleted, updated + 1),
            });

    ctx.update(format!(
        "roles updated, {} created, {} deleted, {} updated",
        created, deleted, updated
    ))
    .await?;
    Ok(())
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

async fn get_desired_roles(
    pk: &PkClient,
    system: &PkId,
    token: String,
) -> Result<HashMap<String, MemberRole>, Error> {
    let roles = pk
        .with_token(token)
        .get_system_members(system)
        .await?
        .into_iter()
        .map(|m| MemberRole {
            id: None,
            name: format!(
                "{} (Alter)",
                get_member_name(&m)
                    .split(" (") // Remove parenthesised pronouns ' (she/her)' and such
                    .next() // get the first part of the split string
                    .unwrap()
            ),
            color: pk_color_to_discord(m.color),
        })
        .map(|r| (r.name.clone(), r))
        .collect();

    Ok(roles)
}

fn get_current_roles(guild: Guild) -> HashMap<String, MemberRole> {
    guild
        .roles
        .into_iter()
        .filter(|v| v.name.ends_with(" (Alter)"))
        .map(|v| MemberRole {
            id: Some(v.id),
            name: v.name.clone(),
            color: v.colors.primary_color,
        })
        .map(|v| (v.name.clone(), v))
        .collect()
}

fn get_ops(
    current: &HashMap<String, MemberRole>,
    desired: &HashMap<String, MemberRole>,
) -> Vec<ChangeOperation> {
    let all_roles: HashSet<&String> = current.keys().chain(desired.keys()).collect();

    all_roles
        .into_iter()
        .filter_map(|role| {
            match (current.get(role), desired.get(role)) {
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
