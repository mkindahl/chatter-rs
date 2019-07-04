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

//! Command-line utility to inject gossip into the network.
//!
//! This utility can be used to inject messages into the gossip
//! network and do this by sending gossip to a server on the gossip
//! port.  Currently, it can only inject debug messages.

extern crate bytes;
extern crate chatter;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate serde_json;

use chatter::gossip::{Gossip, Message};
use chrono::Utc;
use std::env;
use std::io;
use std::io::{stdin, Read};
use std::net::{SocketAddr, UdpSocket};
use uuid::Uuid;

fn read_from_stdin() -> Result<String, io::Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    Ok(input)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let remote_addr: SocketAddr = env::args().nth(1).expect("missing address").parse()?;
    let input = env::args().nth(2).unwrap_or(read_from_stdin()?);
    let socket = {
        let local_addr: SocketAddr = if remote_addr.is_ipv4() {
            "0.0.0.0:0"
        } else {
            "[::]:0"
        }
        .parse()?;
        UdpSocket::bind(&local_addr)?
    };

    let json: Gossip = serde_json::from_str(&input)?;
    debug!("Saw JSON:\n{:#?}", json);
    let message = Message {
        timestamp_millis: Utc::now().timestamp_millis(),
        sender: Uuid::new_v4(),
        hops: 5,
        payload: Some(json),
    };
    debug!("Sending message:\n{:#?}", &message);
    let bytes = serde_cbor::to_vec(&message)?;
    socket.send_to(&bytes, &remote_addr)?;
    Ok(())
}
