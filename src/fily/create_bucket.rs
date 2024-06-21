use std::sync::Arc;

use axum::extract::Path;
use axum::Extension;
use bytes::Bytes;
use tracing::debug;

use super::Config;

pub async fn handle(config: Extension<Arc<Config>>, Path(bucket): Path<String>, body: Bytes) {
    debug!("creating bucket");

    debug!("body -> {:?}", body);

    tokio::fs::create_dir_all(format!("{}/{}", config.location, bucket))
        .await
        .unwrap();
}
