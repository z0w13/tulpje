mod config;
mod metrics;
mod modules;

use std::{sync::Arc, time::Duration};

use pkrs_fork::client::PkClient;
use redis::aio::ConnectionManagerConfig;
use sqlx::{
    ConnectOptions as _,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use tokio::{signal::unix::SignalKind, sync::mpsc};
use tracing::{Instrument as _, Span, log::LevelFilter};
use tulpje_lib::context;
use twilight_gateway::Event;

use reconnecting_amqp::{AmqpHandle, ConnectionArguments};
use tulpje_cache::{Cache, Config as CacheConfig, ResourceType};
use tulpje_common::{DiscordEvent, version};
use tulpje_framework::{Framework, Metadata, Registry, framework::Sender};

use config::Config;

#[tokio::main]
async fn main() {
    // set-up logging
    tulpje_common::logging::init();
    tracing::info!("starting tulpje-handler {} ...", version!());

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
    let config = Config::load().expect("error loading config");

    // needed for fetching recommended shard count
    let client = Arc::new(
        twilight_http::Client::builder()
            .proxy(config.discord_proxy, true)
            .token(config.discord_token)
            .ratelimiter(None)
            .build(),
    );

    // Get and store application id
    let app_id = client
        .current_user_application()
        .await
        .expect("error fetching application")
        .model()
        .await
        .expect("eror decoding application")
        .id;

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
    metrics::install(config.metrics_listen_addr, redis.clone(), config.handler_id)
        .expect("error setting up metrics");

    // set-up cache
    let cache = Arc::new(Cache::new(
        redis.clone(),
        CacheConfig::new().resource_types(
            ResourceType::empty()
                | ResourceType::CHANNEL
                | ResourceType::EMOJI
                | ResourceType::GUILD
                | ResourceType::MEMBER
                | ResourceType::ROLE
                | ResourceType::USER
                | ResourceType::USER_CURRENT,
        ),
    ));

    // create postgres connection
    let connect_opts = config
        .database_url
        .parse::<PgConnectOptions>()
        .unwrap_or_else(|_| panic!("couldn't parse db url: {}", config.database_url))
        .log_statements(LevelFilter::Trace)
        .log_slow_statements(LevelFilter::Warn, Duration::from_secs(5));
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await
        .expect("error connecting to db");

    // create AMQP connection
    let (amqp_tx, mut amqp_rx) = mpsc::unbounded_channel::<Vec<u8>>();
    // create AMQP connection
    let mut amqp = AmqpHandle::try_from_str(
        &config.rabbitmq_address,
        ConnectionArguments::new("discord"),
        Some(amqp_tx),
    )
    .expect("couldn't create amqp client");
    amqp.wait_start().await.expect("couldn't connect to amqp");

    tracing::info!("running migrations...");
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("error running migrations");

    // register interaction handlers
    tracing::info!("registering handlers");
    let mut registry = Registry::<context::Services>::new();

    registry.register(modules::emoji::build());
    registry.register(modules::pk::build());
    registry.register(modules::stats::build());

    // core should always be registered last because it needs the data from
    // previous modules to set up
    registry.register(modules::core::build(&registry));

    // only run scheduled tasks on the "primary" handler
    if config.handler_id != 0 {
        registry.tasks.clear();
    }

    // we don't need to mutate registry anymore after this
    let registry = Arc::new(registry);

    let services = Arc::new(context::Services {
        handler_id: config.handler_id,

        pk: Arc::new(PkClient {
            user_agent: format!("Tulpje {}", version!()),
            ..Default::default()
        }),
        cache: Arc::clone(&cache),
        redis,
        db,
        registry: Arc::clone(&registry),
    });
    let mut framework = Framework::new(
        registry,
        client,
        app_id,
        services,
        Some(|ctx| {
            Box::pin(async move {
                // only register commands on the "primary" handler to avoid
                // sending too many requests to discord
                if ctx.services.handler_id != 0 {
                    return Ok(());
                }

                tracing::info!("registering global commands");
                ctx.interaction()
                    .set_global_commands(&ctx.services.registry.global_commands())
                    .await
                    .map_err(|err| format!(".set_global_commands() error: {}", err))?;

                Ok(())
            })
        }),
    );

    framework.start().await.expect("error starting framework");

    let sender = framework.sender();
    let main_handle = tokio::spawn(async move {
        loop {
            let Some(message) = amqp_rx.recv().await else {
                break;
            };

            let (meta, event) = match parse_delivery(message) {
                Ok((meta, event)) => (meta, event),
                Err(err) => {
                    tracing::error!(?err, "couldn't parse delivery");
                    continue;
                }
            };

            handle_message(&cache, &sender, meta, event).await;
        }
    });

    // listen for SIGTERM/SIGINT signal
    {
        tokio::select! {
            _ = sigint.recv() => {},
            _ = sigterm.recv() => {},
        }
        tracing::info!("shutting down...");
    }

    amqp.shutdown();
    tracing::trace!("waiting for amqp loop to exit...");
    if let Err(err) = amqp.join().await {
        tracing::error!("error joining amqp: {err}");
    }

    tracing::trace!("waiting for main loop to exit...");
    if let Err(err) = main_handle.await {
        tracing::error!("error joining main_handle: {err}");
    }

    framework.shutdown().await;
    tracing::trace!("waiting for framework to exit...");
    if let Err(err) = framework.join().await {
        tracing::error!("error joining framework: {err}");
    }

    tracing::info!("cleanup finished, exiting...");
}

#[tracing::instrument(name="event", fields(shard = meta.shard, uuid = %meta.uuid), skip_all)]
async fn handle_message(cache: &Cache, sender: &Sender, meta: Metadata, event: Event) {
    if let Err(err) = cache.update(&event).in_current_span().await {
        tracing::warn!("error updating cache: {err}");
    }

    tracing::debug!("{:?} received", event.kind());

    if let Err(err) = sender.with_span(meta, event, Span::current()) {
        tracing::error!("error queueing event: {err}");
    };
}

fn parse_delivery(message: Vec<u8>) -> Result<(Metadata, Event), Box<dyn std::error::Error>> {
    let discord_event = serde_json::from_str::<DiscordEvent>(&String::from_utf8(message)?)?;

    // TODO: Don't clone discord_event.payload for debugging stuff, find a better way, ideally just
    //       logging the event type somehow
    Ok((
        discord_event.meta,
        twilight_gateway::Event::from(
            twilight_gateway::parse(
                discord_event.payload.clone(),
                twilight_gateway::EventTypeFlags::all(),
            )?
            .ok_or_else(|| {
                format!(
                    "twilight_gateway::parse returned None, payload: {}",
                    discord_event.payload
                )
            })?,
        ),
    ))
}
