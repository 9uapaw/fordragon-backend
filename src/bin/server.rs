use fordragon_backend::net::connection::DataStreamConnection;
use fordragon_backend::user::session::DefaultSessionManager;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::prelude::*;

const LOCALHOST: &str = "127.0.0.1";
const PORT: &str = "47331";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", LOCALHOST, PORT).parse::<SocketAddr>()?;
    let client = TcpListener::bind(&addr).await?;
    let session_manager = Arc::new(Mutex::new(DefaultSessionManager::new()));

    loop {
        let (mut socket, addr) = client.accept().await?;
        let manager = session_manager.clone();

        tokio::spawn(async move {
            let mut connection = DataStreamConnection::new(Some(socket), addr);
            let auth_res = connection.authenticate(manager).await;
            println!("Auth is {}", auth_res.is_ok());
        });
    }

    Ok(())
}
