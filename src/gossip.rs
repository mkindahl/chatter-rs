use bytes::BytesMut;
use serde_cbor::{from_slice, to_vec};
use tokio_io::_tokio_codec::{Decoder, Encoder};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum Gossip {
    // Debug message
    Debug { text: String },

    // Status information for a device on a machine
    Status {},
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub server: Uuid,
    pub timestamp: u128,
    pub payload: Option<Gossip>,
}

pub struct GossipCodec;

impl Encoder for GossipCodec {
    type Item = Message;
    type Error = std::io::Error;

    fn encode(&mut self, item: Self::Item, buf: &mut BytesMut) -> std::io::Result<()> {
        let bytes = to_vec(&item).unwrap();
        buf.extend_from_slice(&bytes);
        Ok(())
    }
}

impl Decoder for GossipCodec {
    type Item = Message;
    type Error = std::io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> std::io::Result<Option<Self::Item>> {
        Ok(from_slice(&buf).unwrap())
    }
}
