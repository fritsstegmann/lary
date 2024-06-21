use std::sync::Arc;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Extension;
use hyper::StatusCode;

use crate::app_error::AppError;

use super::Config;

pub async fn handle(
    config: Extension<Arc<Config>>,
    Path((bucket, file)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let s = format!("{}/{}/{}", config.location, bucket, file);
    let path = std::path::Path::new(&s);
    match tokio::fs::remove_file(path).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Ok(StatusCode::NOT_FOUND),
    }
}
