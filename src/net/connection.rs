use crate::error::error::{AuthError, Error};
use crate::net::data::RawInternalData;
use crate::net::protocol::decode::ByteToRawDecoder;
use crate::net::protocol::opcode::NetworkRecvOpCode;
use crate::net::provider::{DataStreamReader, DataStreamWriter};
use crate::user::session::UserSessionManager;
use bytes::{BytesMut, Bytes};
use std::future::Future;
use std::net::SocketAddr;
use std::process::Output;
use std::sync::{Arc, Mutex};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::sync::mpsc;

const BUFFER_LIMIT: usize = 1400;

type SafeManager<T> = Arc<Mutex<T>>;

/// Represents an open connection to a client
pub struct DataStreamConnection {
    socket: Option<TcpStream>,
    reader: Option<OwnedReadHalf>,
    writer: Option<OwnedWriteHalf>,
    address: SocketAddr,
    authentication: Option<String>,
}

impl DataStreamConnection {
    pub fn new(socket: TcpStream, address: SocketAddr) -> Self {
        DataStreamConnection {
            socket: Some(socket),
            reader: None,
            writer: None,
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
            let mut buf = [0 as u8; BUFFER_LIMIT];
            let mut buf = BytesMut::from(buf.as_ref());
            let first = socket
                .read(buf.as_mut())
                .await
                .map_err(|e| Error::new_network(&e.to_string()))?;
            debug!("Authentication bytes: #{} {:?}", first, buf);

            let converter = ByteToRawDecoder::new();
            let auth = converter.convert(&buf);
            debug!("Converted {:?}", auth);

            match auth {
                Ok(auth) => match auth {
                    RawInternalData::AUTH { user, hash } => {
                        let manager = session_manager.lock();
                        if !manager.is_ok() {
                            return Err(AuthError::invalid_user_or_password());
                        }
                        return if manager
                            .unwrap()
                            .is_auth_registered(user.as_str(), hash.as_str())
                        {
                            self.authentication.replace(hash.clone());
                            debug!("Successfully authenticated {}", user);
                            Ok(())
                        } else {
                            debug!("Unable to authenticate via user session manager");
                            Err(Error::new_network("Authentication failed"))
                        };
                    }
                    _ => {
                        debug!("Packet is not AUTH");
                        return Err(Error::NetworkError("Authentication failed".to_string()));
                    }
                },
                Err(e) => {
                    debug!("{}", e.to_string());
                    return Err(Error::NetworkError("Authentication failed".to_string()));
                }
            };
            let (reader, writer) = socket.into_split();
            self.writer = Some(writer);
            self.reader = Some(reader);
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
    pub async fn spawn_reader(&mut self) -> Result<DataStreamReader, Error> {
        let (tx, mut rx) = mpsc::channel(100);

        if self.authentication.is_none() {
            return Err(Error::new_network("Client is not authenticated"));
        }

        if let Some(mut socket) = self.reader.take() {
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

        Ok(DataStreamReader::new(rx))
    }

    /// Spawns an asynchronous thread that is reading a channel and transmits the received
    /// data to the writing part of the socket.
    ///
    /// # Returns
    /// A stream, which wraps the sender part of a channel in an asynchronous fashion.
    pub async fn spawn_writer(&mut self) -> Result<DataStreamWriter, Error> {
        let (tx, mut rx):  (mpsc::Sender<Bytes>,  mpsc::Receiver<Bytes>) = mpsc::channel(100);
        let mut bytes: Bytes = Bytes::new();

        if self.authentication.is_none() {
            return Err(Error::new_network("Client is not authenticated"));
        }

        if let Some(mut writer) = self.writer.take() {
            tokio::spawn(async move {
                let converter = ByteToRawDecoder::new();

                loop {
                    match rx.recv().await {
                        Some(bytes) => {let write_res = writer.write_all(bytes.as_ref()).await;
                            if let Err(e) = write_res {
                                error!("{}", e.to_string());
                            }
                        },
                        None => ()
                    };
                }
            })
                .await;
        }

        Ok(DataStreamWriter::new(tx))
    }
}
