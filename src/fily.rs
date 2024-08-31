mod create_bucket;
mod create_general_bucket;
mod delete_bucket;
mod delete_object;
mod error_response;
mod get_object;
mod list_buckets;
mod put_object;
mod search_bucket;

use std::sync::Arc;

use axum::{
    routing::{delete, get, put},
    Extension, Router,
};
use serde::Deserialize;
use tower_http::trace::TraceLayer;
use tracing::info;

#[derive(Deserialize)]
pub struct Config {
    location: String,
    port: String,
    address: String,
}

pub async fn run(config: Config) -> anyhow::Result<()> {
    let config_state = Arc::new(config);

    let port = config_state.port.clone();
    let address = config_state.address.clone();

    // build our application with a route
    let app = Router::new()
        .route("/", get(list_buckets::handle))
        .route("/", put(create_general_bucket::handle))
        .route("/:bucket", put(create_bucket::handle))
        .route("/:bucket", get(search_bucket::handle))
        .route("/:bucket", delete(delete_bucket::handle))
        .route("/:bucket/:file", get(get_object::handle))
        .route("/:bucket/:file", put(put_object::handle))
        .route("/:bucket/:file", delete(delete_object::handle))
        .layer(Extension(config_state))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", &address, &port))
        .await
        .unwrap();

    info!("running fily server on {}:{}", &address, &port);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
