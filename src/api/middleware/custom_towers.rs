use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};
// pub trait Service<Request> {
//     type Response;
//     type Error;
//     type Future: Future<Output = Result<Self::Response, Self::Error>>;
//
//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
//
//     fn call(&mut self, req: Request) -> Self::Future;
// }
//
// pub trait Layer<S> {
//     type Service;
//
//     fn layer(&self, inner: S) -> Self::Service;
// }


#[derive(Clone)]
struct LoggingLayer {
    prefix: &'static str,
}

impl LoggingLayer {
    pub fn new(prefix: &'static str) -> Self {
        Self { prefix }
    }
}

impl <S> Layer <S> for LoggingLayer {
    type Service = LoggingService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggingService { inner, prefix: self.prefix }
    }
}

#[derive(Clone)]
struct LoggingService<S> {
    inner: S,
    prefix: &'static str,
}

impl<S, Request> Service<Request> for LoggingService<S>
where
    S: Service<Request>,
    S::Future: Send + 'static,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    Request: std::fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
}



