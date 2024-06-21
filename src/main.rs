mod app_error;
mod fily;
mod logy;
mod middleware;

use std::str::FromStr;

use clap::{command, Parser, Subcommand};
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

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Serve,
    Fily,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let args = Args::parse();

    let config_file_config = String::from_utf8(fs::read("./config.toml").await?)?;

    let config: Config = toml::from_str(&config_file_config)?;

    tracing_subscriber::fmt()
        .with_max_level(Level::from_str(&config.log_level).unwrap())
        .with_level(true)
        .with_thread_names(true)
        .with_target(true)
        .init();

    match args.cmd {
        Commands::Serve => {
            let fily = tokio::spawn(fily::run(config.fily));
            let logy = tokio::spawn(logy::run());
            let (_, _) = join!(fily, logy);
        }
        Commands::Fily => {}
    }

    Ok(())
}
