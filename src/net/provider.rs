use crate::error::error::Error;
use crate::net::data::RawInternalData;
use std::future::Future;
use tokio::sync::mpsc;

type Rec = mpsc::Receiver<Result<RawInternalData, Error>>;
type Writer = mpsc::Sender<>
type InternalMsg = Result<RawInternalData, Error>;

pub trait DataProvider {
    fn spawn_reader(&mut self) -> Result<DataStream, Error>;
}

pub struct DataStream {
    receiver: Rec,
}

impl DataStream {
    pub fn new(receiver: Rec) -> Self {
        DataStream { receiver }
    }
}

impl DataStream {
    pub async fn recv(
        &mut self,
    ) -> impl Future<Output = Option<InternalMsg>> + '_ {
        self.receiver.recv()
    }
}
