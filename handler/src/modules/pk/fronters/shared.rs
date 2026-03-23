use std::collections::{HashMap, HashSet};

use pkrs_fork::model::Member;
use pkrs_fork::{client::PkClient, model::PkId};
use reqwest::StatusCode;
use serde_either::StringOrStruct;
use tracing::{Level, error};
use tulpje_cache::Cache;
use twilight_http::Client;
use twilight_model::channel::permission_overwrite::{PermissionOverwrite, PermissionOverwriteType};
use twilight_model::channel::{Channel, ChannelType};
use twilight_model::guild::{Guild, Permissions};
use twilight_model::id::Id;
use twilight_model::id::marker::{ChannelMarker, GenericMarker, GuildMarker};

use tulpje_framework::Error;

use crate::context::CommandContext;
use crate::modules::pk::util::SystemRef;
use crate::modules::pk::{db::ModPkGuildRow, util::get_member_name};
use crate::util::error_response;

pub(super) async fn get_desired_fronters(system: &PkId) -> Result<Vec<String>, Error> {
    let pk = PkClient::default();

    Ok(pk
        .get_system_fronters(system)
        .await?
        .map_or_else(Vec::new, |switch| {
            switch
                .members
                .into_iter()
                .filter_map(|m| match m {
                    StringOrStruct::String(_) => None,
                    StringOrStruct::Struct(member) => Some(get_member_name(&member)),
                })
                .collect()
        }))
}

pub(super) async fn get_fronter_channels(
    client: &Client,
    cache: &Cache,
    guild: Id<GuildMarker>,
    cat_id: Id<ChannelMarker>,
) -> Result<Vec<Channel>, Error> {
    // try fetching channels from cache first
    let channel_ids = cache.guild_channels.members(&guild).await?;
    if !channel_ids.is_empty() {
        let mut channels = Vec::new();
        for channel_id in channel_ids {
            // if the channel isn't in the cache log a warning and try to fetch it from discord
            let channel = if let Some(channel) = cache.channels.get(&channel_id).await? {
                channel
            } else {
                tracing::warn!(
                    ?channel_id,
                    "channel in `guild_channels` cache but missing in `channels`, this shouldn't happen"
                );
                match client.channel(channel_id).await?.model().await {
                    Ok(channel) => channel,
                    Err(err) => {
                        tracing::warn!(
                            ?channel_id,
                            ?err,
                            "channel in `guild_channels` cache but an error occured when fetching from discord"
                        );
                        continue;
                    }
                }
            };
            if channel
                .parent_id
                .is_some_and(|parent_id| parent_id == cat_id)
            {
                channels.push(channel);
            }
        }

        Ok(channels)
    } else {
        Ok(client
            .guild_channels(guild)
            .await?
            .models()
            .await?
            .into_iter()
            .filter(|c| c.parent_id.is_some_and(|parent_id| parent_id == cat_id))
            .collect())
    }
}

pub(super) async fn get_fronter_category(
    client: &Client,
    guild: &Guild,
    opt_cat_name: Option<String>,
) -> Result<Option<Channel>, Error> {
    let cat_name = opt_cat_name
        .unwrap_or_else(|| "current fronters".into())
        .to_lowercase();

    Ok(client
        .guild_channels(guild.id)
        .await?
        .models()
        .await?
        .into_iter()
        .find(|c| {
            c.name
                .clone()
                .expect("guild channels have names")
                .to_lowercase()
                == cat_name
                && c.kind == ChannelType::GuildCategory
        }))
}

/// output additional debugging information to debug issues with fronter order
fn debug_fronter_order(
    guild: &Guild,
    fronter_channels: &[Channel],
    desired_fronters: &[String],
    fronter_pos_map: &HashMap<String, u16>,
) {
    tracing::trace!("fronters for '{}' ({})", guild.name, guild.id);
    tracing::trace!("  fronter_channels");
    let mut sorted_fronter_channels: Vec<_> = fronter_channels.to_vec();
    sorted_fronter_channels.sort_by_key(|c| c.position);

    for channel in sorted_fronter_channels {
        tracing::trace!(
            "    - channel {} ({}) position {:?}",
            channel.name.clone().unwrap_or_default(),
            channel.id,
            channel.position,
        );
    }

    tracing::trace!("  desired_fronters");
    for (position, fronter) in desired_fronters.iter().enumerate() {
        tracing::trace!("    - fronter {fronter} position {position}");
    }

    tracing::trace!("  fronter_pos_map");
    let mut sorted_fronter_pos_map: Vec<(String, u16)> =
        fronter_pos_map.clone().into_iter().collect();
    sorted_fronter_pos_map.sort_by_key(|f| f.1);

    for (fronter, pos) in &sorted_fronter_pos_map {
        tracing::trace!("    - fronter {fronter} position {pos}");
    }
}

pub(super) async fn update_fronter_channels(
    client: &Client,
    cache: &Cache,
    guild: Guild,
    gs: &ModPkGuildRow,
    cat: Channel,
    members: Option<&[Member]>,
) -> Result<(), Error> {
    // get the bot's user id
    let user_id = client.current_user().await?.model().await?.id;

    let fronter_channels = get_fronter_channels(client, cache, guild.id, cat.id).await?;
    let desired_fronters = if let Some(members) = members {
        members.iter().map(get_member_name).collect()
    } else {
        get_desired_fronters(&PkId(gs.system_id.clone())).await?
    };

    let current_fronters: HashSet<String> = fronter_channels
        .iter()
        .map(|c| c.name.clone().expect("guild channels have names"))
        .collect();

    let mut fronter_channel_map: HashMap<String, Channel> = fronter_channels
        .iter()
        .map(|c| {
            (
                c.name.clone().expect("guild channels have names"),
                c.to_owned(),
            )
        })
        .collect();

    let fronter_pos_map: HashMap<String, u16> = desired_fronters
        .iter()
        .enumerate()
        // WARN: could this result in a panic/error? usize into u16
        .map(|(k, v)| (v.to_owned(), k.try_into().unwrap()))
        .collect();

    if tracing::event_enabled!(Level::TRACE) {
        debug_fronter_order(
            &guild,
            &fronter_channels,
            &desired_fronters,
            &fronter_pos_map,
        );
    }

    let desired_fronters_set = HashSet::from_iter(desired_fronters);
    let delete_fronters = current_fronters.difference(&desired_fronters_set);
    let create_fronters = desired_fronters_set.difference(&current_fronters);

    // TODO: Use something like thiserror to narrow down error types.
    //       that way we can give users better errors and also not just, string join
    let mut fronter_errors = Vec::new();

    for fronter in delete_fronters {
        #[expect(
            clippy::indexing_slicing,
            reason = "`delete_fronters` should only contain keys from `fronter_channel_map`"
        )]
        let channel = &fronter_channel_map[fronter];
        if let Err(e) = client.delete_channel(channel.id).await {
            let err = format!(
                "error deleting channel '{}' ({}): {}",
                channel.name.clone().unwrap_or_default(),
                channel.id,
                e
            );
            error!("{err}");
            fronter_errors.push(err);

            continue;
        }

        fronter_channel_map.remove(fronter);
    }

    for fronter in create_fronters {
        let pos = fronter_pos_map
            .get(fronter)
            .expect("couldn't get position for fronter, this should never happen!");

        let permissions = vec![
            PermissionOverwrite {
                deny: Permissions::CONNECT,
                allow: Permissions::empty(),
                id: guild.id.cast(),
                kind: PermissionOverwriteType::Role,
            },
            PermissionOverwrite {
                allow: Permissions::CONNECT,
                deny: Permissions::empty(),
                id: user_id.cast::<GenericMarker>(),
                kind: PermissionOverwriteType::Member,
            },
        ];

        let channel = match client
            .create_guild_channel(guild.id, fronter)
            .permission_overwrites(&permissions)
            .position(u64::from(*pos))
            .parent_id(cat.id)
            .kind(ChannelType::GuildVoice)
            .await
        {
            Ok(response) => match response.model().await {
                Ok(chan) => chan,
                Err(e) => {
                    let err = format!("error deserialising new channel for '{fronter}': {e}");
                    error!("{err}");
                    fronter_errors.push(err);

                    continue;
                }
            },
            Err(e) => {
                let err = format!("error creating fronter channel '{fronter}': {e}");
                error!("{err}");
                fronter_errors.push(err);

                continue;
            }
        };

        fronter_channel_map.insert(fronter.to_owned(), channel.clone());
    }
    for (name, position) in fronter_pos_map {
        let channel = fronter_channel_map
            .get(&name)
            .expect("couldn't get channel from fronter_channel_map, this should never happen!")
            .to_owned();

        if channel.position.is_some_and(|p| p == i32::from(position)) {
            continue;
        }

        if let Err(e) = client
            .update_channel(channel.id)
            .position(u64::from(position))
            .await
        {
            let err = format!("error moving channel '{name}': {e}");
            error!("{err}");
            fronter_errors.push(err);

            continue;
        }
    }

    if !fronter_errors.is_empty() {
        return Err(fronter_errors.join("\n").into());
    }

    Ok(())
}

pub(super) async fn create_or_get_fronter_channel(
    client: &Client,
    guild: &Guild,
    cat_name: String,
) -> Result<Channel, Error> {
    if let Some(fronters_category) =
        get_fronter_category(client, guild, Some(cat_name.clone())).await?
    {
        return Ok(fronters_category);
    }

    // get the bot's user id
    let user_id = client.current_user().await?.model().await?.id;

    // define permissions
    let permissions = vec![
        PermissionOverwrite {
            deny: Permissions::VIEW_CHANNEL,
            allow: Permissions::empty(),
            id: guild.id.cast(),
            kind: PermissionOverwriteType::Role,
        },
        PermissionOverwrite {
            allow: Permissions::MANAGE_CHANNELS | Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            id: user_id.cast::<GenericMarker>(),
            kind: PermissionOverwriteType::Member,
        },
    ];

    Ok(client
        .create_guild_channel(guild.id, &cat_name)
        .permission_overwrites(&permissions)
        .kind(ChannelType::GuildCategory)
        .await?
        .model()
        .await?)
}

// check whether a system's front is private and if so
// inform the user with the specified message.
//
// returns true if front is private, with the assumption
// the calling functions early returns after
pub(crate) async fn handle_private_front(
    ctx: &CommandContext,
    pk_client: &PkClient,
    system_ref: SystemRef,
    message: &str,
) -> Result<bool, Error> {
    if let Err(err) = pk_client
        .get_system_fronters(&PkId(system_ref.into()))
        .await
    {
        // inform user front is private and return
        if let Some(status) = err.status()
            && status == StatusCode::FORBIDDEN
        {
            error_response(ctx, message).await?;
            return Ok(true);
        }

        // propagate any other errors
        return Err(err.into());
    };

    Ok(false)
}
