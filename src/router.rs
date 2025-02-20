use core::panic;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use crate::request::HttpRequest;
use crate::response::HttpResponse;

pub struct Router {
    routes: HashMap<String, Handler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn route(
        mut self,
        path: String,
        handler: Handler,
    ) -> Self
    {
        let res = self.routes.insert(path, handler);
        if res.is_some() {
            panic!("Routes must have unique path and method");
        }
        self
    }

    pub fn get_routes(&self) -> &HashMap<String, Handler> {
        &self.routes
    }
}

pub struct Handler {
    method: HttpMethod,
    func: Box<
        dyn Fn(HttpRequest) -> Pin<Box<dyn Future<Output = HttpResponse> + Send>> + Send + Sync,
    >,
}

impl Handler {

    pub fn get_method(&self) -> &HttpMethod {
        &self.method
    }

    pub async fn call(&self, req: HttpRequest) -> HttpResponse {
        (self.func)(req).await
    }
}

pub fn get<F, P>(raw_func: F) -> Handler
where
    F: Fn(HttpRequest) -> P + Send + Sync + 'static,
    P: Future<Output = HttpResponse> + Send + Sync + 'static,
{
    Handler {
        method: HttpMethod::Get,
        func: Box::new(move |req| Box::pin(raw_func(req))),
    }
}
pub fn post<F, P>(raw_func: F) -> Handler
where
    F: Fn(HttpRequest) -> P + Send + Sync + 'static,
    P: Future<Output = HttpResponse> + Send + 'static,
{
    Handler {
        method: HttpMethod::Post,
        func: Box::new(move |req| Box::pin(raw_func(req))),
    }
}
pub fn put<F, P>(raw_func: F) -> Handler
where
    F: Fn(HttpRequest) -> P + Send + Sync + 'static,
    P: Future<Output = HttpResponse> + Send + 'static,
{
    Handler {
        method: HttpMethod::Put,
        func: Box::new(move |req| Box::pin(raw_func(req))),
    }
}
pub fn delete<F, P>(raw_func: F) -> Handler
where
    F: Fn(HttpRequest) -> P + Send + Sync + 'static,
    P: Future<Output = HttpResponse> + Send + 'static,
{
    Handler {
        method: HttpMethod::Delete,
        func: Box::new(move |req| Box::pin(raw_func(req))),
    }
}
pub fn patch<F, P>(raw_func: F) -> Handler
where
    F: Fn(HttpRequest) -> P + Send + Sync + 'static,
    P: Future<Output = HttpResponse> + Send + 'static,
{
    Handler {
        method: HttpMethod::Patch,
        func: Box::new(move |req| Box::pin(raw_func(req))),
    }
}
pub fn head<F, P>(raw_func: F) -> Handler
where
    F: Fn(HttpRequest) -> P + Send + Sync + 'static,
    P: Future<Output = HttpResponse> + Send + 'static,
{
    Handler {
        method: HttpMethod::Head,
        func: Box::new(move |req| Box::pin(raw_func(req))),
    }
}
pub fn options<F, P>(raw_func: F) -> Handler
where
    F: Fn(HttpRequest) -> P + Send + Sync + 'static,
    P: Future<Output = HttpResponse> + Send + 'static,
{
    Handler {
        method: HttpMethod::Options,
        func: Box::new(move |req| Box::pin(raw_func(req))),
    }
}
pub fn trace<F, P>(raw_func: F) -> Handler
where
    F: Fn(HttpRequest) -> P + Send + Sync + 'static,
    P: Future<Output = HttpResponse> + Send + 'static,
{
    Handler {
        method: HttpMethod::Trace,
        func: Box::new(move |req| Box::pin(raw_func(req))),
    }
}
pub fn connect<F, P>(raw_func: F) -> Handler
where
    F: Fn(HttpRequest) -> P + Send + Sync + 'static,
    P: Future<Output = HttpResponse> + Send + 'static,
{
    Handler {
        method: HttpMethod::Connect,
        func: Box::new(move |req| Box::pin(raw_func(req))),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Trace,
    Connect,
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            "PUT" => HttpMethod::Put,
            "DELETE" => HttpMethod::Delete,
            "PATH" => HttpMethod::Patch,
            "HEAD" => HttpMethod::Head,
            "OPTIONS" => HttpMethod::Options,
            "TRACE" => HttpMethod::Trace,
            "CONNECT" => HttpMethod::Connect,
            _ => panic!("Non standart http method"),
        }
    }
}
