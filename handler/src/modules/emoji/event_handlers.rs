use std::collections::HashSet;

use ::chrono::{DateTime, Utc};
use sqlx::types::chrono;
use tracing::{debug, error, trace};
use twilight_gateway::Event;
use twilight_model::{
    channel::message::EmojiReactionType,
    gateway::payload::incoming::GuildCreate,
    id::{marker::EmojiMarker, Id},
};

use crate::context::EventContext;

use tulpje_framework::Error;
use tulpje_shared::is_pk_proxy;

use super::{
    db::{self, delete_emojis_not_in_list_for_guild},
    shared,
};

pub async fn handle_message(ctx: EventContext) -> Result<(), Error> {
    let Event::MessageCreate(msg) = &ctx.event else {
        unreachable!()
    };

    // only track messages in guilds
    let Some(guild_id) = msg.guild_id else {
        return Ok(());
    };

    // don't track PluralKit proxy messages
    if is_pk_proxy(&msg.application_id) {
        debug!("skipping PluralKit proxy message");
        return Ok(());
    }

    let timestamp = chrono::Utc::now();
    let emotes = shared::parse_emojis_from_string(guild_id, &msg.content);

    trace!(message = msg.content, emotes = ?emotes, "message");

    for emote in emotes {
        if shared::is_guild_emoji(&ctx.client, &ctx.services.cache, guild_id, *emote.id).await? {
            if let Err(err) = db::save_emoji_use(&ctx.services.db, &emote, timestamp).await {
                error!(err, guild_id = guild_id.get(), "db::save_emoji_use");
            };
        }
    }

    Ok(())
}

pub async fn message_update(ctx: EventContext) -> Result<(), Error> {
    let Event::MessageUpdate(evt) = &ctx.event else {
        unreachable!()
    };

    // TODO: Cache isn't implemented yet so we can't do stuff with message difference
    // trace!(has_old = ?old_message.is_some(), "message_update");
    //let Some(old_message) = old_message else {
    //    return;
    //};

    // TODO: We can't seem to check application_id here yet, this seems to be fixed in twilight HEAD though
    // // don't track PluralKit proxy messages
    // if is_pk_proxy(&evt.application_id) {
    //     debug!("skipping PluralKit proxy message");
    //     return;
    // }

    let Some(guild_id) = evt.guild_id else {
        // Don't process non-guild messages
        return Ok(());
    };

    let guild_emojis: HashSet<Id<EmojiMarker>> = ctx
        .client
        .emojis(guild_id)
        .await?
        .model()
        .await?
        .into_iter()
        .map(|e| e.id)
        .collect();

    let timestamp =
        DateTime::<Utc>::from_timestamp_micros(evt.timestamp.as_micros()).unwrap_or_else(Utc::now);

    // TODO: Once we implement cache compare the messages
    //       currently every edit considers every emoji a new one
    let old_emote_count = shared::count_emojis(
        shared::parse_emojis_from_string(guild_id, /* &old_message.content */ "")
            .into_iter()
            .filter(|e| guild_emojis.contains(&e.id))
            .collect::<Vec<db::Emoji>>(),
    );

    let new_emote_count = shared::count_emojis(
        shared::parse_emojis_from_string(guild_id, &evt.content)
            .into_iter()
            .filter(|e| guild_emojis.contains(&e.id))
            .collect::<Vec<db::Emoji>>(),
    );

    trace!(old = ?old_emote_count, new = ?new_emote_count, "message_update count");

    // Counting logic:
    //  In old but not new message? -> don't do anything, emote was "used"
    //  In both messages -> don't do anything, emote was "used"
    //  In new but not old message -> new "use" of emote

    for (emote, count) in new_emote_count {
        let change = count - old_emote_count.get(&emote).unwrap_or(&0);
        trace!(change = change, "message_update");

        if change <= 0 {
            // emote count has not incremented, don't need to track
            continue;
        }

        if let Err(err) = db::save_emoji_use(&ctx.services.db, &emote, timestamp).await {
            error!(
                err,
                guild_id = guild_id.get(),
                emote = ?emote,
                "db::save_emoji_use"
            );
        };
    }
    Ok(())
}

pub async fn reaction_add(ctx: EventContext) -> Result<(), Error> {
    let Event::ReactionAdd(reaction) = &ctx.event else {
        unreachable!()
    };

    debug!(reaction = ?reaction.emoji, "reaction_add");
    match &reaction.emoji {
        EmojiReactionType::Custom { animated, id, name } => {
            let now = chrono::Utc::now();
            let (Some(guild_id), Some(name)) = (reaction.guild_id, name) else {
                return Ok(());
            };

            if !shared::is_guild_emoji(&ctx.client, &ctx.services.cache, guild_id, *id).await? {
                return Ok(());
            }

            let emote = db::Emoji::new(*id, guild_id, name.clone(), *animated);

            if let Err(err) = db::save_emoji_use(&ctx.services.db, &emote, now).await {
                error!(err, "db::save_emoji_use");
            };
        }
        EmojiReactionType::Unicode { .. } => {
            // NOTE: We ignore unicode emojis, we're tracking emoji use to see which
            //       are underused, unicode emojis are global anyway
        }
    }

    Ok(())
}

pub(crate) async fn guild_create(ctx: EventContext) -> Result<(), Error> {
    let Event::GuildCreate(guild) = ctx.event else {
        unreachable!()
    };

    let GuildCreate::Available(ref guild) = *guild else {
        tracing::debug!(
            "skipping GuildCreate event for unavailable guild {}",
            guild.id()
        );
        return Ok(());
    };

    let count = delete_emojis_not_in_list_for_guild(
        &ctx.services.db,
        guild.id,
        guild.emojis.iter().map(|emoji| emoji.id).collect(),
    )
    .await?;
    tracing::debug!(
        "guild_create: cleaned up {} emojis for guild {}",
        count,
        guild.id
    );

    Ok(())
}

pub(crate) async fn guild_emojis_update(ctx: EventContext) -> Result<(), Error> {
    let Event::GuildEmojisUpdate(update) = &ctx.event else {
        unreachable!()
    };

    let count = delete_emojis_not_in_list_for_guild(
        &ctx.services.db,
        update.guild_id,
        update.emojis.iter().map(|emoji| emoji.id).collect(),
    )
    .await?;
    tracing::debug!(
        "guild_emojis_update: cleaned up {} emojis for guild {}",
        count,
        update.guild_id
    );

    Ok(())
}
