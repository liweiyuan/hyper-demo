use std::net::SocketAddr;

use hyper::{Body, Response};
use ray::router::{AppContext, Router};

pub mod ray;
pub mod server;

#[tokio::main]
async fn main() {
    let handle_hello = |c: &mut AppContext| {
        c.response = Response::new(Body::from("Hello, world!"));
    };

    let router = Router::new().get("/index", handle_hello);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)).into();

    server::run(addr, router).await;
}
