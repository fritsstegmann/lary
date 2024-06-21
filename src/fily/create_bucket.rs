use std::sync::Arc;

use axum::extract::Path;
use axum::Extension;

use super::Config;

pub async fn handle(config: Extension<Arc<Config>>, Path(bucket): Path<String>) {
    tokio::fs::create_dir_all(format!("{}/{}", config.location, bucket))
        .await
        .unwrap();
}
