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

use chatter::gossip::{GossipCodec, Message};
use chatter::state::State;
use std::net::SocketAddr;
use std::result::Result;
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

    let shared_state = State::new();

    let (mut writer, reader) = UdpFramed::new(socket, GossipCodec::new()).split();

    // Future for updating state based on received gossip.
    let update_future = {
        let mut state = shared_state.clone();
        move |(msg, addr): (Message, SocketAddr)| {
            msg.update_state(&mut state, &addr);
            Ok((msg, addr))
        }
    };

    // Future for forwarding gossip to other servers in the
    // view. Right now, it forwards the message to all servers in the
    // cluster, not just a subset.
    let gossip_future = {
        let state = shared_state.clone();
        move |(mut msg, addr): (Message, SocketAddr)| {
            if msg.hops > 0 {
                msg.hops -= 1;
                let locked_view = state
                    .view
                    .lock()
                    .expect("unable to lock view for forwarding");
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
