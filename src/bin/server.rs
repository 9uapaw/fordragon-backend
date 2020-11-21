use env_logger::Env;
use fordragon_backend::net::connection::DataStreamConnection;
use fordragon_backend::user::session::DefaultSessionManager;
use log::{error, info, warn};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::prelude::*;

const LOCALHOST: &str = "127.0.0.1";
const PORT: &str = "47331";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default().filter_or("BB_LOG_LEVEL", "debug");
    env_logger::init_from_env(env);

    let addr = format!("{}:{}", LOCALHOST, PORT).parse::<SocketAddr>()?;
    let client = TcpListener::bind(&addr).await?;
    info!("Started server on {}:{}", LOCALHOST, PORT);

    let session_manager = Arc::new(Mutex::new(DefaultSessionManager::new()));

    loop {
        let (mut socket, addr) = client.accept().await?;
        info!("Accepted connection from {}", addr);

        let manager = session_manager.clone();
        tokio::spawn(async move {
            let mut connection = DataStreamConnection::new(socket, addr.clone());
            let auth_res = connection.authenticate(manager).await;
            if let Err(e) = auth_res {
                error!("{}", e.to_string());
                warn!("Authentication failed for {}", addr);
            } else {
                info!("Authenticated {}", addr);
            }
        });
    }


    Ok(())
}
