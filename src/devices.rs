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

use std::collections::HashMap;
use std::fmt;
use std::string::String;
use uuid::Uuid;

#[derive(Debug)]
pub struct DeviceInfo {
    pub description: String,
    pub metrics: HashMap<String, String>,
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl DeviceInfo {
    pub fn new(descr: &str) -> DeviceInfo {
        DeviceInfo {
            description: String::from(descr),
            metrics: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeviceUpdate {
    DeviceAdded {
        key: String,
        description: String,
    },

    DeviceRemoved {
        key: String,
    },

    DeviceStatus {
        key: String,
        metrics: HashMap<String, String>,
    },
}

#[derive(Debug)]
pub struct Devices {
    devices: HashMap<String, DeviceInfo>,
}

impl Devices {
    pub fn new() -> Devices {
        Devices {
            devices: HashMap::new(),
        }
    }

    pub fn process(&mut self, _sender: Uuid, _timestamp_millis: i64, gossip: &DeviceUpdate) {
        match gossip {
            DeviceUpdate::DeviceAdded { key, description } => {
                self.devices
                    .insert(key.to_string(), DeviceInfo::new(&description));
            }

            DeviceUpdate::DeviceRemoved { key } => {
                self.devices.remove(key);
            }

            DeviceUpdate::DeviceStatus { key, metrics } => {
                if let Some(info) = self.devices.get_mut(key) {
                    info.metrics = metrics.clone();
                }
            }
        }
        info!("Devices updated: {}", *self);
    }
}

impl fmt::Display for Devices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (k, v) in self.devices.iter() {
            writeln!(f, "{}: {}", k, v.description)?
        }
        Ok(())
    }
}
