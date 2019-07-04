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

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let options = App::new("Chatter Agent")
        .version("0.1")
        .author("Mats Kindahl <mats.kindahl@gmail.com>")
        .about("Monitoring agent for distributed systems.")
        .arg(
            Arg::with_name("listen")
                .short("l")
                .long("listen")
                .value_name("ADDRESS")
                .help("Address to listen for gossip on")
                .takes_value(true),
        )
        .get_matches();

    let socket = {
        let sockaddr = options
            .value_of("listen")
            .unwrap_or("0.0.0.0:2428")
            .parse::<SocketAddr>()?;
        UdpSocket::bind(&sockaddr)?
    };

    info!("Listening on {}", socket.local_addr()?);

    // Shared view of all existing agents in the cluster.
    let shared_view = Arc::new(Mutex::new(ServerView::new()));

    // Shared device information.
    let shared_devices = Arc::new(Mutex::new(Devices::new()));

    let (mut writer, reader) = UdpFramed::new(socket, GossipCodec::new()).split();

    // Future for updating state based on received gossip.
    let update_future = {
        let devices = shared_devices.clone();
        let view = shared_view.clone();
        move |(msg, addr): (Message, SocketAddr)| {
            if let Some(ref gossip) = msg.payload {
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
            Ok((msg, addr))
        }
    };

    // Future for forwarding gossip to other servers in the
    // view. Right now, it forwards the message to all servers in the
    // cluster, not just a subset.
    let gossip_future = {
        let view = shared_view.clone();
        move |(mut msg, addr): (Message, SocketAddr)| {
            if msg.hops > 0 {
                msg.hops -= 1;
                let locked_view = view.lock().expect("unable to lock view for forwarding");
                for (_uuid, info) in &locked_view.servers {
                    writer.start_send((msg.clone(), info.address.clone()))?;
                }
                writer.poll_complete()?;
            }
            Ok((msg, addr))
        }
    };

    tokio::run({
        reader
            .and_then(move |(msg, addr): (Message, SocketAddr)| {
                debug!("Received gossip message from address {}: {:?}", addr, msg);
                Ok((msg, addr))
            })
            .and_then(update_future)
            .and_then(gossip_future)
            .for_each(|(_msg, addr): (Message, SocketAddr)| {
                debug!("Finished processing gossip message from {}", addr);
                Ok(())
            })
            .map(|_| ())
            .map_err(|e| error!("error: {:?}", e))
    });
    Ok(())
}
