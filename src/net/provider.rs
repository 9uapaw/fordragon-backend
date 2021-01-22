use crate::error::error::Error;
use crate::net::data::IntermediateGameData;
use bytes::Bytes;
use std::future::Future;
use tokio::sync::mpsc;
use crate::net::protocol::encode::BBEncodable;

type Rec = mpsc::Receiver<Bytes>;
type Send = mpsc::UnboundedSender<Bytes>;
type InternalMsg = Bytes;

pub struct DataStreamReader {
    receiver: Rec,
}

impl DataStreamReader {
    pub fn new(receiver: Rec) -> Self {
        DataStreamReader { receiver }
    }
}

impl DataStreamReader {
    pub fn recv(&mut self) -> impl Future<Output = Option<InternalMsg>> + '_ {
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
    pub fn send(
        &mut self,
        data: Bytes,
    ) -> Result<(), mpsc::error::SendError<Bytes>> {
        self.sender.send(data)
    }
}
