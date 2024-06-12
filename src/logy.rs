use std::ffi::CStr;
use std::str::FromStr;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio::net::UdpSocket;
use tracing::{debug, error};

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

pub async fn run() -> anyhow::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080").await.unwrap();
    let mut buf = [0; 1024];
    loop {
        let (_, _) = sock.recv_from(&mut buf).await.unwrap();
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
