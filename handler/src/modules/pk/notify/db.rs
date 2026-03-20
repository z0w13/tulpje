use tulpje_framework::Error;
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, GuildMarker},
};
use uuid::Uuid;

use crate::db::DbId;

pub(crate) async fn save_notify_channel(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    channel_id: Id<ChannelMarker>,
) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO pk_notify_channels (guild_id, channel_id) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET channel_id = $2",
        i64::from(DbId(guild_id)),
        i64::from(DbId(channel_id)),
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(crate) async fn get_notify_channel(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
) -> Result<Option<DbId<ChannelMarker>>, Error> {
    let result = sqlx::query_scalar!(
        "SELECT channel_id FROM pk_notify_channels WHERE guild_id = $1",
        i64::from(DbId(guild_id)),
    )
    .fetch_optional(db)
    .await?;

    Ok(result.map(DbId::<ChannelMarker>::from))
}

pub(crate) async fn add_notify_system(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    system_uuid: Uuid,
) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO pk_notify_systems (guild_id, system_uuid) VALUES ($1, $2) ON CONFLICT (guild_id, system_uuid) DO UPDATE SET system_uuid = $2",
        i64::from(DbId(guild_id)),
        system_uuid,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(crate) async fn remove_notify_system(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    system_uuid: Uuid,
) -> Result<(), Error> {
    sqlx::query!(
        "DELETE FROM pk_notify_systems WHERE guild_id = $1 AND system_uuid = $2",
        i64::from(DbId(guild_id)),
        system_uuid,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(crate) async fn get_notify_systems(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
) -> Result<Vec<Uuid>, Error> {
    Ok(sqlx::query_scalar!(
        "SELECT system_uuid FROM pk_notify_systems WHERE guild_id = $1",
        i64::from(DbId(guild_id)),
    )
    .fetch_all(db)
    .await?)
}

pub(crate) async fn get_notify_guilds_for_system(
    db: &sqlx::PgPool,
    system: Uuid,
) -> Result<Vec<DbId<GuildMarker>>, Error> {
    Ok(sqlx::query_scalar!(
        "SELECT guild_id FROM pk_notify_systems WHERE system_uuid = $1",
        system
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(Into::into)
    .collect())
}
