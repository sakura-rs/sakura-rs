use bevy_ecs::prelude::*;
use sakura_message::output::MessageOutput;

use crate::common::PlayerSceneStates;

#[derive(Event)]
pub struct PostEnterSceneEvent(pub u32);

pub fn on_post_enter_scene(
    mut reader: EventReader<PostEnterSceneEvent>,
    player_scene_states: Res<PlayerSceneStates>,
    out: Res<MessageOutput>,
) {
    for PostEnterSceneEvent(uid) in reader.read() {
        out.send(
            *uid,
            sakura_proto::PostEnterSceneRsp {
                retcode: sakura_proto::Retcode::RetSucc.into(),
                enter_scene_token: player_scene_states.get(uid).unwrap().enter_scene_token(),
            },
        );
    }
}
