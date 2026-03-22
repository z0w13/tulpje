use std::{collections::HashSet, slice, sync::Arc};

use chrono::{DateTime, NaiveDateTime};
use pkrs_fork::{
    client::PkClient,
    model::{Member, PkId, Switch as PkSwitch},
};
use reqwest::StatusCode;
use serde_either::StringOrStruct;
use tulpje_cache::Cache;
use twilight_http::Client;
use twilight_model::{
    channel::message::Embed,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker},
    },
    util::Timestamp,
};

use tulpje_framework::Error;
use twilight_util::builder::embed::EmbedBuilder;

use self::pk::db::ModPkGuildRow;
use crate::{
    context::TaskContext,
    modules::{
        core,
        pk::{
            self,
            db::ModPkSystem,
            fronters::db,
            notify::db::{self as notify_db, get_notify_channel},
            util::get_member_name,
        },
    },
};

enum FrontChange {
    Unchanged,
    Changed(Switch),
}

struct Switch {
    pub(crate) fronters: Vec<Member>,
    pub(crate) timestamp: NaiveDateTime,
}

async fn update_system_fronters(
    db: &sqlx::PgPool,
    system: &ModPkSystem,
    client: &PkClient,
) -> Result<FrontChange, Error> {
    let current_front = match client
        .get_system_fronters(&PkId(system.uuid.to_string()))
        .await
    {
        Ok(front) => Ok::<PkSwitch, Error>(front),
        Err(err)
            if err
                .status()
                .is_some_and(|status| status == StatusCode::FORBIDDEN) =>
        {
            db::update_fronters_timestamp(db, system.uuid).await?;
            Err(err.into())
        }
        Err(err) => Err(err.into()),
    }?;
    let mut fronters = Vec::<Member>::new();
    for member in current_front.members {
        match member {
            StringOrStruct::String(_) => Err(format!(
                "system {} returned uuids instead of member structs",
                system.uuid
            ))?,
            StringOrStruct::Struct(member) => fronters.push(member),
        };
    }

    let fronter_uuids: Vec<_> = fronters.iter().map(|m| m.uuid).collect();

    if db::did_fronters_change(db, system.uuid, &fronter_uuids).await? {
        db::update_fronters(db, system.uuid, &fronter_uuids).await?;
        return Ok(FrontChange::Changed(Switch {
            fronters,
            timestamp: DateTime::from_timestamp(
                current_front.timestamp.to_utc().unix_timestamp(),
                0,
            )
            .ok_or_else(|| {
                format!(
                    "timestamp out of range: {}",
                    current_front.timestamp.to_utc().unix_timestamp()
                )
            })?
            .naive_utc(),
        }));
    }

    Ok(FrontChange::Unchanged)
}

async fn update_fronter_category(
    db: &sqlx::PgPool,
    discord_client: &Arc<Client>,
    cache: &Cache,
    enabled_guilds: &HashSet<Id<GuildMarker>>,
    system: &ModPkSystem,
    switch: &Switch,
) -> Result<(), Error> {
    let Some(guild_settings) = pk::db::get_guild_settings_for_system(db, &system.id).await? else {
        tracing::debug!(
            method = "update_fronter_category",
            "no guild with system {}, skipping",
            system.id
        );
        return Ok(());
    };

    metrics::counter!("pk:front-category", "type" => "total").increment(1);
    if !enabled_guilds.contains(&guild_settings.guild_id) {
        metrics::counter!("pk:front-category", "type" => "module-disabled").increment(1);
        tracing::debug!(
            method = "update_fronter_category",
            "Guild {} has pk module disabled, skipping",
            guild_settings.guild_id
        );
        return Ok(());
    }

    let Some(category_id) = db::get_fronter_category(db, *guild_settings.guild_id).await? else {
        metrics::counter!("pk:front-category", "type" => "category-missing").increment(1);
        tracing::debug!(
            method = "update_fronter_category",
            "no fronter category configured for guild {}, skipping",
            guild_settings.guild_id
        );
        return Ok(());
    };

    if let Err(err) = update_fronters_for_guild(
        discord_client,
        cache,
        &guild_settings,
        *category_id,
        &switch.fronters,
    )
    .await
    {
        metrics::counter!("pk:front-category", "type" => "error").increment(1);
        tracing::error!(
            method = "update_fronter_category",
            guild_id = ?guild_settings.guild_id,
            category_id = ?category_id,
            err
        );
    } else {
        metrics::counter!("pk:front-category", "type" => "success").increment(1);
    }
    Ok(())
}

const MAX_FRONTERS_IN_MESSAGE: usize = 20;
// TODO: Components V2
fn create_front_change_embed(system: &ModPkSystem, switch: &Switch) -> Result<Embed, Error> {
    let builder = EmbedBuilder::new().title(format!(
        "Switch: {}",
        system.name.as_ref().unwrap_or(&system.id)
    ));

    let mut embed_parts = Vec::new();
    for member in switch.fronters.iter().take(MAX_FRONTERS_IN_MESSAGE) {
        embed_parts.push(format!("* {}", get_member_name(member)));
    }

    if switch.fronters.len() > MAX_FRONTERS_IN_MESSAGE {
        embed_parts.push(format!(
            "-# and {} more",
            switch.fronters.len() - MAX_FRONTERS_IN_MESSAGE
        ));
    }

    Ok(builder
        .description(embed_parts.join("\n"))
        .timestamp(Timestamp::from_secs(
            switch.timestamp.and_utc().timestamp(),
        )?)
        .validate()?
        .build())
}

async fn notify_front_change(
    db: &sqlx::PgPool,
    discord_client: &Arc<Client>,
    enabled_guilds: &HashSet<Id<GuildMarker>>,
    system: &ModPkSystem,
    switch: &Switch,
) -> Result<(), Error> {
    let embed = create_front_change_embed(system, switch)?;

    let guilds = notify_db::get_notify_guilds_for_system(db, system.uuid).await?;
    tracing::debug!(
        method = "notify_front_change",
        "notifying {} guilds of front change in {}",
        guilds.len(),
        system.id
    );
    for guild_id in guilds {
        metrics::counter!("pk:notifications", "type" => "total").increment(1);
        tracing::debug!(
            method = "notify_front_change",
            "notifying guild {} of front change in {}",
            guild_id,
            system.id
        );
        if !enabled_guilds.contains(&guild_id) {
            metrics::counter!("pk:notifications", "type" => "module-disabled").increment(1);
            tracing::debug!(
                method = "notify_front_change",
                "not notifying guild {} of front change in {} pk module is disabled",
                guild_id,
                system.uuid
            );
            continue;
        }

        let Some(channel_id) = get_notify_channel(db, *guild_id).await? else {
            metrics::counter!("pk:notifications", "type" => "channel-missing").increment(1);
            tracing::warn!(
                method = "notify_front_change",
                "no notify channel configured for guild {} despite it having tracked systems",
                guild_id,
            );
            continue;
        };

        if let Err(err) = discord_client
            .create_message(*channel_id)
            .embeds(slice::from_ref(&embed))
            .await
        {
            metrics::counter!("pk:notifications", "type" => "error").increment(1);
            tracing::warn!(
                method = "notify_front_change",
                "error sending front change notification to guild {} channel {}: {}",
                guild_id,
                channel_id,
                err
            );
        } else {
            metrics::counter!("pk:notifications", "type" => "success").increment(1);
        }
    }
    Ok(())
}

async fn process_system(
    db: &sqlx::PgPool,
    pk_client: &PkClient,
    discord_client: &Arc<Client>,
    cache: &Cache,
    enabled_guilds: &HashSet<Id<GuildMarker>>,
    system: &ModPkSystem,
) -> Result<(), Error> {
    let changed = update_system_fronters(db, system, pk_client).await?;
    match changed {
        FrontChange::Changed(switch) => {
            update_fronter_category(db, discord_client, cache, enabled_guilds, system, &switch)
                .await?;
            notify_front_change(db, discord_client, enabled_guilds, system, &switch).await?;
        }
        FrontChange::Unchanged => {}
    }
    Ok(())
}

pub(crate) async fn update_fronters(ctx: TaskContext) -> Result<(), Error> {
    let system_count = db::get_system_count(&ctx.services.db).await?;
    metrics::counter!("pk:tracked-systems").absolute(system_count as u64);

    if system_count > 100 {
        tracing::warn!(
            ?system_count,
            "system update mechanism overloads after 100 systems"
        );
    }

    let systems_to_update = db::get_systems_to_update(&ctx.services.db).await?;
    let pk_client = PkClient::default();
    let pk_guilds: HashSet<Id<GuildMarker>> =
        core::db::guilds_with_module(&ctx.services.db, "pluralkit")
            .await?
            .into_iter()
            .collect();

    for system in &systems_to_update {
        if let Err(err) = process_system(
            &ctx.services.db,
            &pk_client,
            &ctx.client,
            &ctx.services.cache,
            &pk_guilds,
            system,
        )
        .await
        {
            tracing::warn!("error updating system with uuid {}, {}", system.uuid, err);
        }
    }

    Ok(())
}

async fn update_fronters_for_guild(
    client: &Client,
    cache: &Cache,
    guild_settings: &ModPkGuildRow,
    category_id: Id<ChannelMarker>,
    members: &[Member],
) -> Result<(), Error> {
    let guild = client
        .guild(*guild_settings.guild_id)
        .await?
        .model()
        .await?;

    let category = client
        .channel(category_id)
        .await
        .map_err(|err| {
            format!(
                "couldn't find category for guild '{}' ({}) {}",
                guild.name, guild.id, err
            )
        })?
        .model()
        .await?;

    category.guild_id.ok_or_else(|| {
        format!(
            "channel {} for guild '{}' ({}) isn't a guild channel",
            category.id, guild.name, guild.id
        )
    })?;

    super::commands::update_fronter_channels(
        client,
        cache,
        guild.clone(),
        guild_settings,
        category,
        Some(members),
    )
    .await
    .map_err(|err| {
        format!(
            "error updating fronters for {} ({}): {}",
            guild.name, guild.id, err
        )
    })?;

    tracing::info!(
        guild.id = guild.id.get(),
        guild.name = guild.name,
        "fronters updated"
    );

    Ok(())
}
