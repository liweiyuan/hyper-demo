use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

use crate::ray::router::{AppContext, Router};

async fn handler(req: Request<Body>, router: Arc<Router>) -> Result<Response<Body>, Infallible> {
    let key = format!("{}-{}", req.uri().path(), req.method().as_str());
    if let Some(handle) = router.routers.get(&key) {
        let mut context = AppContext {
            response: Response::new(Body::from("hello,world")),
        };

        (handle)(&mut context);
        Ok(context.response)
    } else {
        Ok(Response::new(Body::from("404 not found")))
    }
}

pub async fn run(addr: SocketAddr, router: Router) {
    //接受路由
    let router: Arc<Router> = Arc::new(router);

    let make_service = make_service_fn(move |_conn| {
        let router = router.clone();
        let service = service_fn(move |req| handler(req, router.clone()));
        async move { Ok::<_, Infallible>(service) }
    });

    println!("Listening on http://{}", addr);
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
