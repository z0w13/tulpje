use twilight_model::id::{Id, marker::GuildMarker};

use crate::db_id::DbId;
use tulpje_framework::Error;

/// track that we've seen the guild in the database
pub async fn touch_guild(db: &sqlx::PgPool, guild_id: Id<GuildMarker>) -> Result<(), Error> {
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
