use bevy_ecs::prelude::*;
use sakura_message::output::MessageOutput;

use crate::common::{CurrentSceneID, PlayerSceneStates, ScenePeerManager};

#[derive(Event)]
pub struct EnterSceneReadyEvent(pub u32);

pub fn on_enter_scene_ready(
    mut reader: EventReader<EnterSceneReadyEvent>,
    out: Res<MessageOutput>,
    player_scene_states: Res<PlayerSceneStates>,
    current_scene_id: Res<CurrentSceneID>,
    mut peer_manager: ResMut<ScenePeerManager>,
) {
    for event in reader.read() {
        let uid = event.0;
        let enter_scene_token = player_scene_states.get(&uid).unwrap().enter_scene_token();

        let peer_id = peer_manager.get_or_add_peer(uid);
        if peer_manager.peer_count() == 1 {
            peer_manager.make_host(peer_id);
        }

        out.send(
            uid,
            sakura_proto::EnterScenePeerNotify {
                enter_scene_token,
                peer_id,
                host_peer_id: peer_manager.host_peer_id(),
                dest_scene_id: **current_scene_id,
            },
        );

        out.send(
            uid,
            sakura_proto::EnterSceneReadyRsp {
                enter_scene_token,
                retcode: sakura_proto::Retcode::RetSucc.into(),
            },
        );
    }
}
