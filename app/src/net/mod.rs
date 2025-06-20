use std::any::Any;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener},
    runtime::Runtime,
};
use std::error::Error;
use std::sync::Arc;
use rush::app::Component;

pub struct TcpServer {
    port: u16,
    rt: Arc<Runtime>
}

impl TcpServer {
    pub fn new(port : u16) -> Self {
        Self {
            port,
            rt: Arc::new(Runtime::new().unwrap()),
        }
    }
}

impl Component for TcpServer {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        println!("init net");
        let rt = self.rt.clone();
        let port = self.port;

        rt.spawn(async move {
            let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)).await {
                Ok(l) => {
                    println!("Listening on port {}", port);
                    l
                },
                Err(e) => {
                    eprintln!("Failed to bind to port {}: {}", port, e);
                    return;
                }
            };
            loop {
                let (mut socket, _) = match listener.accept().await {
                    Ok((s, a)) => {
                        println!("Accepted connection from {}", a);
                        (s, a)
                    },
                    Err(e) => {
                        eprintln!("Accept error: {}", e);
                        continue;
                    }
                };

                tokio::spawn(async move {
                    let mut buf = [0; 1024];
                    loop{
                        match socket.read(&mut buf).await {
                            Ok(0) => {
                                eprintln!("Peer closed connection");
                                return;
                            },
                            Ok(n) => {
                                eprintln!("Read {} bytes: {}", n, String::from_utf8_lossy(&buf[0..n]));
                                if let Err(e) = socket.write_all(&buf[0..n]).await {
                                    eprintln!("Write error: {}", e);
                                }
                            },
                            Err(e) => eprintln!("Read error: {}", e),
                        }
                    }
                });
            }
        });

        Ok(())
    }

    fn run(&mut self) {
        
    }

    fn on_exit(&mut self) {
        
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}