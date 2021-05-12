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
use crossbeam_channel::unbounded;
use crate::user::user_event::UserChangeEvent;

pub struct TcpServer {
    address: SocketAddr,
}

impl TcpServer {
    pub fn new(address: SocketAddr) -> Self {
        TcpServer {
            address,
        }
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        let client = TcpListener::bind(&self.address).await.map_err(|e| {
            Error::new_network(&format!("Unable to start server {}", e.to_string()))
        })?;
        let session_manager = Arc::new(Mutex::new(DefaultSessionManager::new()));
        let (user_change_send, user_change_recv) = unbounded();

        tokio::spawn(async move {
            let mut lobby = Lobby::new(user_change_recv);
            lobby.start();
        });

        loop {
            let new_user_send = user_change_send.clone();
            let (mut socket, addr) = client.accept().await.map_err(|e| {
                Error::new_network(&format!(
                    "Unable to accept incoming connection {}",
                    e.to_string()
                ))
            })?;

            // Disable Natle algorithm
            socket.set_nodelay(true);
            info!("Accepted connection from {}", addr);

            let manager = session_manager.clone();
            tokio::spawn(async move {
                let mut connection = DataStreamConnection::new(socket, addr.clone());
                let auth_res = connection.authenticate(manager).await;
                let writer = connection.spawn_writer(new_user_send.clone()).await;
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
                        debug!("Sent new user {} to lobby", &user);
                        new_user_send.send(UserChangeEvent::NewUser(user));
                    }
                }
            });
        }
    }
}
