#[tokio::main]
async fn main() {
    let token = match std::env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(std::env::VarError::NotPresent) => {
            eprintln!("DISCORD_TOKEN not set");
            std::process::exit(1);
        }
        Err(err) => {
            eprintln!("error reading DISCORD_TOKEN: {}", err);
            std::process::exit(1);
        }
    };

    let client = twilight_http::Client::builder().token(token).build();
    let connection_info = client
        .gateway()
        .authed()
        .await
        .expect("error fetching bot connection info")
        .model()
        .await
        .expect("error decoding bot connection info");

    println!("{}", connection_info.shards);
}
