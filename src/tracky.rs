use std::env;
use std::fmt::Display;
use std::net::SocketAddr;

use anyhow::anyhow;
use axum::extract::{ConnectInfo, OriginalUri};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{debug_handler, Json, Router};
use redis::streams::{StreamReadOptions, StreamReadReply};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::query;
use sqlx::sqlite::SqlitePool;
use sqlx::types::Uuid;
use tokio::join;
use tower_http::trace::TraceLayer;
use tracing::{debug, error};

use crate::app_error::AppError;
use crate::middleware::AuthLayer;

#[derive(Deserialize, Serialize, Debug, Clone)]
enum EventType {
    Visit,
    Custom,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Visit => write!(f, "Visit"),
            Self::Custom => write!(f, "Custom"),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct EventRequestBody {
    name: String,
    event_type: EventType,
    data: Value,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    name: String,
    user_agent: Option<String>,
    event_type: EventType,
    data: Value,
}

#[debug_handler]
async fn track_event(
    headers: HeaderMap,
    OriginalUri(uri): OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(event): Json<EventRequestBody>,
) -> Result<impl IntoResponse, AppError> {
    debug!("tracking event");

    debug!("uri: {:?}", uri.scheme());
    debug!("client ip: {:?}", addr.ip());

    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .map(|v| v.to_str().unwrap().to_string());

    let mut redis = redis::Client::open("redis://127.0.0.1")?
        .get_multiplexed_tokio_connection()
        .await?;

    let event = Event {
        name: event.name,
        event_type: event.event_type,
        data: event.data,
        user_agent,
    };

    let event_json = serde_json::to_string(&event)?;

    redis.xadd("tracky", "*", &[("event", event_json)]).await?;

    Ok(())
}

async fn process_redis_stream() -> anyhow::Result<()> {
    let mut redis = redis::Client::open("redis://127.0.0.1")?
        .get_multiplexed_tokio_connection()
        .await?;
    loop {
        let opts = StreamReadOptions::default().count(1).block(0);
        let result: Option<StreamReadReply> = redis
            .xread_options(&["tracky"], &["$"], &opts)
            .await
            .unwrap();
        if let Some(reply) = result {
            for stream_key in reply.keys {
                for stream_id in stream_key.ids {
                    let event_str = &stream_id
                        .get::<String>("event")
                        .ok_or(anyhow!("failed getting event from redis stream"));

                    match event_str {
                        Ok(event_str) => {
                            let event = serde_json::from_str::<Event>(event_str)?;

                            let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

                            let id = Uuid::new_v4();

                            let event_type = event.event_type.to_string();

                            query!(
                        "INSERT INTO events(id, name, event_type, data) VALUES($1, $2, $3, $4)",
                        id,
                                event.name,
                                event_type,
                                event.data
                    )
                            .execute(&pool)
                            .await?;
                        }
                        Err(e) => {
                            error!("failed getting event from redis {:?}", e);
                        }
                    }
                }
            }
            println!();
        }
    }
}

pub async fn run() -> anyhow::Result<()> {
    // build our application with a route
    let app = Router::new()
        .route("/event", post(track_event))
        .layer(AuthLayer {})
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8335").await.unwrap();
    let web_server = tokio::spawn(async move {
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap()
    });

    let (_, _) = join!(web_server, process_redis_stream());

    Ok(())
}
