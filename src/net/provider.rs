use crate::error::error::Error;
use crate::net::data::IntermediateGamePacket;
use crate::net::protocol::decode::ByteToRawDecoder;
use crate::net::protocol::encode::BBEncodable;
use bytes::{Bytes, BytesMut};
use crossbeam_channel::{Receiver, RecvError, SendError, Sender};
use std::future::Future;

type Rec = Receiver<Bytes>;
type Send = Sender<Bytes>;
type InternalMsg = Bytes;

pub struct DataStreamReader {
    receiver: Rec,
    decoder: ByteToRawDecoder,
}

impl DataStreamReader {
    pub fn new(receiver: Rec) -> Self {
        DataStreamReader {
            receiver,
            decoder: ByteToRawDecoder::new(),
        }
    }
}

impl DataStreamReader {
    pub fn recv(&mut self) -> Result<IntermediateGamePacket, Error> {
        let bytes = BytesMut::from(
            self.receiver
                .recv()
                .map_err(|e| Error::new_network(&e.to_string()))?.as_ref(),
        );
        self.decoder.convert(&bytes)
    }

    pub fn try_recv(&mut self) -> Result<IntermediateGamePacket, Error> {
        let bytes = BytesMut::from(
            self.receiver
                .try_recv()
                .map_err(|e| Error::new_network(&e.to_string()))?.as_ref(),
        );
        self.decoder.convert(&bytes)
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
    pub fn send(&mut self, data: Bytes) -> Result<(), SendError<Bytes>> {
        self.sender.send(data)
    }
}
