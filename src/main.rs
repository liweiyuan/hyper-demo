use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use std::collections::HashMap;
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};

struct AppContext {
    pub response: Response<Body>,
}

//请求处理函数
type Handler = dyn Fn(&mut AppContext) + Send + Sync + 'static;

//路由
pub struct Router {
    routers: HashMap<String, Box<Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routers: HashMap::new(),
        }
    }

    fn add_route<F>(mut self, path: &str, method: Method, handler: F) -> Self
    where
        F: Fn(&mut AppContext) + Send + Sync + 'static,
    {
        let key = format!("{}-{}", path, method);
        self.routers.insert(key, Box::new(handler));
        self
    }

    fn get<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(&mut AppContext) + Send + Sync + 'static,
    {
        self.add_route(path, Method::GET, handler)
    }

    fn post<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(&mut AppContext) + Send + Sync + 'static,
    {
        self.add_route(path, Method::POST, handler)
    }

    fn delete<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(&mut AppContext) + Send + Sync + 'static,
    {
        self.add_route(path, Method::DELETE, handler)
    }

    fn put<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(&mut AppContext) + Send + Sync + 'static,
    {
        self.add_route(path, Method::PUT, handler)
    }

    fn patch<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(&mut AppContext) + Send + Sync + 'static,
    {
        self.add_route(path, Method::PATCH, handler)
    }
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let handle_hello = |c: &mut AppContext| {
        c.response = Response::new(Body::from("handle_hello"));
        println!("Hello, from {:#?}", c.response);
    };

    let router: Arc<Router> = Arc::new(Router::new().get("/index", handle_hello));

    let make_service = make_service_fn(move |_conn| {
        let router = router.clone();
        let service = service_fn(move |req| handler(req, router.clone()));
        async move { Ok::<_, Infallible>(service) }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)).into();

    println!("Listening on http://{}", addr);
    let server = Server::bind(&addr).serve(make_service);

    server.await?;
    Ok(())
}
