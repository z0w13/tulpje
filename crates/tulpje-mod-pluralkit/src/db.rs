use pkrs_fork::model::System;
use sqlx::prelude::FromRow;
use twilight_model::id::{
    Id,
    marker::{GuildMarker, UserMarker},
};

use tulpje_framework::Error;
use uuid::Uuid;

use super::util::SystemRef;
use tulpje_lib::db::DbId;

#[derive(Debug)]
pub(crate) struct ModPkGuildRow {
    pub(crate) guild_id: DbId<GuildMarker>,
    pub(crate) user_id: DbId<UserMarker>,
    pub(crate) system_uuid: Uuid,
}
pub(crate) async fn save_guild_settings(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
    system_uuid: Uuid,
) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO pk_guilds (guild_id, user_id, system_uuid) VALUES ($1, $2, $3) ON CONFLICT (guild_id) DO UPDATE SET system_uuid = $3",
        i64::from(DbId(guild_id)),
        i64::from(DbId(user_id)),
        system_uuid,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(crate) async fn get_guild_settings_for_system(
    db: &sqlx::PgPool,
    system_uuid: Uuid,
) -> Result<Option<ModPkGuildRow>, Error> {
    Ok(sqlx::query_as!(
        ModPkGuildRow,
        "SELECT guild_id, user_id, system_uuid FROM pk_guilds WHERE system_uuid = $1",
        system_uuid
    )
    .fetch_optional(db)
    .await?)
}

pub(crate) async fn get_guild_settings_for_id(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
) -> Result<Option<ModPkGuildRow>, Error> {
    Ok(sqlx::query_as!(
        ModPkGuildRow,
        "SELECT guild_id, user_id, system_uuid FROM pk_guilds WHERE guild_id = $1",
        i64::from(DbId(guild_id))
    )
    .fetch_optional(db)
    .await?)
}

#[expect(dead_code, reason = "useful utility function")]
pub(crate) async fn get_guild_settings(db: &sqlx::PgPool) -> Result<Vec<ModPkGuildRow>, Error> {
    Ok(sqlx::query_as!(
        ModPkGuildRow,
        "SELECT guild_id, user_id, system_uuid FROM pk_guilds",
    )
    .fetch_all(db)
    .await?)
}

#[derive(Debug, FromRow)]
pub(crate) struct ModPkSystem {
    pub(crate) id: String,
    pub(crate) uuid: Uuid,
    pub(crate) name: Option<String>,
}

impl From<System> for ModPkSystem {
    fn from(value: System) -> Self {
        Self {
            id: value.id.0,
            uuid: value.uuid,
            name: value.name,
        }
    }
}

#[expect(dead_code, reason = "useful utility function")]
pub(crate) async fn get_all_systems(db: &sqlx::PgPool) -> Result<Vec<ModPkSystem>, Error> {
    Ok(
        sqlx::query_as!(ModPkSystem, "SELECT id, uuid, name FROM pk_systems",)
            .fetch_all(db)
            .await?,
    )
}
pub(crate) async fn get_systems(
    db: &sqlx::PgPool,
    uuids: Vec<Uuid>,
) -> Result<Vec<ModPkSystem>, Error> {
    Ok(sqlx::query_as!(
        ModPkSystem,
        "SELECT id, uuid, name FROM pk_systems WHERE uuid = ANY($1)",
        &uuids[..],
    )
    .fetch_all(db)
    .await?)
}

pub(crate) async fn get_system(
    db: &sqlx::PgPool,
    system_ref: &SystemRef,
) -> Result<Option<ModPkSystem>, Error> {
    match system_ref {
        SystemRef::Id(id) => Ok(sqlx::query_as!(
            ModPkSystem,
            "SELECT id, uuid, name FROM pk_systems WHERE id = $1",
            id
        )
        .fetch_optional(db)
        .await?),
        SystemRef::Uuid(uuid) => Ok(sqlx::query_as!(
            ModPkSystem,
            "SELECT id, uuid, name FROM pk_systems WHERE uuid = $1",
            uuid
        )
        .fetch_optional(db)
        .await?),
        SystemRef::DiscordId(_) => Err("Deleting by discord ID is unsupported".into()),
    }
}

pub(crate) async fn update_system(db: &sqlx::PgPool, system: &ModPkSystem) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO pk_systems (id, uuid, name) VALUES ($1, $2, $3) ON CONFLICT (uuid) DO UPDATE SET id = $1, name = $3",
        system.id,
        system.uuid,
        system.name,
    )
    .execute(db)
    .await?;

    Ok(())
}

#[expect(dead_code, reason = "useful utility function")]
pub(crate) async fn delete_system(db: &sqlx::PgPool, system_ref: SystemRef) -> Result<(), Error> {
    match system_ref {
        SystemRef::Uuid(uuid) => {
            sqlx::query!("DELETE FROM pk_systems WHERE uuid = $1", uuid,)
                .execute(db)
                .await?;
            Ok(())
        }
        SystemRef::Id(id) => {
            sqlx::query!("DELETE FROM pk_systems WHERE id = $1", id,)
                .execute(db)
                .await?;
            Ok(())
        }
        SystemRef::DiscordId(_) => Err("Deleting by discord ID is unsupported".into()),
    }
}

pub(crate) async fn cleanup_systems(db: &sqlx::PgPool) -> Result<u64, Error> {
    Ok(sqlx::query!(
        r#"
            DELETE
            FROM
                pk_systems
            WHERE
                NOT EXISTS (
                    SELECT 1 FROM pk_guilds WHERE pk_guilds.system_uuid = pk_systems.uuid
                )
            AND
                NOT EXISTS (
                    SELECT 1 FROM pk_notify_systems WHERE pk_notify_systems.system_uuid = pk_systems.uuid
                )
        "#
    )
    .execute(db)
    .await?
    .rows_affected())
}
