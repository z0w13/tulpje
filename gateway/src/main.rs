use std::{env, time::Duration};

use futures_util::StreamExt;
use redis::aio::ConnectionManagerConfig;
use tokio::signal::unix::SignalKind;
use tokio_util::sync::CancellationToken;
use twilight_gateway::CloseFrame;
use twilight_model::gateway::{
    payload::outgoing::{identify::IdentifyProperties, update_presence::UpdatePresencePayload},
    presence::{Activity, MinimalActivity, Status},
};

use reconnecting_amqp::{AmqpHandle, ConnectionArguments};
use tulpje_shared::{DiscordEvent, version};

mod config;
mod metrics;
mod parsed_event;
mod shard_reporter;

use config::Config;
use parsed_event::ParsedEvent;
use shard_reporter::{ReporterEvent, ShardReporterHandle};

#[tokio::main]
async fn main() {
    // set-up logging
    tracing_subscriber::fmt::init();
    tracing::info!("starting tulpje-gateway {} ...", version!());

    // register signal handlers
    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate())
        .expect("error registering SIGTERM handler");
    let mut sigint = tokio::signal::unix::signal(SignalKind::interrupt())
        .expect("error registering SIGINT (Ctrl+C) handler");

    // configure tls
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("error setting tls provider");

    // create config from environment vars
    let config = Config::load().expect("error loading config from env");

    // create AMQP connection
    let mut amqp = AmqpHandle::try_from_str(
        &config.rabbitmq_address,
        ConnectionArguments::new("discord"),
        None,
    )
    .expect("couldn't create amqp client");
    amqp.wait_start().await.expect("couldn't connect to amqp");
    let amqp_tx = amqp.sender();

    // create the redis connection
    let redis_client = redis::Client::open(config.redis_url).expect("error initialising redis");
    let redis = redis_client
        .get_connection_manager_with_config(
            ConnectionManagerConfig::new()
                .set_connection_timeout(Some(Duration::from_secs(5)))
                .set_response_timeout(Some(Duration::from_secs(5))),
        )
        .await
        .expect("error creating connection manager");

    // set-up metrics
    tracing::info!("installing metrics collector and exporter...");
    metrics::install(config.metrics_listen_addr, redis.clone(), config.shard_id)
        .expect("error setting up metrics");

    // create the shard
    tracing::info!("shard: {}, total: {}", config.shard_id, config.shard_count);
    let shard_config = twilight_gateway::ConfigBuilder::new(
        config.discord_token,
        twilight_gateway::Intents::all(),
    )
    .presence(create_presence())
    .identify_properties(IdentifyProperties {
        browser: "tulpje".into(),
        device: "tulpje".into(),
        os: std::env::consts::OS.into(),
    })
    .build();
    let shard_id = twilight_gateway::ShardId::new_checked(config.shard_id, config.shard_count)
        .expect("error constructing shard ID");
    let mut shard = twilight_gateway::Shard::with_config(shard_id, shard_config);

    // create shard reporter
    let (shard_reporter_join, mut shard_reporter) =
        ShardReporterHandle::new(redis.clone(), shard_id.number());

    // initialisation done, ratelimit on session_limit
    tracing::info!("waiting for gateway queue...");
    reqwest::get(config.discord_gateway_queue)
        .await
        .expect("error waiting for gateway queue");

    // start main loop
    tracing::info!("starting main loop...");

    let shutdown_token = CancellationToken::new();

    let shard_reporter_inner = shard_reporter.clone();
    let shutdown_token_inner = shutdown_token.clone();
    let main_handle = tokio::spawn(async move {
        let mut shutting_down = false;
        loop {
            tokio::select! {
                msg = shard.next() => {
                    match msg {
                        Some(Ok(message)) => {
                            let event = match ParsedEvent::from_message(message) {
                                Ok(evt) => evt,
                                Err(err) => {
                                    tracing::warn!("error parsing gateway message: {err}");
                                    continue;
                                }
                            };

                            // if this is a close frame and we're shutting, we break at the
                            // end of the loop, checking it here to avoid having to clone `event`
                            let should_stop = event.is_close() && shutting_down;

                            // track event metrics
                            metrics::track_gateway_event(
                                shard_id.number(),
                                event.name.as_deref().unwrap_or("default"),
                            );

                            if let Some(event) = event.event
                                && let Err(err) = shard_reporter_inner
                                    .try_send(ReporterEvent::from_event(event, shard.latency()))
                            {
                                tracing::error!("error sending message to ShardManager: {err}");
                            }

                            if let Some(text) = event.text
                                && event.forward
                            {
                                let event = DiscordEvent::new(shard_id.number(), text);
                                let serialized_event = match serde_json::to_vec(&event) {
                                    Ok(val) => val,
                                    Err(err) => {
                                        tracing::error!("error serializing event: {}", err);
                                        continue;
                                    }
                                };

                                if let Err(err) = amqp_tx.send(serialized_event) {
                                    tracing::error!("error sending event to amqp: {}", err);
                                    continue;
                                }

                                tracing::debug!(
                                    uuid = ?event.meta.uuid,
                                    shard = event.meta.shard,
                                    "event sent"
                                );
                            }

                            if should_stop {
                                break
                            }
                        }
                        Some(Err(err)) => {
                            tracing::error!(?err, "error receiving discord message");
                        }
                        None => {
                            tracing::error!("empty message, connection irrecoverably closed, exiting...");
                            break;
                        }
                    }
                },
                () = shutdown_token_inner.cancelled(), if !shutting_down => {
                    tracing::info!("disconnecting from Discord...");
                    shard.close(CloseFrame::RESUME);
                    shutting_down = true;
                },
            }
        }
    });

    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => {},
            _ = sigterm.recv() => {},
        }

        tracing::info!("shutting down...");
        shutdown_token.cancel();
    });

    tracing::trace!("running main loop ...");
    if let Err(err) = main_handle.await {
        tracing::error!("error joining main_handle: {err}");
    }

    shard_reporter.shutdown();
    tracing::trace!("waiting for shard reporter to exit...");
    if let Err(err) = shard_reporter_join.await {
        tracing::error!("error joining shard reporter: {err}");
    }

    amqp.shutdown();
    tracing::trace!("waiting for amqp to exit...");
    if let Err(err) = amqp.join().await {
        tracing::error!("error joining amqp handle: {err}");
    }

    tracing::info!("cleanup finished, existing...")
}

fn create_presence() -> UpdatePresencePayload {
    let state = format!(" Version: {}", version!());

    let mut activity: Activity = MinimalActivity {
        kind: twilight_model::gateway::presence::ActivityType::Custom,
        name: "~".into(),
        url: None,
    }
    .into();
    activity.state = Some(state);

    UpdatePresencePayload::new(vec![activity], false, None, Status::Online)
        .expect("couldn't create UpdatePresence struct")
}
