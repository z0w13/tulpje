mod amqp;
mod config;
mod context;
mod db;
mod metrics;
mod modules;

use std::{env, sync::Arc, time::Duration};

use bb8_redis::RedisConnectionManager;
use context::Services;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions as _,
};
use tracing::log::LevelFilter;

use tulpje_framework::{Error, Framework, Registry};
use tulpje_shared::{DiscordEvent, DiscordEventMeta};

use config::Config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // load .env into environment vars, ignore if not found
    match dotenvy::dotenv().map(|_| ()) {
        Err(err) if err.not_found() => {
            tracing::warn!("no .env file found");
        }
        result => result.expect("error loading .env file"),
    };

    // parse TASK_SLOT env var if it exists and use it for the handler id
    if let Ok(task_slot) = env::var("TASK_SLOT") {
        tracing::info!("TASK_SLOT env var found, using it for handler id");
        tracing::debug!("TASK_SLOT = {}", task_slot);

        env::set_var(
            "HANDLER_ID",
            format!(
                "{}",
                task_slot.parse::<u64>().expect("couldn't parse task_slot") - 1
            ),
        );
    }

    // create config from environment vars
    let config = Config::from_env().expect("error loading config from env");

    // set-up logging
    tracing_subscriber::fmt::init();

    // needed for fetching recommended shard count
    let client = Arc::new(
        twilight_http::Client::builder()
            .proxy(config.discord_proxy, true)
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
    let manager = RedisConnectionManager::new(config.redis_url).expect("error initialising redis");
    let redis = bb8::Pool::builder()
        .build(manager)
        .await
        .expect("error initialising redis pool");

    // set-up metrics
    tracing::info!("installing metrics collector and exporter...");
    metrics::install(redis.clone(), config.handler_id).expect("error setting up metrics");

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
    let mut amqp = amqp::create(&config.rabbitmq_address).await;

    tracing::info!("running migrations...");
    sqlx::migrate!("../migrations")
        .run(&db)
        .await
        .expect("error running migrations");

    // register interaction handlers
    tracing::info!("registering handlers");
    let mut registry = Registry::<Services>::new();

    registry.register(modules::emoji::build());
    registry.register(modules::pk::build());
    registry.register(modules::stats::build());

    // core should always be registered last because it needs the data from
    // previous modules to set up
    registry.register(modules::core::build(&registry));

    // we don't need to mutate registry anymore after this
    let registry = Arc::new(registry);

    let services = Arc::new(context::Services {
        handler_id: config.handler_id,

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
                tracing::info!("registering global commands");
                ctx.interaction()
                    .set_global_commands(&ctx.services.registry.global_commands())
                    .await?;

                // register guild commands
                let guild_modules = modules::core::db_all_guild_modules(&ctx.services.db)
                    .await
                    .expect("error fetching guild modules");
                for (guild_id, modules) in guild_modules {
                    if let Err(err) = modules::core::set_guild_commands_for_guild(
                        &modules,
                        guild_id,
                        ctx.interaction(),
                        &ctx.services.registry,
                    )
                    .await
                    {
                        tracing::error!(
                            "error registering commands for guild {}: {}",
                            guild_id,
                            err
                        );
                    }
                }

                Ok(())
            })
        }),
    );

    framework.start().await.expect("error starting framework");

    let sender = framework.sender();
    let main_handle = tokio::spawn(async move {
        loop {
            let Some(message) = amqp.recv().await else {
                break;
            };

            let (meta, event) = match parse_delivery(message) {
                Ok((meta, event)) => (meta, event),
                Err(err) => {
                    tracing::error!(?err, "couldn't parse delivery");
                    continue;
                }
            };

            tracing::debug!(
                event = ?event.kind(),
                uuid = ?meta.uuid,
                shard = meta.shard,
                "event received",
            );

            if let Err(err) = sender.send(meta, event) {
                tracing::error!("error queueing event: {}", err);
            };
        }
    });

    framework.join().await.expect("error joining framework");
    main_handle.await.expect("error joining main_handle");

    Ok(())
}

fn parse_delivery(
    message: Vec<u8>,
) -> Result<(DiscordEventMeta, twilight_model::gateway::event::Event), Box<dyn std::error::Error>> {
    let discord_event = serde_json::from_str::<DiscordEvent>(&String::from_utf8(message)?)?;

    Ok((
        discord_event.meta,
        twilight_gateway::Event::from(
            twilight_gateway::parse(
                discord_event.payload,
                twilight_gateway::EventTypeFlags::all(),
            )?
            .unwrap(),
        ),
    ))
}
