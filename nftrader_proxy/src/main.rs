use std::{convert::Infallible, error::Error, fmt::Display, net::SocketAddr, pin::Pin, task::Poll, time::{Duration, Instant}};
use hyper::{Body, Request, Response, Server, server::conn::AddrStream, service::{make_service_fn, service_fn}};
use tokio::time::Sleep;
use tower::{BoxError, Service};
use futures::{Future, future::{Ready, ready}};
use pin_project::pin_project;

#[tokio::main]
async fn main() {
    env_logger::init();
    let addr = SocketAddr::new([127, 0, 0, 1].into(), 3000);

    let make_service = make_service_fn(|_conn: &AddrStream| async {
        let svc = service_fn(handle);
        let svc = Timeout::new(svc, Duration::from_secs(2));
        let svc = Logging::new(svc);
        Ok::<_, Infallible>(svc)
    });

    let server = Server::bind(&addr).serve(make_service);
    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    };
}

#[derive(Clone)]
struct HelloWorld;

impl Service<Request<Body>> for HelloWorld {
    type Response = Response<Body>;

    type Error = Infallible;

    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        ready(Ok(Response::new(Body::from("derpity derp"))))
    }
}

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(Response::new("Hello, world!".into()))
}

#[derive(Clone)]
struct Logging<S> {
    inner: S
}

impl<S> Logging<S> {
    fn new(inner: S) -> Self { Self { inner } }
}

impl<B, S> Service<Request<B>> for Logging<S>
where
    S: Service<Request<B>, Response = Response<B>> + Send + Clone + 'static,
    B: Send + 'static,
    S::Future: Send + 'static
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = LoggingFuture<S::Future>;
    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let timer = Instant::now();
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        log::debug!("processing request {} {}", method, path);
        LoggingFuture { future: self.inner.call(req), method, path, timer}
    }
}

#[pin_project]
struct LoggingFuture<F> {
    #[pin]
    future: F,
    method: hyper::Method,
    path: String,
    timer: Instant
}

impl<F, S, E> Future for LoggingFuture<F>
where
    F: Future<Output = Result<Response<S>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let res: F::Output = match this.future.poll(cx) {
            Poll::Ready(res) => res,
            Poll::Pending => { return Poll::Pending }
        };

       // let status = match res {
       //     Ok(st) => st.as_u16()
       // };
        let duration = this.timer.elapsed();
        log::debug!("finished processing request {} {}; time = {:?}", this.method, this.path, duration);
        Poll::Ready(res)
    }
}


#[derive(Clone)]
struct Timeout<S> {
    inner: S,
    timeout: Duration
}

impl<S> Timeout<S> {
    fn new(inner: S, timeout: Duration) -> Self { Self { inner, timeout } }
}

impl<S, R> Service<R> for Timeout<S>
where
    S: Service<R>,
    S::Error: Send + Sync + Error + 'static
{
    type Response = S::Response;

    type Error = BoxError;

    type Future = TimeoutFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self.inner.poll_ready(cx) {
            Poll::Ready(Ok(val)) => Poll::Ready(Ok(val)),
            Poll::Ready(Err(err)) => Poll::Ready(Err(Box::new(err))),
            Poll::Pending => return Poll::Pending
        }
    }

    fn call(&mut self, req: R) -> Self::Future {
        TimeoutFuture {
            future: self.inner.call(req),
            sleep: tokio::time::sleep(self.timeout)
        }
    }
}

#[pin_project]
pub struct TimeoutFuture<F> {
    #[pin]
    future: F,
    #[pin]
    sleep: Sleep
}

impl<F, S, E> Future for TimeoutFuture<F>
where F: Future<Output = Result<S, E>>,
      E: Send + Sync + Error + 'static
{
    type Output = Result<S, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.future.poll(cx) {
            Poll::Pending => {},
            Poll::Ready(res) => {
                match res {
                    Ok(res) => return Poll::Ready(Ok(res)),
                    Err(err) => return Poll::Ready(Err(Box::new(err)))
                } 
            }
        };

        match this.sleep.poll(cx) {
            Poll::Pending => {},
            Poll::Ready(_) => return Poll::Ready(Err(Box::new(ElapsedError)))
        }
        Poll::Pending
    }
}

#[derive(Debug)]
struct ElapsedError;

impl Display for ElapsedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Elapsed timeout")
    }
}

impl Error for ElapsedError {}
