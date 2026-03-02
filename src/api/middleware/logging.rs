use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct LoggingMiddleware<S> {
    pub inner: S,
    pub prefix: &'static str
}

impl<S, Request> Service<Request> for LoggingMiddleware<S>
where
    S: Service<Request>,
    S::Future: Send + 'static,
    Request: std::fmt::Debug + Send + 'static,
    S::Response: std::fmt::Debug + Send + 'static,
    S::Error: std::fmt::Debug + Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<S::Response, S::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let prefix = self.prefix;
        println!("[{}] Incoming request: {:?}", prefix, req);
        let start = Instant::now();
        let future = self.inner.call(req);

        Box::pin(async move {
            let result = future.await;
            let elapsed = start.elapsed();

            match &result {
                Ok(resp) => println!("[{}] ✅ Réponse ({:?}) : {:?}", prefix, elapsed, resp),
                Err(err) => println!("[{}] ❌ Erreur ({:?}) : {:?}", prefix, elapsed, err),
            }

            result
        })
    }
}

#[derive(Clone)]
pub struct LoggingLayer {
    pub prefix: &'static str
}

impl LoggingLayer {
    pub fn new(prefix: &'static str) -> Self {
        Self { prefix }
    }
}

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggingMiddleware {inner, prefix: self.prefix}
    }
}