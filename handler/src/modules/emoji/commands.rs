use std::cmp::{max, min};

use twilight_model::{
    application::interaction::application_command::CommandOptionValue,
    channel::message::{
        Component, Embed, EmojiReactionType,
        component::{ActionRow, Button, ButtonStyle, SelectMenu, SelectMenuType},
    },
    gateway::payload::incoming::InteractionCreate,
    guild::Guild,
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{
        Id,
        marker::{EmojiMarker, GuildMarker},
    },
};
use twilight_util::builder::{
    InteractionResponseDataBuilder,
    embed::{EmbedBuilder, EmbedFooterBuilder},
};

use tulpje_framework::Error;

use super::db;
use crate::{
    context::{CommandContext, ComponentInteractionContext},
    modules::emoji::shared::StatsSort,
};

const EMOJIS_PER_PAGE: u16 = 15;

fn create_emoji_stats_sort_menu() -> SelectMenu {
    SelectMenu {
        id: None,
        required: None, // NOTE: only used in modals
        custom_id: "emoji_stats_sort".into(),
        kind: SelectMenuType::Text,
        options: Some(vec![
            StatsSort::CountDesc.into(),
            StatsSort::CountAsc.into(),
            StatsSort::DateDesc.into(),
            StatsSort::DateAsc.into(),
        ]),
        placeholder: Some("Sort".into()),

        // defaults
        disabled: false,
        max_values: None,
        min_values: None,
        default_values: None,
        channel_types: None,
    }
}

fn create_emoji_stats_pagination_buttons(current_page: u16, total_pages: u16) -> Vec<Component> {
    vec![
        Component::Button(Button {
            id: None,
            custom_id: Some(String::from("emoji_stats_first_page")),
            disabled: current_page == 1,
            style: ButtonStyle::Primary,
            emoji: Some(EmojiReactionType::Unicode {
                name: String::from("⏮️"),
            }),

            label: None,
            url: None,
            sku_id: None,
        }),
        Component::Button(Button {
            id: None,
            custom_id: Some(String::from("emoji_stats_prev_page")),
            disabled: current_page == 1,
            style: ButtonStyle::Primary,
            emoji: Some(EmojiReactionType::Unicode {
                name: String::from("◀️"),
            }),

            label: None,
            url: None,
            sku_id: None,
        }),
        Component::Button(Button {
            id: None,
            custom_id: Some(String::from("emoji_stats_next_page")),
            disabled: current_page == total_pages,
            style: ButtonStyle::Primary,
            emoji: Some(EmojiReactionType::Unicode {
                name: String::from("▶️"),
            }),

            label: None,
            url: None,
            sku_id: None,
        }),
        Component::Button(Button {
            id: None,
            custom_id: Some(String::from("emoji_stats_last_page")),
            disabled: current_page == total_pages,
            style: ButtonStyle::Primary,
            emoji: Some(EmojiReactionType::Unicode {
                name: String::from("⏭️"),
            }),

            label: None,
            url: None,
            sku_id: None,
        }),
    ]
}

async fn create_emoji_stats_embed(
    db: &sqlx::PgPool,
    guild: &Guild,
    sort: &StatsSort,
    current_page: u16,
    total_pages: u16,
) -> Result<Embed, Error> {
    let emoji_stats = db::get_emoji_stats(
        db,
        guild.id,
        sort,
        (current_page - 1) * EMOJIS_PER_PAGE,
        EMOJIS_PER_PAGE,
    )
    .await?;
    let emoji_str = if !emoji_stats.is_empty() {
        emoji_stats
            .into_iter()
            .map(|emoji_stats| {
                format!(
                    "{} • Used {} times • Last used <t:{}:R>",
                    emoji_stats.emoji,
                    emoji_stats.times_used,
                    emoji_stats.last_used_at.and_utc().timestamp(),
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        "No Data".to_string()
    };

    let mut builder = EmbedBuilder::new()
        .title(format!("{} Emotes in {}", sort.name(), guild.name))
        .description(emoji_str);

    if total_pages > 0 {
        builder = builder.footer(
            EmbedFooterBuilder::new(format!("Page {} of {}", current_page, total_pages)).build(),
        );
    }

    Ok(builder.validate()?.build())
}

fn extract_page_and_sort(event: &InteractionCreate) -> Option<(u16, StatsSort)> {
    let Some(ref message) = event.message else {
        tracing::trace!("extract_page: message is None");
        return None;
    };

    let Some(embed) = message.embeds.first() else {
        tracing::trace!("extract_page: first embed is None");
        return None;
    };

    let Some(ref title) = embed.title else {
        tracing::trace!("extract_page: title is None");
        return None;
    };

    let Some(ref footer) = embed.footer else {
        tracing::trace!("extract_page: footer is None");
        return None;
    };

    let Some(page_string) = footer.text.split_whitespace().nth(1) else {
        tracing::trace!("extract_page: page_string is None, {}", footer.text);
        return None;
    };

    // extract the current sorting method
    let sort = if title.starts_with(StatsSort::CountDesc.name()) {
        StatsSort::CountDesc
    } else if title.starts_with(StatsSort::CountAsc.name()) {
        StatsSort::CountAsc
    } else if title.starts_with(StatsSort::DateDesc.name()) {
        StatsSort::DateDesc
    } else if title.starts_with(StatsSort::DateAsc.name()) {
        StatsSort::DateAsc
    } else {
        // fallback to CountDesc if we can't figure it out
        StatsSort::CountDesc
    };

    // parse the current page number
    match page_string.parse::<u16>() {
        Ok(page) => Some((page, sort)),
        Err(err) => {
            tracing::trace!(
                "extract_page: error parsing page_string '{}': {}",
                page_string,
                err,
            );
            None
        }
    }
}

pub async fn handle_emoji_pagination(ctx: ComponentInteractionContext) -> Result<(), Error> {
    let guild = ctx.guild().await?.ok_or("not in guild")?;

    ctx.response(InteractionResponse {
        kind: InteractionResponseType::DeferredUpdateMessage,
        data: None,
    })
    .await?;

    let Some((page, sort)) = extract_page_and_sort(&ctx.event) else {
        tracing::warn!("handle_emoji_pagination: couldn't parse page id from event");
        return Ok(());
    };

    let total_pages = get_total_pages(&ctx.services.db, guild.id).await?;

    let new_page = match ctx.interaction.custom_id.as_str() {
        "emoji_stats_first_page" => 1,
        "emoji_stats_prev_page" => max(page - 1, 1),
        "emoji_stats_next_page" => min(page + 1, total_pages),
        "emoji_stats_last_page" => total_pages,
        other => {
            return Err(format!(
                "unknown interaction id for handle_emoji_pagination: {}",
                other
            )
            .into());
        }
    };

    if let Err(err) = ctx
        .interaction()
        .update_response(&ctx.event.token)
        .embeds(Some(&[create_emoji_stats_embed(
            &ctx.services.db,
            &guild,
            &sort,
            new_page,
            total_pages,
        )
        .await?]))
        .components(Some(&get_components(new_page, total_pages)))
        .await
    {
        tracing::warn!(?err, "failed to update message");
    }

    Ok(())
}
pub async fn handle_emoji_stats_sort(ctx: ComponentInteractionContext) -> Result<(), Error> {
    if ctx.interaction.custom_id != "emoji_stats_sort" {
        tracing::debug!(
            "ignoring interaction with incorrect custom_id: {}",
            ctx.interaction.custom_id
        );
        return Ok(());
    }
    tracing::trace!(interaction = ?ctx.interaction);

    ctx.response(InteractionResponse {
        kind: InteractionResponseType::DeferredUpdateMessage,
        data: None,
    })
    .await?;

    let Some(sort_by) = ctx.interaction.values.first() else {
        return Err("couldn't get selected value".into());
    };
    tracing::trace!(?sort_by);

    let sort = StatsSort::try_from_string(sort_by)?;
    tracing::trace!(sort = ?sort);

    let guild = ctx.guild().await?.ok_or("outside of guild")?;
    let total_pages = get_total_pages(&ctx.services.db, guild.id).await?;

    if let Err(err) = ctx
        .interaction()
        .update_response(&ctx.event.token)
        .embeds(Some(&[create_emoji_stats_embed(
            &ctx.services.db,
            &guild,
            &sort,
            1, // we reset back to first page after resetting sorting method
            total_pages,
        )
        .await?]))
        .components(Some(&get_components(1, total_pages)))
        .await
    {
        tracing::warn!(?err, "failed to update message");
    }

    Ok(())
}

pub async fn cmd_emoji_stats(ctx: CommandContext) -> Result<(), Error> {
    tracing::info!(command_info = ?ctx.command.options);

    let sort = if let Some(option) = ctx.command.options.first() {
        if let CommandOptionValue::String(str) = &option.value {
            StatsSort::try_from_string(str)?
        } else {
            StatsSort::CountDesc
        }
    } else {
        StatsSort::CountDesc
    };

    let guild = ctx.guild().await?.ok_or("not in guild")?;
    let total_pages = get_total_pages(&ctx.services.db, guild.id).await?;

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(
            InteractionResponseDataBuilder::new()
                .embeds([
                    create_emoji_stats_embed(&ctx.services.db, &guild, &sort, 1, total_pages)
                        .await?,
                ])
                .components(get_components(1, total_pages))
                .build(),
        ),
    };

    ctx.response(response).await?;

    Ok(())
}

pub async fn cmd_emoji_maintenance(ctx: CommandContext) -> Result<(), Error> {
    let guild = ctx.guild().await?.ok_or("not in guild")?;
    ctx.defer().await?;

    let emoji_ids: Vec<Id<EmojiMarker>> = ctx
        .client()
        .emojis(guild.id)
        .await?
        .models()
        .await?
        .into_iter()
        .map(|emoji| emoji.id)
        .collect();

    let count =
        db::delete_emojis_not_in_list_for_guild(&ctx.services.db, guild.id, emoji_ids).await?;

    ctx.update(format!("cleaned up {} deleted emotes", count))
        .await?;

    Ok(())
}

fn get_components(current_page: u16, total_pages: u16) -> Vec<Component> {
    if total_pages == 0 {
        return vec![];
    }

    vec![
        ActionRow {
            id: None,
            components: vec![create_emoji_stats_sort_menu().into()],
        }
        .into(),
        ActionRow {
            id: None,
            components: create_emoji_stats_pagination_buttons(current_page, total_pages),
        }
        .into(),
    ]
}

async fn get_total_pages(db: &sqlx::PgPool, guild_id: Id<GuildMarker>) -> Result<u16, Error> {
    #[expect(
        clippy::cast_precision_loss,
        reason = "page counts won't ever get big enough to cause data loss"
    )]
    Ok(
        (db::get_emoji_stat_count(db, guild_id).await? as f64 / f64::from(EMOJIS_PER_PAGE)).ceil()
            as u16,
    )
}
