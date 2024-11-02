use bevy_ecs::prelude::*;
use sakura_entity::transform::Vector3;
use sakura_persistence::Players;

use crate::{common::CurrentSceneID, BeginEnterSceneEvent};

#[derive(Event)]
pub struct ScenePlayerJumpEvent(pub u32, pub Vector3);

pub fn player_jump(
    mut events: EventReader<ScenePlayerJumpEvent>,
    mut players: ResMut<Players>,
    current_scene_id: Res<CurrentSceneID>,
    mut enter_events: EventWriter<BeginEnterSceneEvent>,
) {
    for ScenePlayerJumpEvent(uid, destination) in events.read() {
        let player = players.get_mut(*uid);
        player.world_position.position = (*destination).into();

        enter_events.send(BeginEnterSceneEvent {
            uid: *uid,
            scene_id: **current_scene_id,
            enter_type: sakura_proto::EnterType::EnterJump,
            position: *destination,
        });
    }
}
