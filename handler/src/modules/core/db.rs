use std::collections::HashMap;

use chrono::NaiveDateTime;
use twilight_model::id::{Id, marker::GuildMarker};

use tulpje_framework::Error;

use crate::db::DbId;

pub(super) async fn enable_module(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    module: &String,
) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO guild_modules (guild_id, module) VALUES ($1, $2) ON CONFLICT (guild_id) DO NOTHING",
        i64::from(DbId(guild_id)),
        module,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(super) async fn disable_module(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    module: &String,
) -> Result<(), Error> {
    sqlx::query!(
        "DELETE FROM guild_modules WHERE guild_id = $1 AND module = $2",
        i64::from(DbId(guild_id)),
        module,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(super) async fn guild_modules(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
) -> Result<Vec<String>, Error> {
    Ok(sqlx::query_scalar!(
        "SELECT module FROM guild_modules WHERE guild_id = $1",
        i64::from(DbId(guild_id))
    )
    .fetch_all(db)
    .await?)
}

#[expect(dead_code, reason = "useful utility function")]
pub(crate) async fn guilds_with_module(
    db: &sqlx::PgPool,
    module: &str,
) -> Result<Vec<Id<GuildMarker>>, Error> {
    Ok(sqlx::query_scalar!(
        "SELECT guild_id FROM guild_modules WHERE module = $1",
        module
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|id| *DbId::from(id))
    .collect())
}

#[expect(dead_code, reason = "useful utility function")]
pub(crate) async fn all_guild_modules(
    db: &sqlx::PgPool,
) -> Result<HashMap<Id<GuildMarker>, Vec<String>>, Error> {
    let rows = sqlx::query!("SELECT guild_id, module FROM guild_modules")
        .fetch_all(db)
        .await?;

    let mut result = HashMap::new();
    for row in rows {
        result
            .entry(*DbId::from(row.guild_id))
            .or_insert(Vec::new())
            .push(row.module);
    }

    Ok(result)
}

/// track that we've seen the guild in the database
pub(super) async fn touch_guild(db: &sqlx::PgPool, guild_id: Id<GuildMarker>) -> Result<(), Error> {
    sqlx::query!(
        r#"
            INSERT INTO
                guilds (guild_id, created_at, updated_at)
            VALUES
                ($1, NOW(), NOW())
            ON CONFLICT
                (guild_id)
            DO UPDATE
                SET updated_at = NOW()
        "#,
        i64::from(DbId(guild_id))
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(super) async fn leave_guild(db: &sqlx::PgPool, guild_id: Id<GuildMarker>) -> Result<(), Error> {
    sqlx::query!(
        r#"
            UPDATE
                guilds
            SET
                deleted_at = NOW()
            WHERE
                guild_id = $1
        "#,
        i64::from(DbId(guild_id))
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(super) async fn _delete_guild(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
) -> Result<bool, Error> {
    Ok(sqlx::query_scalar!(
        r#"
            DELETE FROM
                guilds
            WHERE
                guild_id = $1
        "#,
        i64::from(DbId(guild_id))
    )
    .execute(db)
    .await?
    .rows_affected()
        > 0)
}

pub(super) async fn guilds_eligible_for_deletion(
    db: &sqlx::PgPool,
) -> Result<Vec<(Id<GuildMarker>, NaiveDateTime)>, Error> {
    Ok(sqlx::query!(
        r#"
            SELECT
                guild_id, deleted_at AS "deleted_at!"
            FROM
                guilds
            WHERE
                deleted_at IS NOT NULL
            AND
                deleted_at < NOW() - interval '7 days'
        "#
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|r| (*DbId::from(r.guild_id), r.deleted_at))
    .collect())
}
