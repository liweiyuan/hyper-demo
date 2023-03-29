use hyper::service::service_fn;
use hyper::service::Service;
use hyper::Server;
use hyper::{body::HttpBody, service::make_service_fn, Body, Method, Request, Response};
use std::{convert::Infallible, net::SocketAddr};

#[derive(Clone)]
struct AppContext {}

async fn handler(_context: AppContext, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.uri().path(), req.method()) {
        ("/", &Method::GET) => Ok(Response::new(Body::from("Hello, World"))),
        ("/index", &Method::GET) => Ok(Response::new(Body::from("Hello from index."))),
        _ => Ok(Response::new(Body::from("Hello empty"))),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let context = AppContext {};

    let make_service = make_service_fn(move |_conn| {
        let context = context.clone();

        let service = service_fn(move |req| handler(context.clone(), req));
        async move { Ok::<_, anyhow::Error>(service) }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)).into();

    let server = Server::bind(&addr).serve(make_service);

    print!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
