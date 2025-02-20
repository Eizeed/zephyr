use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use crate::request::HttpRequest;

pub struct HttpResponse;
pub trait IntoResponse {}

pub struct Router {
    routes: HashMap<String, Handler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn route<P>(mut self, path: String, f: fn(req: HttpRequest) -> P) -> Self
    where
        P: Future<Output = HttpResponse> + Send + Sync + 'static,
    {
        let handler = Handler::new(f);
        self.routes.insert(path, handler);
        self
    }

    pub fn get_routes(&self) -> &HashMap<String, Handler> {
        &self.routes
    }
}

pub struct Handler {
    func: Box<
        dyn Fn(HttpRequest) -> Pin<Box<dyn Future<Output = HttpResponse> + Send>> + Send + Sync,
    >,
}

impl Handler {
    pub fn new<P>(raw_func: fn(req: HttpRequest) -> P) -> Self
    where
        P: Future<Output = HttpResponse> + Send + Sync + 'static,
    {
        Handler {
            func: Box::new(move |req| Box::pin(raw_func(req))),
        }
    }

    pub async fn call(&self, req: HttpRequest) -> HttpResponse {
        (self.func)(req).await
    }
}
