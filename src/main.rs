use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
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
    let make_service = make_service_fn(move |_conn| {
        let context = AppContext {};

        let service = service_fn(move |req| handler(context.clone(), req));
        async move { Ok::<_, anyhow::Error>(service) }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)).into();

    let server = Server::bind(&addr).serve(make_service);

    print!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
