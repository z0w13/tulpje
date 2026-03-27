use chrono::NaiveDateTime;
use tulpje_framework::Error;
use tulpje_lib::context::TaskContext;
use twilight_model::id::{Id, marker::GuildMarker};

use crate::modules::core::db;

#[tracing::instrument(level = "error", skip_all)]
pub(super) async fn delete_removed_guilds(ctx: TaskContext) -> Result<(), Error> {
    for (guild_id, deleted_at) in db::guilds_eligible_for_deletion(&ctx.services.db).await? {
        if let Err(err) = delete_guild(&ctx.services.db, guild_id, deleted_at).await {
            tracing::error!("error deleting guild `{guild_id}`: {err}");
        }
    }

    Ok(())
}

pub(super) async fn delete_guild(
    _db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    deleted_at: NaiveDateTime,
) -> Result<(), Error> {
    tracing::info!("deleting guild `{guild_id}` which was deleted at {deleted_at}");
    tracing::warn!(
        "delete_guild is running in dry run mode until foreign keys and cascades are configured correctly"
    );

    //if !db::delete_guild(db, guild_id).await? {
    //    tracing::warn!("tried deleting guild {guild_id} but nothing was deleted");
    //}

    Ok(())
}
