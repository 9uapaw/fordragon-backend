use crate::error::error::{AuthError, Error};
use crate::net::data::RawInternalData;
use crate::net::protocol::decode::ByteToRawDecoder;
use crate::net::protocol::opcode::NetworkRecvOpCode;
use crate::net::provider::{DataProvider, DataStream};
use crate::user::session::UserSessionManager;
use bytes::BytesMut;
use std::future::Future;
use std::net::SocketAddr;
use std::process::Output;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::sync::mpsc;

const BUFFER_LIMIT: usize = 1400;

type SafeManager<T> = Arc<Mutex<T>>;

/// Represents an open connection to a client
pub struct DataStreamConnection {
    socket: Option<TcpStream>,
    address: SocketAddr,
    authentication: Option<String>,
}

impl DataStreamConnection {
    pub fn new(socket: Option<TcpStream>, address: SocketAddr) -> Self {
        DataStreamConnection {
            socket,
            address,
            authentication: None,
        }
    }

    /// Checks if the client has authorized themselves.
    pub fn is_authenticated(&self) -> bool {
        self.authentication.is_some()
    }
}

impl DataStreamConnection {
    /// Waits for an authentication packet and validates it. Must be done before receiving
    /// any other packet, otherwise, the session is dropped and closed.
    pub async fn authenticate<T: UserSessionManager>(
        &mut self,
        session_manager: SafeManager<T>,
    ) -> Result<(), Error> {
        if let Some(mut socket) = self.socket.take() {
            let mut buf = BytesMut::with_capacity(BUFFER_LIMIT);
            let first = socket.read(buf.as_mut()).await;
            let converter = ByteToRawDecoder::new();
            let auth = converter.convert(&buf);

            match auth {
                Ok(auth) => match auth {
                    RawInternalData::AUTH { user, hash } => {
                        let manager = session_manager.lock();
                        if !manager.is_ok() {
                            return Err(AuthError::invalid_user_or_password());
                        }
                        if session_manager
                            .lock()
                            .unwrap()
                            .is_auth_registered(user.as_str(), hash.as_str())
                        {
                            self.authentication.replace(hash.clone());
                        } else {
                            return Err(Error::new_network("Authentication failed"));
                        }
                    }
                    _ => {
                        return Err(Error::NetworkError("Authentication failed".to_string()));
                    }
                },
                _ => {
                    return Err(Error::NetworkError("Authentication failed".to_string()));
                }
            };
        } else {
            return Err(Error::new_network("No session is stored"));
        }

        Ok(())
    }

    /// Spawns an asynchronous thread that is reading the output of the socket connection
    /// and converts the raw bytes to [RawInternalData][crate::net::data::RawInternalData].
    ///
    /// # Returns
    /// A stream, which wraps the receiving part of a channel in an asynchronous fashion.
    pub async fn spawn_reader(&mut self) -> Result<DataStream, Error> {
        let (tx, mut rx) = mpsc::channel(100);

        if self.authentication.is_none() {
            return Err(Error::new_network("Client is not authenticated"));
        }

        if let Some(mut socket) = self.socket.take() {
            tokio::spawn(async move {
                let converter = ByteToRawDecoder::new();

                loop {
                    let mut buf = BytesMut::with_capacity(BUFFER_LIMIT);
                    match socket.read(buf.as_mut()).await {
                        Ok(n) if n == 0 => (),
                        Ok(n) => {
                            tx.send(converter.convert(&buf)).await;
                        }
                        Err(e) => {
                            eprintln!("Error while reading socket");
                        }
                    }
                }
            })
            .await;
        } else {
            return Err(Error::NetworkError("No session is stored".to_string()));
        }

        Ok(DataStream::new(rx))
    }
}
