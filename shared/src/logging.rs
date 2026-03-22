use tracing_subscriber::{
    EnvFilter, Layer as _, fmt::layer, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

pub fn init() {
    let log_format = std::env::var("RUST_LOG_FORMAT").unwrap_or_else(|_| String::from("full"));
    let format = match log_format.as_str() {
        "json" => layer().json().boxed(),
        "pretty" => layer().pretty().boxed(),
        "compact" => layer().compact().boxed(),
        "full" => layer().boxed(),
        _ => panic!("Unknown RUST_LOG_FORMAT, full, compact, pretty, and json supported"),
    };
    tracing_subscriber::registry()
        .with(format)
        .with(EnvFilter::from_default_env())
        .init();
}
