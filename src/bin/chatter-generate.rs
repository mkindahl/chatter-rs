extern crate chatter;

use chatter::devices::DeviceUpdate;
use chatter::gossip::Gossip;
use chatter::view::ViewUpdate;
use std::net::SocketAddr;
use uuid::Uuid;

fn print_json(gossip: Gossip) {
    match serde_json::to_string(&gossip) {
        Ok(json) => println!("JSON: {}", json),
        Err(err) => println!("Error: {}", err),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = Uuid::new_v4();
    print_json(Gossip::DebugMessage {
        text: "hello world".to_string(),
    });
    print_json(Gossip::DeviceGossip(DeviceUpdate::DeviceAdded {
        name: "gateway".to_string(),
        origin: uuid,
        description: "ASUS Router model RT-N55U ".to_string(),
    }));
    print_json(Gossip::ViewGossip(ViewUpdate::ServerAdded {
        uuid: uuid,
        addr: "127.0.0.1:8080".to_string().parse::<SocketAddr>()?,
    }));
    Ok(())
}
