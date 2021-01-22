use env_logger::Env;
use fordragon_backend::error::error::Error;
use fordragon_backend::net::connection::DataStreamConnection;
use fordragon_backend::net::server::TcpServer;
use fordragon_backend::user::session::DefaultSessionManager;
use log::{error, info, warn};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::prelude::*;

const LOCALHOST: &str = "127.0.0.1";
const PORT: &str = "47331";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = Env::default().filter_or("BB_LOG_LEVEL", "debug");
    env_logger::init_from_env(env);

    let addr = format!("{}:{}", LOCALHOST, PORT)
        .parse::<SocketAddr>()
        .map_err(|e| Error::new_network("Invalid local address"))?;
    info!("Started server on {}:{}", LOCALHOST, PORT);
    let mut server = TcpServer::new(addr);
    server.start().await?;
    Ok(())
}
