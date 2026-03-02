use std::error::Error;
use std::fmt::Formatter;
use std::pin::Pin;
use std::task::{Context, Poll};
use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use tower::{Layer, Service, ServiceBuilder};
use crate::api::middleware::logging::{LoggingLayer, LoggingMiddleware};

type BoxError = Box<dyn Error + Send + Sync>;

#[derive(Debug)]
pub enum AuthError<E> {
    Unauthorized(String),
    Inner(E)
}

impl<E: std::fmt::Display> std::fmt::Display for AuthError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AuthError::Inner(err) => write!(f, "Inner error: {}", err)
        }
    }
}

impl<E: std::fmt::Display + std::fmt::Debug> std::error::Error for AuthError<E> {}

#[derive(Clone)]
pub struct AuthLayer {
    pub expected_token: String
}

impl AuthLayer {
    pub fn new(token: impl Into<String>) -> Self {
        Self { expected_token: token.into() }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner, expected_token: self.expected_token.clone() }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    pub inner: S,
    pub expected_token: String
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<BoxError> + 'static,
{
    type Response = Response<Body>;
    type Error = BoxError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), BoxError>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let auth_request = req
            .headers()
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .map(|token| token.to_string());

        let is_valid = match &auth_request {
            Some(header) if header.starts_with("Bearer ") => {
                let token = &header[7..];
                token == self.expected_token
            }
            _ => false
        };

        if!is_valid {
            return Box::pin(async {
                Err(
                    Box::new(AuthError::<String>::Unauthorized("Invalid token".into())) as BoxError
                )
            });
        }

        let future = self.inner.call(req);

        Box::pin(async move {
            future.await.map_err(Into::into)
        })
    }
}

struct Greeter;

fn run() {
    let with_auth = AuthMiddleware {
        inner: LoggingMiddleware {
            inner: Greeter,
            prefix: "App"
        },
        expected_token: "123".to_string()
    };

    let svc = ServiceBuilder::new()
        .layer(AuthLayer::new("123"))
        .layer(LoggingLayer::new("App"))
        .service(Greeter);
}




