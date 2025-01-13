use futures_util::{Stream, TryStreamExt as _};
use sqlx::types::chrono;

use tulpje_framework::Error;
use twilight_model::id::{
    marker::{EmojiMarker, GuildMarker},
    Id,
};

use super::shared::StatsSort;
use crate::db::DbId;

#[derive(Debug)]
// TODO: tests to confirm this still matches the database structure
#[expect(dead_code, reason = "reflects database structure")]
pub(crate) struct EmojiUse {
    pub(crate) id: i64,
    pub(crate) guild_id: DbId<GuildMarker>,
    pub(crate) emoji_id: DbId<EmojiMarker>,
    pub(crate) name: String,
    pub(crate) animated: bool,
    pub(crate) created_at: chrono::NaiveDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct EmojiStats {
    #[sqlx(flatten)]
    pub(crate) emoji: Emoji,
    pub(crate) times_used: i64,
    pub(crate) last_used_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub(crate) struct Emoji {
    #[sqlx(rename = "emoji_id")]
    pub(crate) id: DbId<EmojiMarker>,
    pub(crate) guild_id: DbId<GuildMarker>,
    pub(crate) name: String,
    pub(crate) animated: bool,
}

impl Emoji {
    pub(crate) fn new(
        id: Id<EmojiMarker>,
        guild_id: Id<GuildMarker>,
        name: String,
        animated: bool,
    ) -> Self {
        Self {
            id: DbId(id),
            guild_id: DbId(guild_id),
            name,
            animated,
        }
    }

    pub(crate) fn from_twilight(
        val: twilight_model::guild::Emoji,
        guild_id: Id<GuildMarker>,
    ) -> Self {
        Self::new(val.id, guild_id, val.name, val.animated)
    }
}

impl std::fmt::Display for Emoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}:{}:{}>",
            if self.animated { "a" } else { "" },
            self.name,
            self.id
        )
    }
}

impl std::hash::Hash for Emoji {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Emoji {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Emoji {}

pub(crate) async fn save_emoji_use(
    db: &sqlx::PgPool,
    emote: &Emoji,
    timestamp: chrono::DateTime<chrono::Utc>,
) -> Result<(), Error> {
    sqlx::query!(
        "
            INSERT INTO emoji_uses (
                guild_id,
                emoji_id,
                name,
                animated,
                created_at
            ) VALUES ($1, $2, $3, $4, $5)
        ",
        i64::from(emote.guild_id),
        i64::from(emote.id),
        emote.name,
        emote.animated,
        timestamp.naive_utc(),
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(crate) async fn get_emoji_stat_count(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
) -> Result<i64, Error> {
    Ok(sqlx::query_scalar!(
        "SELECT COUNT(DISTINCT emoji_id) FROM emoji_uses WHERE guild_id = $1",
        i64::from(DbId(guild_id)),
    )
    .fetch_one(db)
    .await?
    .unwrap_or(0))
}

pub(crate) async fn get_emoji_stats(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    sort: &StatsSort,
    offset: u16,
    limit: u16,
) -> Result<Vec<EmojiStats>, Error> {
    let order_by_clause = match sort {
        StatsSort::CountDesc => "times_used DESC",
        StatsSort::CountAsc => "times_used ASC",
        StatsSort::DateDesc => "last_used_at DESC",
        StatsSort::DateAsc => "last_used_at ASC",
    };

    // NOTE: Wish we could use query_as! but we're using a dynamic SORT BY clause
    let result: Vec<EmojiStats> = sqlx::query_as(&format!(
        "
            SELECT
                emoji_id, MAX(name) as name,
                $1 AS guild_id,
                ANY_VALUE(animated) as animated,
                COUNT(emoji_id) AS times_used,
                MAX(created_at) AS last_used_at
            FROM emoji_uses
            WHERE guild_id = $1
            GROUP BY emoji_id
            ORDER BY {}
            OFFSET $2
            LIMIT $3
        ",
        order_by_clause
    ))
    .bind(DbId(guild_id))
    .bind(i32::from(offset))
    .bind(i32::from(limit))
    .fetch_all(db)
    .await?;

    Ok(result)
}

pub(crate) fn get_tracked_guilds(
    db: &sqlx::PgPool,
) -> impl Stream<Item = Result<DbId<GuildMarker>, sqlx::Error>> + use<'_> {
    sqlx::query_scalar!("SELECT DISTINCT guild_id FROM emoji_uses")
        .fetch(db)
        .map_ok(DbId::from)
}

pub(crate) async fn delete_emojis_not_in_list_for_guild(
    db: &sqlx::PgPool,
    guild_id: Id<GuildMarker>,
    emojis: Vec<Id<EmojiMarker>>,
) -> Result<u64, Error> {
    // convert ids to i64
    let emoji_ids: Vec<i64> = emojis.into_iter().map(DbId).map(i64::from).collect();

    let result = sqlx::query("DELETE FROM emoji_uses WHERE guild_id = $1 AND emoji_id != ALL($2)")
        .bind(i64::from(DbId(guild_id)))
        .bind(emoji_ids)
        .execute(db)
        .await?;

    Ok(result.rows_affected())
}
