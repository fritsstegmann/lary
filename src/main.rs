use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, io, str::FromStr};
use tokio::net::UdpSocket;
use tracing::{debug, error, Level};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Log {
    #[serde(rename(deserialize = "@timestamp"))]
    timestamp: String,
    #[serde(rename(deserialize = "@version"))]
    version: u16,
    host: String,
    message: String,
    channel: String,
    level: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        // Configure formatting settings.
        .with_max_level(Level::DEBUG)
        .with_level(false)
        .with_thread_names(false)
        .with_target(false)
        .without_time()
        // Set the subscriber as the default.
        .init();

    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buf = [0; 1024];
    loop {
        let (_, _) = sock.recv_from(&mut buf).await?;
        let log = CStr::from_bytes_until_nul(&buf).unwrap();

        match serde_json::from_str::<Log>(log.to_str().unwrap()) {
            Ok(json) => {
                let datetime = DateTime::<Local>::from_str(&json.timestamp);

                let message = format!(
                    "{} {}.{}: {}",
                    datetime.unwrap().format("[%Y-%m-%d %H:%M:%S]"),
                    json.channel,
                    json.level,
                    json.message
                );

                debug!("{}", message);
            }
            Err(e) => {
                error!("{:?}", e);
            }
        }
    }
}
