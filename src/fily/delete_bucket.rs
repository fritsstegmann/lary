use std::sync::Arc;

use axum::Extension;
use bytes::Bytes;
use tracing::{debug, info};

use super::Config;

pub async fn handle(_config: Extension<Arc<Config>>, body: Bytes) {
    info!("delete bucket");

    debug!("body -> {:?}", body);
}
