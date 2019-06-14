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

#[derive(Serialize, Deserialize, Debug)]
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

    pub fn process(&mut self, _sender: Uuid, _timestamp_millis: i64, gossip: DeviceUpdate) {
        match gossip {
            DeviceUpdate::DeviceAdded { key, description } => {
                self.devices.insert(key, DeviceInfo::new(&description));
            }

            DeviceUpdate::DeviceRemoved { key } => {
                self.devices.remove(&key);
            }

            DeviceUpdate::DeviceStatus { key, metrics } => {
                if let Some(info) = self.devices.get_mut(&key) {
                    info.metrics = metrics;
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
