use axum::{
    body::Bytes,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, put},
    Router,
};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, io, str::FromStr};
use tokio::{join, net::UdpSocket};
use tower_http::trace::TraceLayer;
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

async fn create_bucket(Path(bucket): Path<String>) {
    tokio::fs::create_dir_all(bucket).await.unwrap();
}

async fn delete_object(Path((bucket, file)): Path<(String, String)>) -> impl IntoResponse {
    let s = format!("{}/{}", bucket, file);
    let path = std::path::Path::new(&s);
    tokio::fs::remove_file(path).await.unwrap();
    StatusCode::OK
}

async fn get_object(Path((bucket, file)): Path<(String, String)>) -> impl IntoResponse {
    let s = format!("{}/{}", bucket, file);
    let path = std::path::Path::new(&s);
    (StatusCode::OK, tokio::fs::read(&path).await.unwrap())
}

async fn put_object(
    Path((bucket, file)): Path<(String, String)>,
    bytes: Bytes,
) -> impl IntoResponse {
    debug!("debug {}, {} -> {:?}", bucket, file, bytes);

    // if tokio::fs::try_exists(&bucket).await.unwrap() {
    let s = format!("{}/{}", bucket, file);
    let path = std::path::Path::new(&s);
    let prefix = path.parent().unwrap();
    tokio::fs::create_dir_all(prefix).await.unwrap();

    tokio::fs::write(&path, bytes.as_ref()).await.unwrap();
    (StatusCode::OK, "")
    // } else {
    //     (StatusCode::BAD_REQUEST, "")
    // }
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

    let filly = tokio::spawn(async {
        // build our application with a route
        let app = Router::new()
            .route("/:bucket", put(create_bucket))
            .route("/:bucket/:file", get(get_object))
            .route("/:bucket/:file", put(put_object))
            .route("/:bucket/:file", delete(delete_object))
            .layer(TraceLayer::new_for_http());

        debug!("running axum server");

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8333").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    let loggy = tokio::spawn(async {
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
    });

    let (_, _) = join!(filly, loggy);

    Ok(())
}
