use axum::body::Body;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Error {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Message")]
    pub resource: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let err = to_string(&self).unwrap().into_bytes();
        let mut resp = Response::new(Body::from(err));
        *resp.status_mut() = StatusCode::NOT_FOUND;

        resp
    }
}
