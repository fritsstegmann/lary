use std::sync::Arc;

use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use hyper::StatusCode;

use crate::app_error::AppError;

use super::{error_response, Config};

pub async fn handle(
    config: Extension<Arc<Config>>,
    Path((bucket, file)): Path<(String, String)>,
) -> Result<Response, AppError> {
    let s = format!("{}/{}/{}", config.location, bucket, file);
    let path = std::path::Path::new(&s);

    let file_results = tokio::fs::read(&path).await;

    if let Ok(file_results) = file_results {
        return Ok((StatusCode::OK, file_results).into_response());
    }

    return Ok(error_response::Error {
        code: "NotFound".to_string(),
        message: "The resource you requested does no exist".to_string(),
        resource: path.to_str().unwrap().to_string(),
        request_id: "".to_string(),
    }
    .into_response());
}
