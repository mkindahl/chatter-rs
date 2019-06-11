//! Command-line utility to inject gossip into the network.
//!
//! This utility can be used to inject messages into the gossip network.

extern crate bytes;
extern crate futures;
extern crate tattler;

use std::env;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use tattler::gossip::{Gossip, GossipCodec, Message};
use tokio::net::{UdpFramed, UdpSocket};
use tokio::prelude::*;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let remote_addr: SocketAddr = env::args()
        .nth(1)
        .unwrap_or("127.0.0.1:8080".into())
        .parse()?;
    let socket = {
        let local_addr: SocketAddr = if remote_addr.is_ipv4() {
            "0.0.0.0:0"
        } else {
            "[::]:0"
        }
        .parse()?;
        UdpSocket::bind(&local_addr)?
    };
    tokio::run(
        UdpFramed::new(socket, GossipCodec {})
            .send((
                Message {
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
                    server: Uuid::new_v4(),
                    payload: Some(Gossip::Debug {
                        text: "hello world".to_string(),
                    }),
                },
                remote_addr,
            ))
            .map(|_| ())
            .map_err(|e| println!("Error: {:?}", e)),
    );
    Ok(())
}
