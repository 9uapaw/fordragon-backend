use crate::error::error::Error;
use crate::net::data::RawInternalData;
use bytes::Bytes;
use std::future::Future;
use tokio::sync::mpsc;
use crate::net::protocol::encode::BBEncodable;

type Rec = mpsc::Receiver<Result<RawInternalData, Error>>;
type Send = mpsc::Sender<Bytes>;
type InternalMsg = Result<RawInternalData, Error>;

pub struct DataStreamReader {
    receiver: Rec,
}

impl DataStreamReader {
    pub fn new(receiver: Rec) -> Self {
        DataStreamReader { receiver }
    }
}

impl DataStreamReader {
    pub async fn recv(&mut self) -> impl Future<Output = Option<InternalMsg>> + '_ {
        self.receiver.recv()
    }
}

pub struct DataStreamWriter {
    sender: Send,
}

impl DataStreamWriter {
    pub fn new(sender: Send) -> Self {
        DataStreamWriter { sender }
    }
}

impl DataStreamWriter {
    pub async fn send<T>(
        &mut self,
        data: T,
    ) -> impl Future<Output = Result<(), mpsc::error::SendError<Bytes>>> + '_  where T: BBEncodable{
        self.sender.send(data.encode_as_bbp())
    }
}
