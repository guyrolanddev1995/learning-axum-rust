use std::error::Error;
use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use std::future::Future;
use tokio::time::Instant;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct LoggingService<S> {
    inner: S,
    prefix: &'static str
}

impl<S, Request> Service<Request> for LoggingService<S>
where
    S: Service<Request>,
    S::Future: Send + 'static,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    Request: std::fmt::Debug
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
            let duration = start.elapsed();

            match &result {
                Ok(response) => {
                    println!("[{}] Response: {:?} in {:?}", prefix, response, duration)
                }
                Err(error) => {
                    println!("[{}] Error: {:?}", prefix, error)
                }
            }

            result
        })
    }
}

#[derive(Clone)]
pub struct LoggingLayer {
    prefix: &'static str
}

impl LoggingLayer {
    pub fn new(prefix: &'static str) -> Self {
        Self { prefix }
    }
}

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggingService { inner, prefix: self.prefix }
    }
}

// ============== TIMEOUT MIDDLEWARE ==============
#[derive(Debug)]
pub struct TimeoutExpired;

impl fmt::Display for TimeoutExpired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Request timed out")
    }
}

impl Error for TimeoutExpired {}

#[derive(Clone)]
pub struct TimeoutService<S> {
    inner: S,
    duration: Duration
}

impl<S, Request> Service<Request> for TimeoutService<S>
where
    S: Service<Request>,
    S::Future: Send + 'static,
    S::Error: Into<Box<dyn Error + Send + Sync>> + 'static,
    Request: Send + 'static
{
    type Response = S::Response;
    type Error = Box<dyn Error + Send + Sync>;
    type Future =Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let duration = self.duration;
        let fut = self.inner.call(req);

        Box::pin(async move {
            tokio::select! {
                res = fut => {
                    res.map_err(Into::into)
                }

                _ = tokio::time::sleep(duration) => {
                    Err(Box::new(TimeoutExpired) as Box<dyn Error + Send + Sync>)
                }
            }
        })
    }
}

#[derive(Clone)]
pub struct TimeoutLayer {
    duration: Duration
}

impl TimeoutLayer {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

impl<S> Layer<S> for TimeoutLayer {
    type Service = TimeoutService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TimeoutService { inner, duration: self.duration }
    }
}


// CREATING A SIMPLE SERVICE IN ORDER TO MASTER MIDDLEWARE //

struct Greeter;

impl Service<String> for Greeter {
    type Response = String;
    type Error = std::convert::Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, name: String) -> Self::Future {
        Box::pin(async move {
            Ok(format!("Hello, {}!", name))
        })
    }
}