use chrono::Utc;
use num_format::{Locale, ToFormattedString as _};

use tulpje_framework::Error;
use tulpje_shared::{metrics::Metrics, shard_state::ShardState, version};
use twilight_model::{
    http::interaction::{InteractionResponse, InteractionResponseType},
    util::Timestamp,
};
use twilight_util::builder::{
    embed::{EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder},
    InteractionResponseDataBuilder,
};

use super::redis;
use crate::context::CommandContext;

async fn minimal_stats_response(ctx: CommandContext, api_latency: i64) -> Result<(), Error> {
    if let Err(err) = ctx
        .interaction()
        .update_response(&ctx.event.token)
        .content(Some(
            &format!(
                "
                    Stats unavailable (either the bot just restarted or something is wrong)

                    **Basic Info:**
                    Tulpje {}
                    Current Shard: #{}
                    API Latency: {} ms
                ",
                version!(),
                ctx.meta.shard,
                api_latency.to_formatted_string(&Locale::en),
            )
            // remove leading whitespace
            .lines()
            .map(str::trim_start)
            .collect::<Vec<&str>>()
            .join("\n"),
        ))
        .await
    {
        tracing::warn!(?err, "failed to respond to command");
    }

    Ok(())
}

pub async fn stats(ctx: CommandContext) -> Result<(), Error> {
    let time_before = chrono::Utc::now().timestamp_millis();
    ctx.reply("...").await?;
    let time_after = chrono::Utc::now().timestamp_millis();
    let api_latency = time_after - time_before;

    let shard_stats = match redis::get_all_shard_stats(ctx.services.redis.clone()).await {
        Ok(stats) => stats,
        Err(err) => {
            tracing::warn!("error getting shard stats: {}", err);
            return minimal_stats_response(ctx, api_latency).await;
        }
    };

    let total_shards = shard_stats.len();
    let Some(current_shard_state) = shard_stats.get(&ctx.meta.shard) else {
        tracing::warn!(shard = ctx.meta.shard, "couldn't find current shard state",);
        return minimal_stats_response(ctx, api_latency).await;
    };

    // TODO: Handle dead shards somehow, they don't get cleaned up automatically
    let shards_up = shard_stats.iter().filter(|(_, s)| s.is_up()).count();
    let guild_count: u64 = shard_stats.values().map(|s| s.guild_count).sum();

    let handler_stats = redis::get_process_stats(
        &ctx.services.redis,
        &format!("handler-{}", ctx.services.handler_id),
    )
    .await?;
    let gateway_stats =
        redis::get_process_stats(&ctx.services.redis, &format!("gateway-{}", ctx.meta.shard))
            .await?;

    #[expect(
        clippy::cast_precision_loss,
        reason = "`Metrics::memory_usage` overflows at 9PiB, we should never hit that"
    )]
    let (cpu_usage_str, mem_usage_str) = if let (Some(handler_stats), Some(gateway_stats)) =
        (handler_stats, gateway_stats)
    {
        (
            format!(
                "{:.2} %",
                (handler_stats.cpu_usage + gateway_stats.cpu_usage)
            ),
            format!(
                "{:.2} MiB",
                (handler_stats.memory_usage + gateway_stats.memory_usage) as f64 / 1024. / 1024.
            ),
        )
    } else {
        (String::from("N/A"), String::from("N/A"))
    };

    let embed = EmbedBuilder::new()
        .title("Tulpje Discord Bot")
        .url("https://github.com/z0w13/tulpje")
        .field(EmbedFieldBuilder::new("Version", version!()).inline())
        .field(
            EmbedFieldBuilder::new("Servers", guild_count.to_formatted_string(&Locale::en))
                .inline(),
        )
        .field(
            EmbedFieldBuilder::new(
                "Current Shard",
                format!(
                    "Shard #{} (of {} total, {} are up)",
                    ctx.meta.shard, total_shards, shards_up,
                ),
            )
            .inline(),
        )
        .field(
            EmbedFieldBuilder::new(
                "Shard Uptime",
                format!(
                    "{} ({} disconnections)",
                    tulpje_shared::format_significant_duration(
                        chrono::DateTime::from_timestamp(
                            current_shard_state.last_connection.try_into()?,
                            0
                        )
                        .ok_or("couldn't create timestamp")?
                        .signed_duration_since(Utc::now())
                        .num_seconds()
                        .unsigned_abs()
                    ),
                    current_shard_state.disconnect_count
                ),
            )
            .inline(),
        )
        .field(
            EmbedFieldBuilder::new(
                "Latency",
                format!(
                    "API: {} ms, Shard: {}",
                    api_latency,
                    match current_shard_state.latency {
                        0 => "N/A".into(),
                        ms => format!("{} ms ", ms.to_formatted_string(&Locale::en)),
                    }
                ),
            )
            .inline(),
        )
        .field(EmbedFieldBuilder::new("CPU Usage", cpu_usage_str).inline())
        .field(EmbedFieldBuilder::new("Memory Usage", mem_usage_str).inline())
        .footer(EmbedFooterBuilder::new(
            "Tulpje • https://github.com/z0w13/tulpje • Last Restarted:",
        ))
        .timestamp(
            Timestamp::from_secs(
                current_shard_state
                    .last_started
                    .try_into()
                    .expect("couldn't parse timestamp into i64"),
            )
            .expect("couldn't parse unix timestamp somehow"),
        )
        .build();

    if let Err(err) = ctx
        .interaction()
        .update_response(&ctx.event.token)
        .content(None)
        .embeds(Some(&[embed]))
        .await
    {
        tracing::warn!(?err, "failed to respond to command");
    }

    Ok(())
}

pub async fn shards(ctx: CommandContext) -> Result<(), Error> {
    let mut shard_stats = redis::get_all_shard_stats(ctx.services.redis.clone())
        .await?
        .into_values()
        .collect::<Vec<ShardState>>();
    shard_stats.sort_by_key(|s| s.shard_id);

    let mut embed = EmbedBuilder::new().title("Tulpje Discord Bot").build();
    if !shard_stats.is_empty() {
        for shard in shard_stats {
            embed.fields.push(
                EmbedFieldBuilder::new(
                    format!("Shard #{}", shard.shard_id),
                    if shard.is_up() {
                        format!(
                            "Latency: {} ms / Uptime: {} / Servers: {} / Disconnects: {}",
                            shard.latency.to_formatted_string(&Locale::en),
                            tulpje_shared::format_significant_duration(
                                chrono::DateTime::from_timestamp(
                                    shard.last_connection.try_into()?,
                                    0
                                )
                                .ok_or("couldn't create timestamp")?
                                .signed_duration_since(Utc::now())
                                .num_seconds()
                                .unsigned_abs()
                            ),
                            shard.guild_count.to_formatted_string(&Locale::en),
                            shard.disconnect_count.to_formatted_string(&Locale::en),
                        )
                    } else {
                        "Down".into()
                    },
                )
                .into(),
            );
        }
    } else {
        embed.description = Some(String::from("No data available"));
    }

    let response = InteractionResponseDataBuilder::new()
        .embeds([embed])
        .build();

    if let Err(err) = ctx
        .response(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(response),
        })
        .await
    {
        tracing::warn!(?err, "failed to respond to command");
    }

    Ok(())
}

#[expect(
    clippy::cast_precision_loss,
    reason = "using 8PiB of RAM is probably a bigger issue than `process.memory_usage as f64`"
)]
pub async fn processes(ctx: CommandContext) -> Result<(), Error> {
    let mut process_stats = redis::get_all_process_stats(ctx.services.redis.clone())
        .await?
        .into_values()
        .collect::<Vec<Metrics>>();
    process_stats.sort_by_key(|m| m.name.clone());

    let mut embed = EmbedBuilder::new().title("Tulpje Discord Bot").build();

    if !process_stats.is_empty() {
        for process in process_stats {
            embed.fields.push(
                EmbedFieldBuilder::new(
                    process.name,
                    format!(
                        "CPU: {:.2}% / Mem: {:.2}MiB / Uptime: {} / Version: {}",
                        process.cpu_usage,
                        process.memory_usage as f64 / 1024. / 1024.,
                        tulpje_shared::format_significant_duration(
                            chrono::DateTime::from_timestamp(process.last_started.try_into()?, 0)
                                .ok_or("couldn't create timestamp")?
                                .signed_duration_since(Utc::now())
                                .num_seconds()
                                .unsigned_abs()
                        ),
                        process.version,
                    ),
                )
                .into(),
            );
        }
    } else {
        embed.description = Some(String::from("No data available"));
    }

    let response = InteractionResponseDataBuilder::new()
        .embeds([embed])
        .build();

    if let Err(err) = ctx
        .response(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(response),
        })
        .await
    {
        tracing::warn!(?err, "failed to respond to command");
    }

    Ok(())
}
