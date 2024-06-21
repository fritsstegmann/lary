use std::sync::Arc;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Extension;
use bytes::Bytes;
use hyper::{HeaderMap, StatusCode};
use tracing::debug;

use super::Config;

pub async fn handle(
    config: Extension<Arc<Config>>,
    headers: HeaderMap,
    Path((bucket, file)): Path<(String, String)>,
    bytes: Bytes,
) -> impl IntoResponse {
    debug!("headers: {:?}", headers);

    let s = format!("{}/{}/{}", config.location, bucket, file);
    let path = std::path::Path::new(&s);
    let prefix = path.parent().unwrap();
    tokio::fs::create_dir_all(prefix).await.unwrap();

    tokio::fs::write(&path, bytes.as_ref()).await.unwrap();
    (StatusCode::OK, "")
}
