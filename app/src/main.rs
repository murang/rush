mod nice;
mod net;

use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{lookup_host, TcpListener};
use tokio::runtime::Runtime;
use rush::{app::App};


fn main() {
    let mut app = App::new();
    app.add_component(nice::Nice::new());
    app.add_component(net::TcpServer::new(10086));
    app.on_exit(||{
       println!("lets exit"); 
    });
    app.start_run(1);
}
