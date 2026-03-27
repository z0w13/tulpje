use std::collections::HashSet;

use sqlx::prelude::FromRow;
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, GuildMarker},
};

use tulpje_framework::Error;
use uuid::Uuid;

use crate::{db::DbId, modules::pk::db::ModPkSystem};

#[expect(dead_code, reason = "reflects db structure, keep intact")]
pub(crate) struct ModPkFrontersRow {
    pub(crate) guild_id: DbId<GuildMarker>,
    pub(crate) category_id: DbId<ChannelMarker>,
}

#[expect(dead_code, reason = "useful utility function")]
pub(crate) async fn get_fronter_categories(
    db: &sqlx::PgPool,
) -> Result<Vec<ModPkFrontersRow>, Error> {
    Ok(sqlx::query_as!(
        ModPkFrontersRow,
        "SELECT guild_id, category_id FROM pk_fronters"
    )
    .fetch_all(db)
    .await?)
}

pub(crate) async fn get_fronter_category(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
) -> Result<Option<DbId<ChannelMarker>>, Error> {
    let result = sqlx::query_scalar!(
        "SELECT category_id FROM pk_fronters WHERE guild_id = $1",
        i64::from(DbId(guild_id)),
    )
    .fetch_optional(db)
    .await?;

    Ok(result.map(Into::into))
}

pub(crate) async fn save_fronter_category(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    channel_id: Id<ChannelMarker>,
) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO pk_fronters (guild_id, category_id) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET category_id = $2",
        i64::from(DbId(guild_id)),
        i64::from(DbId(channel_id)),
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(crate) async fn get_tracked_system_count(db: &sqlx::PgPool) -> Result<usize, Error> {
    Ok(sqlx::query_scalar!(
        r#"
        SELECT
            COUNT(uuid)
        FROM
            pk_systems
        WHERE
            uuid IN (
                SELECT
                    system_uuid
                FROM
                    pk_notify_systems
            )
        OR
            id IN (
                SELECT
                    system_id
                FROM
                    pk_guilds
                INNER JOIN
                    pk_fronters
                ON
                    pk_guilds.guild_id = pk_fronters.guild_id
            )
        "#
    )
    .fetch_one(db)
    .await?
    .unwrap_or_default() as usize)
}

pub(crate) async fn get_system_count(db: &sqlx::PgPool) -> Result<usize, Error> {
    Ok(sqlx::query_scalar!(
        r#"
        SELECT COUNT(uuid) FROM pk_systems;
    "#
    )
    .fetch_one(db)
    .await?
    .unwrap_or_default() as usize)
}

#[derive(Debug, FromRow)]
#[expect(dead_code, reason = "reflects db structure, keep intact")]
pub(crate) struct ModPkSystemFronters {
    pub(crate) system_uuid: Uuid,
    pub(crate) fronters: sqlx::types::Json<Vec<Uuid>>,
    pub(crate) updated_at: chrono::NaiveDateTime,
}

pub(crate) async fn did_fronters_change(
    db: &sqlx::PgPool,
    system_uuid: Uuid,
    new_fronters: &[Uuid],
) -> Result<bool, Error> {
    let Some(saved_front) = get_fronters(db, system_uuid).await? else {
        return Ok(true);
    };

    let saved_fronters: HashSet<&Uuid> = saved_front.fronters.iter().collect();
    let new_fronters: HashSet<&Uuid> = new_fronters.iter().collect();

    Ok(saved_fronters != new_fronters)
}

pub(crate) async fn get_fronters(
    db: &sqlx::PgPool,
    system_uuid: Uuid,
) -> Result<Option<ModPkSystemFronters>, Error> {
    Ok(sqlx::query_as!(
        ModPkSystemFronters,
        r#"SELECT system_uuid, fronters as "fronters: sqlx::types::Json<Vec<Uuid>>", updated_at FROM pk_system_fronters WHERE system_uuid = $1"#,
        system_uuid
    )
    .fetch_optional(db)
    .await?)
}

pub(crate) async fn update_fronters_timestamp(
    db: &sqlx::PgPool,
    system_uuid: Uuid,
) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO pk_system_fronters (system_uuid, fronters, updated_at) VALUES ($1, '[]', NOW()) ON CONFLICT (system_uuid) DO UPDATE SET updated_at = NOW()",
        system_uuid,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(crate) async fn update_fronters(
    db: &sqlx::PgPool,
    system_uuid: Uuid,
    fronters: &[Uuid],
) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO pk_system_fronters (system_uuid, fronters, updated_at) VALUES ($1, $2, $3) ON CONFLICT (system_uuid) DO UPDATE SET fronters = $2, updated_at = $3",
        system_uuid,
        sqlx::types::Json(fronters) as _,
        chrono::Utc::now().naive_utc(),
    )
    .execute(db)
    .await?;

    Ok(())
}

#[expect(dead_code, reason = "useful utility function")]
pub(crate) async fn delete_fronters(db: &sqlx::PgPool, system_uuid: Uuid) -> Result<(), Error> {
    sqlx::query!(
        "DELETE FROM pk_system_fronters WHERE system_uuid = $1",
        system_uuid
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(crate) async fn get_systems_to_update(db: &sqlx::PgPool) -> Result<Vec<ModPkSystem>, Error> {
    // fetch 10 systems that
    //   * haven't had fronters updated for over 10 minutes,
    //   * or that are missing in pk_system_fronters
    // running this once a minute should get us a limit of 100 systems, which should be sufficient
    // for now
    //
    // NOTE: We don't need to filter on whether a system is in pk_notify_systems or pk_fronters
    //       because the pk_systems table should get cleaned up by the calls to cleanup_system
    //       automatically
    // TODO: Write a query that shows us any "orphan" systems
    Ok(sqlx::query_as!(
        ModPkSystem,
        r#"
            SELECT
                uuid, id, name
            FROM
                pk_systems
            LEFT JOIN
                pk_system_fronters
            ON pk_systems.uuid = pk_system_fronters.system_uuid
            WHERE (
                    updated_at IS NULL
                OR
                    updated_at <= NOW() - interval '10 minutes'
            ) AND (
                    uuid IN (
                        SELECT
                            system_uuid
                        FROM
                            pk_notify_systems
                    )
                OR
                    id IN (
                        SELECT
                            system_id
                        FROM
                            pk_guilds
                        INNER JOIN
                            pk_fronters
                        ON
                            pk_guilds.guild_id = pk_fronters.guild_id
                    )
            )
            ORDER BY
                updated_at
            ASC NULLS FIRST
            LIMIT 10
        "#
    )
    .fetch_all(db)
    .await?)
}
