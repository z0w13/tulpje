use metrics_exporter_prometheus::PrometheusBuilder;
use redis::aio::ConnectionManager as RedisConnectionManager;
use tulpje_shared::version;

pub(crate) fn install(
    redis: RedisConnectionManager,
    handler_id: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // install metrics collector and exporter
    tulpje_shared::metrics::install(
        PrometheusBuilder::new(),
        redis,
        format!("handler-{}", handler_id),
        version!(),
    )?;

    // define metrics
    // ..

    Ok(())
}
