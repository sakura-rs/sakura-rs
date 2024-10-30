mod enter_scene_done;
mod enter_scene_ready;
mod post_enter_scene;
mod scene_init_finish;

pub use enter_scene_done::EnterSceneDoneEvent;
pub use enter_scene_ready::EnterSceneReadyEvent;
pub use post_enter_scene::PostEnterSceneEvent;
pub use scene_init_finish::SceneInitFinishEvent;

pub use enter_scene_done::enter_scene_done_send_rsp;
pub use scene_init_finish::scene_init_finish_send_rsp;

use crate::common::PlayerSceneStates;

use bevy_ecs::{prelude::*, system::SystemId};
use sakura_message::{event::ClientMessageEvent, output::MessageOutput};
use sakura_proto::{
    CmdID, EnterSceneDoneReq, EnterSceneDoneRsp, EnterSceneReadyReq, EnterSceneReadyRsp,
    PostEnterSceneReq, PostEnterSceneRsp, Retcode, SceneInitFinishReq, SceneInitFinishRsp,
};
use paste::paste;
use std::collections::HashMap;
use tracing::debug;

macro_rules! check_token_and_get_next_state {
    (($pkt:expr, $token:expr, $out:expr), $(($ty:ident, $state:ident)),*) => {
        match $pkt.cmd_id() {
            $(
                paste!([<$ty Req>]::CMD_ID) => {
                    let uid = $pkt.sender_uid();
                    let Some(req) = $pkt.decode::<paste!([<$ty Req>])>() else {
                        continue;
                    };

                    if req.enter_scene_token != $token {
                        $out.send(uid, ::paste::paste!([<$ty Rsp>] {
                            retcode: Retcode::RetEnterSceneTokenInvalid.into(),
                            ..Default::default()
                        }));
                        continue;
                    }
                    else {
                        EnterSceneState::$state
                    }
                }
            )*
            _ => continue,
        }
    };
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum EnterSceneState {
    Ready,
    InitFinish,
    Done,
    Post,
}

#[derive(Resource)]
pub struct EnterSceneStateSystems(HashMap<EnterSceneState, SystemId>);

impl FromWorld for EnterSceneStateSystems {
    fn from_world(world: &mut World) -> Self {
        let mut systems = HashMap::new();

        systems.insert(
            EnterSceneState::Ready,
            world.register_system(enter_scene_ready::on_enter_scene_ready),
        );

        systems.insert(
            EnterSceneState::InitFinish,
            world.register_system(scene_init_finish::on_scene_init_finish),
        );

        systems.insert(
            EnterSceneState::Done,
            world.register_system(enter_scene_done::on_enter_scene_done),
        );

        systems.insert(
            EnterSceneState::Post,
            world.register_system(post_enter_scene::on_post_enter_scene),
        );

        Self(systems)
    }
}

pub fn handle_enter_scene_state_change(
    mut messages: EventReader<ClientMessageEvent>,
    mut commands: Commands,
    systems: Res<EnterSceneStateSystems>,
    output: Res<MessageOutput>,
    mut player_scene_states: ResMut<PlayerSceneStates>,
    mut ready_events: EventWriter<EnterSceneReadyEvent>,
    mut init_finish_events: EventWriter<SceneInitFinishEvent>,
    mut done_events: EventWriter<EnterSceneDoneEvent>,
    mut post_events: EventWriter<PostEnterSceneEvent>,
) {
    for msg in messages.read() {
        let token = player_scene_states
            .get(&msg.sender_uid())
            .unwrap()
            .enter_scene_token();
        let next_enter_state = check_token_and_get_next_state!(
            (msg, token, output),
            (EnterSceneReady, Ready),
            (SceneInitFinish, InitFinish),
            (EnterSceneDone, Done),
            (PostEnterScene, Post)
        );

        let player_scene_state = player_scene_states.get_mut(&msg.sender_uid()).unwrap();
        let prev_enter_state = player_scene_state.enter_state();

        if player_scene_state.change_enter_state(next_enter_state) {
            debug!(
                "EnterScene: changing state {:?} -> {:?}",
                prev_enter_state, next_enter_state
            );

            let uid = msg.sender_uid();
            match next_enter_state {
                EnterSceneState::Ready => {
                    ready_events.send(EnterSceneReadyEvent(uid));
                }
                EnterSceneState::InitFinish => {
                    init_finish_events.send(SceneInitFinishEvent(uid));
                }
                EnterSceneState::Done => {
                    done_events.send(EnterSceneDoneEvent(uid));
                }
                EnterSceneState::Post => {
                    post_events.send(PostEnterSceneEvent(uid));
                }
            }

            commands.run_system(*systems.0.get(&next_enter_state).unwrap());
        } else {
            debug!(
                "EnterScene: state transition not allowed: {:?} -> {:?}",
                prev_enter_state, next_enter_state
            );
        }
    }
}
