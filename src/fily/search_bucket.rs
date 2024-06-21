use std::sync::Arc;

use axum::response::IntoResponse;
use axum::Extension;
use hyper::StatusCode;

use crate::app_error::AppError;

use super::Config;

pub async fn handle(_: Extension<Arc<Config>>) -> Result<impl IntoResponse, AppError> {
    Ok(StatusCode::OK)
}
