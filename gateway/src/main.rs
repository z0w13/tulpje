use std::{env, error::Error, time::Duration};

use futures_util::StreamExt;
use redis::aio::ConnectionManagerConfig;
use twilight_gateway::EventTypeFlags;
use twilight_model::gateway::{
    OpCode,
    event::{Event, GatewayEventDeserializer},
    payload::outgoing::{identify::IdentifyProperties, update_presence::UpdatePresencePayload},
    presence::{Activity, MinimalActivity, Status},
};

use reconnecting_amqp::{AmqpHandle, ConnectionArguments};
use tulpje_shared::{DiscordEvent, version};

mod config;
mod metrics;
mod shard_state;

use config::Config;

#[tokio::main]
async fn main() {
    // set-up logging
    tracing_subscriber::fmt::init();
    tracing::info!("starting tulpje-gateway {} ...", version!());

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
    let amqp_chan = amqp.sender();

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

    // create shard state manager
    let mut shard_state_manager = shard_state::ShardManager::new(redis.clone(), shard_id.number());

    // initialisation done, ratelimit on session_limit
    tracing::info!("waiting for gateway queue...");
    reqwest::get(config.discord_gateway_queue)
        .await
        .expect("error waiting for gateway queue");

    // start main loop
    tracing::info!("starting main loop...");

    let main_handle = tokio::spawn(async move {
        loop {
            match shard.next().await {
                Some(Ok(twilight_gateway::Message::Close(frame))) => {
                    tracing::warn!(?frame, "gateway connection closed");

                    // have to handle this hear separate as twilight_gateway::parse doesn't
                    // parse into Event::GatewayClose as that's a separate event type
                    if let Err(err) = shard_state_manager
                        .handle_event(Event::GatewayClose(frame), shard.latency())
                        .await
                    {
                        tracing::error!("error updating shard state: {}", err);
                    }
                }
                Some(Ok(twilight_gateway::Message::Text(text))) => {
                    let opcode = match parse_opcode(&text) {
                        Err(err) => {
                            tracing::error!(?err, "couldn't parse opcode");
                            continue;
                        }
                        Ok(Some(opcode)) => opcode,
                        Ok(None) => {
                            tracing::error!("received empty opcode");
                            continue;
                        }
                    };

                    tracing::trace!(?opcode, "opcode received");

                    if let Ok(Some(event)) =
                        twilight_gateway::parse(text.clone(), EventTypeFlags::all())
                    {
                        let event = twilight_model::gateway::event::Event::from(event);

                        // track event metrics
                        metrics::track_gateway_event(shard_id.number(), &event);

                        if let Err(err) = shard_state_manager
                            .handle_event(event.clone(), shard.latency())
                            .await
                        {
                            tracing::error!("error updating shard state: {}", err);
                        }
                    }

                    // only publish non-gateway events, aka everything DISPATCH
                    if opcode == OpCode::Dispatch {
                        let event = DiscordEvent::new(shard_id.number(), text);
                        let serialized_event = match serde_json::to_vec(&event) {
                            Ok(val) => val,
                            Err(err) => {
                                tracing::error!("error serializing event: {}", err);
                                continue;
                            }
                        };

                        if let Err(err) = amqp_chan.send(serialized_event) {
                            tracing::error!("error sending event to amqp: {}", err);
                            continue;
                        }

                        tracing::debug!(
                            uuid = ?event.meta.uuid,
                            shard = event.meta.shard,
                            "event sent"
                        );
                    }
                }
                Some(Err(err)) => {
                    tracing::error!(?err, "error receiving discord message");
                }
                None => {
                    tracing::error!("empty message, connection irrecoverably closed, exiting...");
                    break;
                }
            };
        }
    });

    main_handle.await.expect("error joining main_handle");
    amqp.join().await.expect("error joining amqp");
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

fn parse_opcode(event: &str) -> Result<Option<OpCode>, Box<dyn Error>> {
    let Some(gateway_deserializer) = GatewayEventDeserializer::from_json(event) else {
        return Err("couldn't deserialise event".into());
    };

    Ok(OpCode::from(gateway_deserializer.op()))
}
