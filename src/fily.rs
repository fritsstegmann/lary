use axum::body::Bytes;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, put};
use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::debug;

async fn create_bucket(Path(bucket): Path<String>) {
    tokio::fs::create_dir_all(bucket).await.unwrap();
}

async fn delete_object(Path((bucket, file)): Path<(String, String)>) -> impl IntoResponse {
    let s = format!("{}/{}", bucket, file);
    let path = std::path::Path::new(&s);
    tokio::fs::remove_file(path).await.unwrap();
    StatusCode::OK
}

async fn get_object(Path((bucket, file)): Path<(String, String)>) -> impl IntoResponse {
    let s = format!("{}/{}", bucket, file);
    let path = std::path::Path::new(&s);
    (StatusCode::OK, tokio::fs::read(&path).await.unwrap())
}

async fn put_object(
    Path((bucket, file)): Path<(String, String)>,
    bytes: Bytes,
) -> impl IntoResponse {
    debug!("debug {}, {} -> {:?}", bucket, file, bytes);

    let s = format!("{}/{}", bucket, file);
    let path = std::path::Path::new(&s);
    let prefix = path.parent().unwrap();
    tokio::fs::create_dir_all(prefix).await.unwrap();

    tokio::fs::write(&path, bytes.as_ref()).await.unwrap();
    (StatusCode::OK, "")
}

pub async fn run() -> anyhow::Result<()> {
    // build our application with a route
    let app = Router::new()
        .route("/:bucket", put(create_bucket))
        .route("/:bucket/:file", get(get_object))
        .route("/:bucket/:file", put(put_object))
        .route("/:bucket/:file", delete(delete_object))
        .layer(TraceLayer::new_for_http());

    debug!("running axum server");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8333").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
