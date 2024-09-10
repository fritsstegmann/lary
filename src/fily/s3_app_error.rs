use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct S3Error {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Resource")]
    resource: String,
    #[serde(rename = "RequestId")]
    request_id: String,
}

// Make our own error that wraps `anyhow::Error`.
pub struct S3AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for S3AppError {
    fn into_response(self) -> Response {
        let err = S3Error {
            code: "AccountProblem".to_string(),
            message: self.0.to_string(),
            resource: "/".to_string(),
            request_id: "".to_string(),
        };

        let err = to_string(&err).unwrap();
        (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for S3AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
