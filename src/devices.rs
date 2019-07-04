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

//! Module for managing the device collection.

use std::collections::HashMap;
use std::fmt;
use std::string::String;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Metric {
    Text(String),
}

#[derive(Debug)]
pub struct DeviceInfo {
    /// The UUID of the agent that is responsible for the device.
    pub owner: Uuid,

    /// Name of the device.
    pub name: String,

    /// Description of the device.
    pub description: String,

    /// Collection of metrics containing the current status of the
    /// device.
    pub metrics: HashMap<String, Metric>,
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl DeviceInfo {
    /// Construct device information
    ///
    /// Each chatter agent owns a set of named devices, where each
    /// name is unique for a device. The combination of agent UUID and
    /// device name uniquely identify the device in the entire
    /// deployment. Any changes to the device status or generation of
    /// metrics is goverened completely by the owner, which guarantee
    /// the order of changes to the device status.
    ///
    /// # Parameters
    ///
    /// * `uuid` - The UUID of the agent that owns the device.
    ///
    /// * `name` - The name of the device, unique for each agent.
    ///
    /// * `descr` - Device description. Intended for humans.
    ///
    pub fn new(uuid: &Uuid, name: &str, descr: &str) -> DeviceInfo {
        DeviceInfo {
            owner: uuid.clone(),
            name: String::from(name),
            description: String::from(descr),
            metrics: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeviceUpdate {
    DeviceAdded {
        origin: Uuid,
        name: String,
        description: String,
    },

    DeviceRemoved {
        origin: Uuid,
        name: String,
    },

    DeviceStatus {
        origin: Uuid,
        name: String,
        metrics: HashMap<String, Metric>,
    },
}

#[derive(Debug)]
pub struct DeviceCollection {
    devices: HashMap<Uuid, HashMap<String, DeviceInfo>>,
}

impl DeviceCollection {
    pub fn new() -> DeviceCollection {
        DeviceCollection {
            devices: HashMap::new(),
        }
    }

    pub fn update(&mut self, gossip: &DeviceUpdate, _origin: &Uuid, _timestamp_millis: i64) {
        match gossip {
            DeviceUpdate::DeviceAdded {
                origin,
                name,
                description,
            } => {
                self.devices
                    .entry(origin.clone())
                    .or_insert(HashMap::new())
                    .insert(
                        name.to_string(),
                        DeviceInfo::new(origin, &name, &description),
                    );
                debug!("Added device {} to {}", name, origin);
            }

            DeviceUpdate::DeviceRemoved { origin, name } => {
                if let Some(entry) = self.devices.get_mut(origin) {
                    entry.remove(name);
                    debug!("Removed device {} from {}", name, origin);
                }
            }

            DeviceUpdate::DeviceStatus {
                origin,
                name,
                metrics,
            } => {
                if let Some(agent) = self.devices.get_mut(origin) {
                    if let Some(info) = agent.get_mut(name) {
                        for (metric, value) in metrics {
                            *info
                                .metrics
                                .entry(metric.to_string())
                                .or_insert(value.clone()) = value.clone();
                        }
                    }
                    debug!("Updated device {} on {}: {:?}", name, origin, metrics);
                } else {
                    warn!(
                        "Update of device {} on agent {} failed - agent not added",
                        name, origin
                    );
                }
            }
        }
        info!("Devices updated: {}", *self);
    }
}

impl fmt::Display for DeviceCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (origin, map) in self.devices.iter() {
            for (name, info) in map {
                writeln!(f, "{} {}: {}", origin, name, info.description)?
            }
        }
        Ok(())
    }
}
