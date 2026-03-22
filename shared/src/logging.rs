use tracing_subscriber::fmt;

pub fn init() {
    match std::env::var("RUST_LOG_FORMAT")
        .unwrap_or_else(|_| String::from("full"))
        .as_str()
    {
        "json" => fmt().json().init(),
        "pretty" => fmt().pretty().init(),
        "compact" => fmt().compact().init(),
        "full" => fmt().init(),
        _ => panic!("Unknown RUST_LOG_FORMAT, full, compact, pretty, and json supported"),
    };
}
