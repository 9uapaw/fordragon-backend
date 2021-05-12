use crate::error::error::{AuthError, Error};
use crate::net::data::IntermediateGamePacket;
use crate::net::protocol::decode::ByteToRawDecoder;
use crate::net::protocol::opcode::NetworkRecvOpCode;
use crate::net::provider::{DataStreamReader, DataStreamWriter};
use crate::user::session::UserSessionManager;
use crate::user::user_event::UserChangeEvent;
use bytes::{Bytes, BytesMut};
use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use std::future::Future;
use std::net::SocketAddr;
use std::process::Output;
use std::sync::{Arc, Mutex};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::prelude::*;

const BUFFER_LIMIT: usize = 1400;

type SafeManager<T> = Arc<Mutex<T>>;

/// Represents an open connection to a client. In order to receive additional messages from client,
/// the first package must be a valid authentication package.
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
    ) -> Result<String, Error> {
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

            return match auth {
                Ok(auth) => match auth {
                    IntermediateGamePacket::Auth { user, hash } => {
                        let manager = session_manager.lock();
                        if !manager.is_ok() {
                            return Err(AuthError::invalid_user_or_password());
                        }
                        if manager
                            .unwrap()
                            .is_auth_registered(user.as_str(), hash.as_str())
                        {
                            self.authentication.replace(hash.clone());
                            debug!("Successfully authenticated {}", user);
                            let (reader, writer) = socket.into_split();
                            self.writer = Some(writer);
                            self.reader = Some(reader);
                            Ok((user.clone()))
                        } else {
                            debug!("Unable to authenticate via user session manager");
                            Err(Error::new_network("Authentication failed"))
                        }
                    }
                    _ => {
                        debug!("Packet is not AUTH");
                        Err(Error::NetworkError(
                            "Authentication failed due to invalid packet type".to_string(),
                        ))
                    }
                },
                Err(e) => {
                    debug!("{}", e.to_string());
                    Err(Error::NetworkError(format!(
                        "Authentication failed due to conversion error - {}",
                        e.to_string()
                    )))
                }
            };
        } else {
            return Err(Error::new_network("No session is stored"));
        }

        Err(AuthError::invalid_user_or_password())
    }

    /// Spawns an asynchronous thread that is reading the output of the socket connection
    ///
    /// # Returns
    /// A stream, which wraps the receiving part of a channel in an asynchronous fashion.
    pub async fn spawn_reader(&mut self) -> Result<DataStreamReader, Error> {
        let (tx, mut rx) = unbounded();

        if self.authentication.is_none() {
            return Err(Error::new_network("Client is not authenticated"));
        }

        if let Some(mut socket) = self.reader.take() {
            tokio::spawn(async move {
                loop {
                    let mut buf = [0 as u8; BUFFER_LIMIT];
                    match socket.read(buf.as_mut()).await {
                        Ok(n) if n == 0 => (),
                        Ok(n) => {
                            debug!("Received data {}", n);
                            tx.send(Bytes::copy_from_slice(&buf));
                        }
                        Err(e) => {
                            eprintln!("Error while reading socket");
                        }
                    }
                }
            });
        } else {
            return Err(Error::NetworkError(
                "No reader session is stored".to_string(),
            ));
        }

        Ok(DataStreamReader::new(rx))
    }

    /// Spawns an asynchronous thread that is reading a channel and transmits the received
    /// data to the writing part of the socket.
    ///
    /// # Returns
    /// A stream, which wraps the sender part of a channel in an asynchronous fashion.
    pub async fn spawn_writer(
        &mut self,
        user_change: Sender<UserChangeEvent>,
    ) -> Result<DataStreamWriter, Error> {
        let (tx, mut rx): (Sender<Bytes>, Receiver<Bytes>) = unbounded();

        if self.authentication.is_none() {
            return Err(Error::new_network("Client is not authenticated"));
        }

        let addr = self.address.clone();

        if let Some(mut writer) = self.writer.take() {
            tokio::spawn(async move {
                loop {
                    match rx.recv() {
                        Ok(bytes) => {
                            debug!("Received data to be transmitted");
                            let write_res = writer.write_all(bytes.as_ref()).await;
                            writer.flush().await;
                            debug!("Sent data to {:?}", writer);
                            if let Err(e) = write_res {
                                error!("Error writing data: {}", e.to_string());
                                user_change.send(UserChangeEvent::DisconnectedUser(addr));
                            }
                        }
                        Err(e) => {
                            // error!("Error receiving writable data: {}", e.to_string());
                        }
                    };
                }
            });
        } else {
            return Err(Error::NetworkError(
                "No writer session is stored".to_string(),
            ));
        }

        Ok(DataStreamWriter::new(tx))
    }
}

#[cfg(test)]
mod tests {
    use crate::net::connection::DataStreamConnection;
    use crate::net::protocol::encode::{BBEncodable, ByteEncoder};
    use crate::net::protocol::opcode::NetworkRecvOpCode;
    use crate::user::session::UserSessionManager;
    use bytes::{Buf, Bytes, BytesMut};
    use env_logger::Env;
    use std::sync::{Arc, Mutex};
    use tokio::net::TcpListener;
    use tokio::net::TcpStream;
    use tokio::prelude::*;

    #[tokio::test]
    async fn test_bidirectional_messaging() {
        let env = Env::default().filter_or("BB_LOG_LEVEL", "debug");
        env_logger::init_from_env(env);

        let client = TcpListener::bind("localhost:44444")
            .await
            .expect("Can not start server on localhost");
        let test_msg = "1";

        tokio::spawn(async move {
            let test_msg = test_msg.to_string();
            let mut tcp_stream = TcpStream::connect("localhost:44444")
                .await
                .expect("Can not connect to localhost");
            let mut auth = BytesMut::new();
            let mut encoder = ByteEncoder::new(&mut auth);
            encoder.encode(&NetworkRecvOpCode::AUTH);
            encoder.encode_str("test");
            encoder.encode_str("test:test:test");
            drop(encoder);
            tcp_stream.write_all(auth.as_ref()).await;
            tcp_stream.flush().await;

            let mut buf = [0 as u8; 1024];
            match tcp_stream.read(buf.as_mut()).await {
                Ok(n) if n == 0 => {
                    println!("0");
                }
                Ok(n) => {
                    println!("{:?}", buf);
                    return;
                }
                Err(e) => {
                    eprintln!("Error while reading socket");
                }
            }
            if !buf.is_empty() {
                println!("buff");
                let res = tcp_stream.write(buf.as_ref()).await.expect("Error sending");
                tcp_stream.flush().await;
            }
        });

        let (mut socket, addr) = client.accept().await.expect("Socket");
        let mut connection = DataStreamConnection::new(socket, addr);
        let authentication = connection
            .authenticate(Arc::new(Mutex::new(FakeManager {})))
            .await;
        authentication.expect("Authentication error");
        let mut writer = connection.spawn_writer().await.expect("Writer error");
        let mut reader = connection.spawn_reader().await.expect("Reader error");
        writer.send(BytesMut::from(test_msg.as_bytes()).freeze());
    }

    struct FakeManager {}

    impl UserSessionManager for FakeManager {
        fn is_auth_registered(&self, user: &str, auth: &str) -> bool {
            return true;
        }
    }
}
