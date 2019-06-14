#[macro_use]
extern crate log;
extern crate env_logger;

extern crate bytes;
extern crate chatter;
extern crate futures;

use chatter::devices::Devices;
use chatter::gossip::{Gossip, GossipCodec, Message};
use chatter::view::ServerView;
use std::env::args;
use std::net::SocketAddr;
use std::result::Result;
use std::sync::{Arc, Mutex};
use tokio::net::{UdpFramed, UdpSocket};
use tokio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let socket = {
        let sockaddr = args()
            .nth(1)
            .unwrap_or("127.0.0.1:8080".to_string())
            .parse::<SocketAddr>()?;
        UdpSocket::bind(&sockaddr)?
    };

    info!("Listening on {}", socket.local_addr()?);

    // Shared view of all existing agents in the cluster.
    let shared_view = Arc::new(Mutex::new(ServerView::new()));

    // Shared device information.
    let shared_devices = Arc::new(Mutex::new(Devices::new()));

    // Future for updating state based on received gossip.
    let update_future = {
        let devices = shared_devices.clone();
        let view = shared_view.clone();
        move |(msg, addr): (Message, SocketAddr)| {
            debug!("Received gossip message from address {}: {:?}", addr, msg);
            if let Some(gossip) = msg.payload {
                match gossip {
                    Gossip::DebugMessage { text } => info!("From {}  {}", addr, text),

                    Gossip::DeviceGossip(device_gossip) => devices
                        .lock()
                        .expect("unable to lock devices database for update")
                        .process(msg.sender, msg.timestamp_millis, device_gossip),

                    Gossip::ViewGossip(view_gossip) => view
                        .lock()
                        .expect("unable to lock view for update")
                        .process(msg.sender, msg.timestamp_millis, view_gossip),
                }
            }
            Ok(())
        }
    };

    tokio::run({
        UdpFramed::new(socket, GossipCodec::new())
            .for_each(update_future)
            .map(|_| ())
            .map_err(|e| error!("error: {:?}", e))
    });
    Ok(())
}
