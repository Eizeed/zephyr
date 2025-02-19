use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub struct Server {
    ip_addr: String,
    port: u16,
}

pub struct ServerBuilder {
    ip_addr: String,
    port: u16,
}

impl Server {
    pub fn new(ip_addr: String, port: u16) -> Server {
        Server { ip_addr, port }
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


                        let body = String::from_utf8_lossy(&buf[0..n]);

                        socket
                            .write_all(
                                format!(
                                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                                    body.len(),
                                    body
                                )
                                .as_bytes(),
                            )
                            .await
                            .expect("failed to write data to socket");
                    });
                }
            })
    }
}
