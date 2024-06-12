use std::task::{Context, Poll};

use axum::http::Request;
use tower::{Layer, Service};
use tracing::debug;

#[derive(Clone)]
pub struct AuthLayer {}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthService { inner }
    }
}

#[derive(Clone)]
pub struct AuthService<S> {
    inner: S,
}

impl<S, B> Service<Request<B>> for AuthService<S>
where
    S: Service<Request<B>>,
    B: std::fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let uri = req.uri();

        debug!("host {:?}", uri.host());

        self.inner.call(req)
    }
}
