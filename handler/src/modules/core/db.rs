use std::collections::HashMap;

use twilight_model::id::{marker::GuildMarker, Id};

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
