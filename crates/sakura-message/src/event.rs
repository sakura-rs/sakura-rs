use bevy_ecs::prelude::*;
use sakura_proto::YSMessage;

#[derive(Event)]
pub struct ClientMessageEvent(u32, u16, Box<[u8]>);

impl ClientMessageEvent {
    pub fn new(uid: u32, cmd_id: u16, data: Box<[u8]>) -> Self {
        Self(uid, cmd_id, data)
    }

    pub const fn sender_uid(&self) -> u32 {
        self.0
    }

    pub const fn cmd_id(&self) -> u16 {
        self.1
    }

    pub fn decode<T: YSMessage + Default>(&self) -> Option<T> {
        (self.1 == T::CMD_ID)
            .then_some(T::decode(self.2.as_ref()).ok())
            .flatten()
    }
}
