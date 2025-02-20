use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::request::HttpRequest;
use crate::router::Router;

pub struct Server {
    ip_addr: String,
    port: u16,
    pub router: Arc<Router>,
}

impl Server {
    pub fn new(ip_addr: String, port: u16, router: Router) -> Server {
        let router = Arc::new(router);
        Server {
            ip_addr,
            port,
            router,
        }
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let addr = format!("{}:{}", self.ip_addr, self.port);
                let listener = TcpListener::bind(addr).await?;

                loop {
                    let routes = self.router.clone();
                    let (mut socket, _) = listener.accept().await?;

                    tokio::spawn(async move {
                        let mut buf = vec![0; 1024];

                        let n = socket
                            .read(&mut buf)
                            .await
                            .expect("failed to read data from socket");
                        if n == 0 {
                            return;
                        }

                        let req = HttpRequest::from(&buf[0..n]);

                        let route = routes
                            .get_routes()
                            .iter()
                            .find(|(path, func)| *path == req.path());

                        if let Some((path, func)) = route {
                            func.call(req).await;
                            let res = format!("Func called on path {}", path);
                            socket
                                .write_all(
                                    format!(
                                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                                        res.len(),
                                        res
                                    )
                                    .as_bytes(),
                                )
                                .await
                                .expect("failed to write data to socket");
                        } else {
                            socket
                                .write_all(b"HTTP/1.1 404 NOT_FOUND\r\n")
                                .await
                                .expect("failed to write data to socket");
                        }
                    });
                }
            })
    }
}
