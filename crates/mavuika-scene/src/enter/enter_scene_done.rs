use bevy_ecs::prelude::*;
use mavuika_entity::{
    avatar::{AvatarQueryReadOnly, CurrentPlayerAvatarMarker},
    common::Visible,
};
use mavuika_message::output::MessageOutput;
use mavuika_proto::{EnterSceneDoneRsp, Retcode};

use crate::common::PlayerSceneStates;

#[derive(Event)]
pub struct EnterSceneDoneEvent(pub u32);

pub fn on_enter_scene_done(
    mut commands: Commands,
    mut reader: EventReader<EnterSceneDoneEvent>,
    avatars: Query<(Entity, AvatarQueryReadOnly), With<CurrentPlayerAvatarMarker>>,
) {
    for event in reader.read() {
        let uid = event.0;

        let (cur_player_avatar, _) = avatars
            .iter()
            .find(|(_, data)| data.owner_player_uid.0 == uid)
            .unwrap();

        commands.entity(cur_player_avatar).insert(Visible);
    }
}

pub fn enter_scene_done_send_rsp(
    mut enter_scene_done_events: EventReader<EnterSceneDoneEvent>,
    player_scene_states: Res<PlayerSceneStates>,
    message_output: Res<MessageOutput>,
) {
    for event in enter_scene_done_events.read() {
        let uid = event.0;

        message_output.send(
            uid,
            EnterSceneDoneRsp {
                retcode: Retcode::RetSucc.into(),
                enter_scene_token: player_scene_states.get(&uid).unwrap().enter_scene_token(),
            },
        );
    }
}
