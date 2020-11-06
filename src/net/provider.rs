use crate::error::error::Error;
use crate::net::data::RawInternalData;
use tokio::sync::mpsc;
use std::future::Future;

pub trait DataProvider {
    fn spawn_reader(&mut self) -> Result<DataStream, Error>;
}

pub struct DataStream {
    receiver: mpsc::Receiver<RawInternalData>,
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
