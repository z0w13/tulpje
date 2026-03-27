use tracing_subscriber::{
    EnvFilter, Layer as _, fmt::layer, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

pub fn init() {
    // get configuration from environment
    let log_format = std::env::var("RUST_LOG_FORMAT").unwrap_or_else(|_| String::from("full"));
    let log_source =
        std::env::var("RUST_LOG_SOURCE").unwrap_or_else(|_| String::from("false")) == "true";

    // generic options
    let output_format = layer().with_file(log_source).with_line_number(log_source);

    // use specific default format
    let output_format = match log_format.as_str() {
        "json" => output_format.json().boxed(),
        "pretty" => output_format.pretty().boxed(),
        "compact" => output_format.compact().boxed(),
        "full" => output_format.boxed(),
        _ => panic!("Unknown RUST_LOG_FORMAT, full, compact, pretty, and json supported"),
    };

    // initialise logging
    tracing_subscriber::registry()
        .with(output_format)
        .with(EnvFilter::from_default_env())
        .init();
}
