//! Command-line utility to inject gossip into the network.
//!
//! This utility can be used to inject messages into the gossip network.

extern crate bytes;
extern crate chatter;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate log;

use chatter::gossip::{Gossip, GossipCodec, Message};
use chatter::view::ViewUpdate;
use chrono::Utc;
use std::env;
use std::net::SocketAddr;
use tokio::net::{UdpFramed, UdpSocket};
use tokio::prelude::*;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let remote_addr: SocketAddr = env::args()
        .nth(1)
        .unwrap_or("127.0.0.1:8080".into())
        .parse()?;
    let local_addr = "0.0.0.0:8084".parse::<SocketAddr>()?;
    let socket = UdpSocket::bind(&local_addr)?;

    let uuid = Uuid::new_v4();
    let server_uuid = Uuid::new_v4();
    tokio::run(
        UdpFramed::new(socket, GossipCodec {})
            .send((
                Message {
                    timestamp_millis: Utc::now().timestamp_millis(),
                    sender: uuid,
                    payload: Some(Gossip::DebugMessage {
                        text: "hello world".to_string(),
                    }),
                },
                remote_addr,
            ))
            .wait()
            .expect("debug message failed")
            .send((
                Message {
                    timestamp_millis: Utc::now().timestamp_millis(),
                    sender: uuid,
                    payload: Some(Gossip::ViewGossip(ViewUpdate::ServerAdded {
                        uuid: server_uuid,
                        addr: local_addr,
                    })),
                },
                remote_addr,
            ))
            .wait()
            .expect("debug message failed")
            .send((
                Message {
                    timestamp_millis: Utc::now().timestamp_millis(),
                    sender: uuid,
                    payload: Some(Gossip::ViewGossip(ViewUpdate::ServerRemoved {
                        uuid: server_uuid,
                    })),
                },
                remote_addr,
            ))
            .map(|_| ())
            .map_err(|e| error!("error: {:?}", e)),
    );
    Ok(())
}
