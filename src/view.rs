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

use chrono::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Debug)]
pub struct ServerInfo {
    pub address: SocketAddr,
    pub last_seen: NaiveDateTime,
}

impl ServerInfo {
    pub fn new(address: SocketAddr, last_seen: NaiveDateTime) -> ServerInfo {
        ServerInfo {
            address: address,
            last_seen: last_seen,
        }
    }
}

impl fmt::Display for ServerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ViewUpdate {
    ServerAdded { uuid: Uuid, addr: SocketAddr },

    ServerRemoved { uuid: Uuid },
}

pub struct ServerView {
    pub servers: HashMap<Uuid, ServerInfo>,
}

impl ServerView {
    pub fn new() -> ServerView {
        ServerView {
            servers: HashMap::new(),
        }
    }

    pub fn process(&mut self, _sender: Uuid, timestamp_millis: i64, gossip: &ViewUpdate) {
        match gossip {
            ViewUpdate::ServerAdded { uuid, addr } => {
                let ts = NaiveDateTime::from_timestamp(
                    timestamp_millis / 1000,
                    (1_000_000 * timestamp_millis % 1000) as u32,
                );
                info!("Adding server {} with address {} to view", uuid, addr);
                self.servers
                    .insert(uuid.clone(), ServerInfo::new(addr.clone(), ts));
            }

            ViewUpdate::ServerRemoved { uuid } => {
                info!("Removing server {} from view", uuid);
                self.servers.remove(&uuid);
            }
        }
        info!("View updated: {}", *self);
    }
}

impl fmt::Display for ServerView {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (k, v) in self.servers.iter() {
            write!(f, "{}/{} ", k, v)?;
        }
        Ok(())
    }
}
