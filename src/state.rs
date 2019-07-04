use crate::devices::{DeviceCollection, DeviceUpdate};
use crate::view::{ServerView, ViewUpdate};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct State {
    pub devices: Arc<Mutex<DeviceCollection>>,
    pub view: Arc<Mutex<ServerView>>,
}

impl State {
    pub fn new() -> State {
        State {
            devices: Arc::new(Mutex::new(DeviceCollection::new())),
            view: Arc::new(Mutex::new(ServerView::new())),
        }
    }

    pub fn update_devices(&mut self, update: &DeviceUpdate, sender: &Uuid, timestamp_millis: i64) {
        self.devices
            .lock()
            .expect("unable to lock device collection for update")
            .update(update, sender, timestamp_millis);
    }

    pub fn update_view(&mut self, update: &ViewUpdate, sender: &Uuid, timestamp_millis: i64) {
        self.view
            .lock()
            .expect("unable to lock view for update")
            .update(update, sender, timestamp_millis);
    }
}
