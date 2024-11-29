use bevy_ecs::prelude::*;
use mavuika_message::output::MessageOutput;
use mavuika_persistence::Players;
use mavuika_proto::{Retcode, SceneInitFinishRsp};

use crate::{common::PlayerSceneStates, player_join_team::PlayerJoinTeamEvent};

#[derive(Event)]
pub struct SceneInitFinishEvent(pub u32);

pub fn on_scene_init_finish(
    mut reader: EventReader<SceneInitFinishEvent>,
    players: Res<Players>,
    mut join_team_events: EventWriter<PlayerJoinTeamEvent>,
) {
    for event in reader.read() {
        let uid = event.0;
        let player_info = players.get(uid);

        let appear_avatar_guid = player_info
            .avatar_module
            .team_map
            .get(&1)
            .unwrap()
            .avatar_guid_list
            .first()
            .copied()
            .unwrap();

        join_team_events.send(PlayerJoinTeamEvent {
            player_uid: uid,
            avatar_guid_list: player_info
                .avatar_module
                .team_map
                .get(&1)
                .unwrap()
                .avatar_guid_list
                .clone(),
            appear_avatar_guid,
        });
    }
}

pub fn scene_init_finish_send_rsp(
    mut scene_init_finish_events: EventReader<SceneInitFinishEvent>,
    player_scene_states: Res<PlayerSceneStates>,
    message_output: Res<MessageOutput>,
) {
    for event in scene_init_finish_events.read() {
        let uid = event.0;

        message_output.send(
            uid,
            SceneInitFinishRsp {
                retcode: Retcode::RetSucc.into(),
                enter_scene_token: player_scene_states.get(&uid).unwrap().enter_scene_token(),
            },
        );
    }
}
