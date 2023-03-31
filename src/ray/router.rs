use std::collections::HashMap;

use hyper::{Response, Body, Method};

pub struct AppContext {
    pub response: Response<Body>,
}

//请求处理函数
type Handler = dyn Fn(&mut AppContext) + Send + Sync + 'static;

//路由
pub struct Router {
    pub routers: HashMap<String, Box<Handler>>,
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

    pub fn get<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(&mut AppContext) + Send + Sync + 'static,
    {
        self.add_route(path, Method::GET, handler)
    }

    pub fn post<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(&mut AppContext) + Send + Sync + 'static,
    {
        self.add_route(path, Method::POST, handler)
    }

    pub fn delete<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(&mut AppContext) + Send + Sync + 'static,
    {
        self.add_route(path, Method::DELETE, handler)
    }

    pub fn put<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(&mut AppContext) + Send + Sync + 'static,
    {
        self.add_route(path, Method::PUT, handler)
    }

    pub fn patch<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(&mut AppContext) + Send + Sync + 'static,
    {
        self.add_route(path, Method::PATCH, handler)
    }
}
