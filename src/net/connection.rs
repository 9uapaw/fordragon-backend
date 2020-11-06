use crate::error::error::Error;
use crate::net::converter::ByteToRawConverter;
use crate::net::data::RawInternalData;
use crate::net::provider::{DataProvider, DataStream};
use bytes::BytesMut;
use std::future::Future;
use std::net::SocketAddr;
use std::process::Output;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::sync::mpsc;

const BUFFER_LIMIT: usize = 1400;

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
}

impl DataStreamConnection {
    async fn spawn_reader(&mut self) -> Result<DataStream, Error> {
        let (tx, mut rx) = mpsc::channel(100);

        if let Some(mut socket) = self.socket.take() {
            tokio::spawn(async move {
                let converter = ByteToRawConverter::new();
                let mut buf = BytesMut::with_capacity(BUFFER_LIMIT);
                loop {
                    match socket.read(buf.as_mut()).await {
                        Ok(n) if n == 0 => return,
                        Ok(n) => {
                            tx.send(converter.convert(&buf)).await;
                        }
                        Err(e) => {
                            eprintln!("Error while reading socket");
                            return;
                        }
                    }
                }
            }).await;
        } else {
            return Err(Error::NetworkError("No socket connection".to_string()));
        }

        Ok(DataStream::new(rx))
    }
}
