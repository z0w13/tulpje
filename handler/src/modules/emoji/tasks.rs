use futures_util::TryStreamExt as _;

use twilight_http::Client;
use twilight_model::id::{
    marker::{EmojiMarker, GuildMarker},
    Id,
};

use tulpje_framework::Error;

use super::db;
use crate::context::TaskContext;

pub(crate) async fn clean_deleted_emojis(ctx: TaskContext) -> Result<(), Error> {
    tracing::info!("cleaning emojis for guilds...");
    let mut guild_ids = db::get_tracked_guilds(&ctx.services.db);

    while let Some(guild_id) = guild_ids.try_next().await? {
        match clean_deleted_emojis_for_guild(&ctx.services.db, &ctx.client, *guild_id).await {
            Ok(count) => tracing::info!("cleaned {} emojis for guild {}", count, guild_id),
            Err(err) => tracing::warn!(
                "error cleaning deleted emojis for guild {}: {}",
                guild_id,
                err
            ),
        }
    }

    Ok(())
}

pub(crate) async fn clean_deleted_emojis_for_guild(
    db: &sqlx::PgPool,
    client: &Client,
    guild_id: Id<GuildMarker>,
) -> Result<u64, Error> {
    let emoji_ids: Vec<Id<EmojiMarker>> = client
        .emojis(guild_id)
        .await?
        .models()
        .await?
        .into_iter()
        .map(|emoji| emoji.id)
        .collect();

    db::delete_emojis_not_in_list_for_guild(db, guild_id, emoji_ids).await
}
