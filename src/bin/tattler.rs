extern crate bytes;
extern crate futures;
extern crate tattler;

use std::env::args;
use std::net::SocketAddr;
use std::result::Result;
use tattler::gossip::GossipCodec;
use tokio::net::{UdpFramed, UdpSocket};
use tokio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = {
        let sockaddr = args()
            .nth(1)
            .unwrap_or("127.0.0.1:0".to_string())
            .parse::<SocketAddr>()?;
        UdpSocket::bind(&sockaddr)?
    };

    println!("Listening on {}", socket.local_addr()?);

    let udp_recv = UdpFramed::new(socket, GossipCodec {}).for_each(|(gossip, addr)| {
        println!("Gossip {:?} from address {}", gossip, addr);
        Ok(())
    });

    tokio::run({ udp_recv.map(|_| ()).map_err(|e| println!("Error: {:?}", e)) });
    Ok(())
}
