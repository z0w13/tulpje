use std::error::Error;

use metrics::{counter, describe_counter};
use metrics_exporter_prometheus::PrometheusBuilder;
use redis::aio::ConnectionManager as RedisConnectionManager;
use twilight_gateway::{Event, EventType};

use tulpje_shared::{metrics::MetricsListenAddr, version};

pub(crate) fn install(
    listen_addr: MetricsListenAddr,
    redis: RedisConnectionManager,
    shard_id: u32,
) -> Result<(), Box<dyn Error>> {
    // install metrics collector and exporter
    tulpje_shared::metrics::install(
        PrometheusBuilder::new(),
        listen_addr,
        redis,
        format!("gateway-{}", shard_id),
        version!(),
    )?;

    // define metrics
    describe_counter!("gateway_events", "Discord Gateway Events");

    Ok(())
}

pub(crate) fn track_gateway_event(shard: u32, event: &Event) {
    let event_name = match event.kind().name() {
        Some(name) => name,
        None => match event.kind() {
            EventType::GatewayClose => "GATEWAY_CLOSE",
            EventType::GatewayHeartbeat => "GATEWAY_HEARTBEAT",
            EventType::GatewayHeartbeatAck => "GATEWAY_HEARTBEAT_ACK",
            EventType::GatewayHello => "GATEWAY_HELLO",
            EventType::GatewayInvalidateSession => "GATEWAY_INVALIDATE_SESSION",
            EventType::GatewayReconnect => "GATEWAY_RECONNECT",
            _ => panic!("unknown event: {:?}", event),
        },
    };

    counter!(
        "gateway_events",
        "shard" => shard.to_string(),
        "event" => event_name.to_string()
    )
    .increment(1);
}
