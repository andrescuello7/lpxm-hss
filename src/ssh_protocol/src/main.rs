mod server;
use crate::server::Server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let serv = Server {
        host: "0.0.0.0",
        port: "22",
        listener: None,
        channels: None,
    };
    let _ = serv.init().await.expect("ERROR creation server");
    Ok(())
}