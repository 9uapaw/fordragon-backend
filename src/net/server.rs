use crate::error::error::AuthError;
use crate::error::error::Error;
use crate::game::lobby::Lobby;
use crate::net::connection::DataStreamConnection;
use crate::net::protocol::encode::BBEncodable;
use crate::user::auth::AuthPackage;
use crate::user::session::DefaultSessionManager;
use crate::user::user::AuthenticatedUser;
use bytes::{BufMut, BytesMut};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::prelude::*;

pub struct TcpServer {
    address: SocketAddr,
    game_loop: Arc<Mutex<Lobby>>,
}

impl TcpServer {
    pub fn new(address: SocketAddr) -> Self {
        TcpServer {
            address,
            game_loop: Arc::new(Mutex::new(Lobby::new())),
        }
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        let client = TcpListener::bind(&self.address).await.map_err(|e| {
            Error::new_network(&format!("Unable to start server {}", e.to_string()))
        })?;
        let session_manager = Arc::new(Mutex::new(DefaultSessionManager::new()));
        self.game_loop.lock().unwrap().start();

        loop {
            let (mut socket, addr) = client.accept().await.map_err(|e| {
                Error::new_network(&format!(
                    "Unable to accept incoming connection {}",
                    e.to_string()
                ))
            })?;
            let game = self.game_loop.clone();

            // Disable Natle algorithm
            socket.set_nodelay(true);
            info!("Accepted connection from {}", addr);

            let manager = session_manager.clone();
            tokio::spawn(async move {
                let mut connection = DataStreamConnection::new(socket, addr.clone());
                let auth_res = connection.authenticate(manager).await;
                let writer = connection.spawn_writer().await;
                let reader = connection.spawn_reader().await;
                debug!("Spawned writer");
                if let Err(e) = writer {
                    error!("Unable to acquire writer: {}", e.to_string());
                    return;
                }

                let mut writer = writer.expect("Unexpected error when unwrapping Writer");
                let mut reader = reader.expect("Unexpected error when unwrapping Reader");

                match auth_res {
                    Err(e) => {
                        error!("Authentication error: {}", e.to_string());
                        let error = match e {
                            Error::AuthError(auth_error) => AuthPackage::from(&auth_error),
                            _ => AuthPackage::AUTH_NETWORK_ERR,
                        };
                        warn!("Authentication failed for {}", addr);
                        let mut b = BytesMut::new();
                        error.encode_as_bbp(&mut b);
                        writer.send(b.freeze());
                        return;
                    }
                    Ok(user_name) => {
                        info!("Authenticated {}", addr);
                        let mut b = BytesMut::new();
                        AuthPackage::AUTH_OK.encode_as_bbp(&mut b);
                        let res = writer.send(b.freeze());
                        if let Err(e) = res {
                            error!("Error while sending authentication OK: {}", e.to_string());
                            return;
                        }
                        let user =
                            AuthenticatedUser::new(addr, user_name, Some(reader), Some(writer));
                        if let Ok(mut game) = game.lock() {
                            game.enter_user(user);
                        }
                    }
                }
            });
        }
    }
}
