// Copyright 2019 Mats Kindahl
//
// Licensed under the Apache License, Version 2.0 (the "License"); you
// may not use this file except in compliance with the License.  You
// may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied.  See the License for the specific language governing
// permissions and limitations under the License.

use crate::devices::DeviceUpdate;
use crate::state::State;
use crate::view::ViewUpdate;
use bytes::BytesMut;
use serde_cbor::{from_slice, to_vec};
use std::net::SocketAddr;
use tokio::codec::{Decoder, Encoder};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Gossip {
    DebugMessage { text: String },
    DeviceGossip(DeviceUpdate),
    ViewGossip(ViewUpdate),
}

impl Gossip {
    pub fn update_state(
        &self,
        state: &mut State,
        sender: &Uuid,
        timestamp_millis: i64,
        peer: &SocketAddr,
    ) {
        match self {
            Gossip::DebugMessage { text } => info!("From {}  {}", peer, text),

            Gossip::DeviceGossip(device_gossip) => {
                state.update_devices(device_gossip, sender, timestamp_millis)
            }

            Gossip::ViewGossip(view_gossip) => {
                state.update_view(view_gossip, sender, timestamp_millis)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub sender: Uuid,
    pub timestamp_millis: i64,
    pub hops: u32,
    pub payload: Option<Gossip>,
}

impl Message {
    pub fn update_state(&self, state: &mut State, peer: &SocketAddr) {
        if let Some(ref gossip) = self.payload {
            gossip.update_state(state, &self.sender, self.timestamp_millis, peer)
        }
    }
}

pub struct GossipCodec;

impl GossipCodec {
    pub fn new() -> GossipCodec {
        GossipCodec {}
    }
}

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
