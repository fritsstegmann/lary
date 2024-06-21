mod app_error;
mod fily;
mod logy;
mod middleware;

use std::str::FromStr;

use dotenv::dotenv;
use serde::Deserialize;
use tokio::fs;
use tokio::join;
use tracing::Level;

#[derive(Deserialize)]
struct Config {
    log_level: String,
    fily: fily::Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config_file_config = String::from_utf8(fs::read("./config.toml").await?)?;

    let config: Config = toml::from_str(&config_file_config)?;

    tracing_subscriber::fmt()
        // Configure formatting settings.
        .with_max_level(Level::from_str(&config.log_level).unwrap())
        .with_level(true)
        .with_thread_names(true)
        .with_target(true)
        // Set the subscriber as the default.
        .init();

    // let tracky = tokio::spawn(tracky::run());
    let fily = tokio::spawn(fily::run(config.fily));
    let logy = tokio::spawn(logy::run());

    let (_, _) = join!(fily, logy);

    // let _ = join!(fily);

    // let (_, _, _, _) = join!(fily, logy, tracky);
    Ok(())
}
