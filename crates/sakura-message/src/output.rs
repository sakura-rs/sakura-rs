use std::collections::HashMap;

use bevy_ecs::system::Resource;
use sakura_proto::YSMessage;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct ClientOutput(mpsc::Sender<(u16, Box<[u8]>)>);

#[derive(Resource)]
pub struct MessageOutput(HashMap<u32, ClientOutput>);

impl MessageOutput {
    pub fn new(client_map: HashMap<u32, ClientOutput>) -> Self {
        Self(client_map)
    }

    pub fn send(&self, player_uid: u32, message: impl YSMessage) {
        if let Some(out) = self.0.get(&player_uid) {
            out.push(message);
        }
    }

    pub fn send_to_all(&self, message: impl YSMessage + Clone) {
        for out in self.0.values() {
            out.push(message.clone());
        }
    }
}

impl ClientOutput {
    pub fn new(tx: mpsc::Sender<(u16, Box<[u8]>)>) -> Self {
        Self(tx)
    }

    pub fn push(&self, message: impl YSMessage) {
        self.0
            .blocking_send((
                message.get_cmd_id(),
                message.encode_to_vec().into_boxed_slice(),
            ))
            .unwrap()
    }
}