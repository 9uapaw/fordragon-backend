use tokio::prelude::*;
use tokio::net::TcpListener;

const LOCALHOST: &str = "127.0.0.1";
const PORT: &str = "491337";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", LOCALHOST, PORT);
    let client = TcpListener::bind(&addr).await?;

    Ok(())
}
