use std::sync::Arc;

use anyhow::Context;
use axum::body::Body;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};

use crate::app_error::AppError;

use super::Config;

#[derive(Deserialize, Serialize, Debug)]
struct Bucket {
    #[serde(rename = "CreationDate")]
    creation_date: String,
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Deserialize, Serialize)]
struct List {
    #[serde(rename = "Bucket", default)]
    buckets: Vec<Bucket>,
}

#[derive(Deserialize, Serialize)]
struct ListAllMyBucketsResult {
    #[serde(rename = "Buckets")]
    buckets: List,
    #[serde(rename = "Owner")]
    owner: String,
}

impl IntoResponse for ListAllMyBucketsResult {
    fn into_response(self) -> axum::response::Response {
        let err = to_string(&self).unwrap().into_bytes();
        let mut resp = Response::new(Body::from(err));
        *resp.status_mut() = StatusCode::OK;

        resp
    }
}

pub async fn handle(config: Extension<Arc<Config>>) -> Result<Response, AppError> {
    let location = &config.location;

    let mut buckets: Vec<Bucket> = vec![];

    let mut read_dir = tokio::fs::read_dir(location).await?;
    while let Some(entry) = read_dir.next_entry().await? {
        if let Ok(metadata) = entry.metadata().await {
            if metadata.is_dir() {
                let created_time: DateTime<Utc> = metadata.created()?.into();

                buckets.push(Bucket {
                    creation_date: created_time.format("%FT%T%:z").to_string(),
                    name: entry
                        .file_name()
                        .to_str()
                        .context("failed turning os string to rust string")
                        .unwrap()
                        .to_string(),
                });
            }
        }
    }

    Ok(ListAllMyBucketsResult {
        buckets: List { buckets },
        owner: "".to_string(),
    }
    .into_response())
}
