use crate::error::error::Error;
use crate::net::data::RawInternalData;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use std::process::Output;
use std::future::Future;

pub trait DataProvider {
    fn spawn_reader() -> Result<DataStream, Error>;
}

pub struct DataStream {
    receiver: mpsc::Receiver<RawInternalData>
}

impl DataStream {
    pub fn new(receiver: mpsc::Receiver<RawInternalData>) -> Self {
        DataStream { receiver }
    }
}

impl DataStream {
    pub async fn recv(&mut self) -> impl Future<Output = Option<RawInternalData>> + '_ {
        self.receiver.recv()
    }
}

pub struct DataStreamConnection {
    socket: TcpListener,
    address: SocketAddr,
    authentication: Option<String>,
}

impl DataStreamConnection {
    pub fn new(socket: TcpListener, address: SocketAddr) -> Self {
        DataStreamConnection {
            socket,
            address,
            authentication: None,
        }
    }
}

impl DataProvider for DataStreamConnection {
    fn spawn_reader() -> Result<DataStream, Error> {
       let (tx, mut rx) = mpsc::channel(100);

        Ok(DataStream::new(rx))
    }
}
