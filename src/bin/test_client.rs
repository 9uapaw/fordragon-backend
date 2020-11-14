use tokio::prelude::*;
use tokio::net::TcpStream;
use fordragon_backend::user::session::DefaultSessionManager;
use std::net::SocketAddr;

const LOCALHOST: &str = "127.0.0.1";
const PORT: &str = "47331";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", LOCALHOST, PORT).parse::<SocketAddr>()?;
    let stream = TcpStream::connect(&addr).await?;

    Ok(())
}
