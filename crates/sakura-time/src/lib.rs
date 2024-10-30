use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use common::time_util;
use sakura_message::{event::ClientMessageEvent, output::MessageOutput};
use sakura_persistence::Players;
use sakura_proto::{
    ClientSetGameTimeReq, ClientSetGameTimeRsp, PlayerGameTimeNotify, Retcode, ServerTimeNotify,
};
use sakura_scene::{common::CurrentSceneID, SceneInitFinishEvent};
use tracing::{debug, instrument};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneTime::default())
            .add_systems(Startup, init_scene_time)
            .add_systems(PreUpdate, client_set_game_time)
            .add_systems(PostUpdate, sync_scene_time_on_scene_init);
    }
}

#[derive(Resource, Default)]
pub struct SceneTime {
    pub scene_time: u64,
    pub game_time: u32,
}

pub fn init_scene_time(mut time: ResMut<SceneTime>) {
    time.game_time = (7.31 * 60.0) as u32;
}

#[instrument(skip_all)]
pub fn client_set_game_time(
    mut events: EventReader<ClientMessageEvent>,
    players: Res<Players>,
    mut time: ResMut<SceneTime>,
    message_output: Res<MessageOutput>,
) {
    for message in events.read() {
        if let Some(request) = message.decode::<ClientSetGameTimeReq>() {
            let uid = message.sender_uid();
            let player = players.get(uid);

            let mut rsp = ClientSetGameTimeRsp::default();

            if player.basic_module.is_game_time_locked {
                debug!("game time is locked, uid: {uid}");
                rsp.retcode = Retcode::RetPlayerTimeLocked.into();
            } else {
                debug!("set game time to {}, uid: {uid}", request.game_time);

                rsp.game_time = request.game_time;
                rsp.client_game_time = request.client_game_time;
                time.game_time = request.game_time;

                message_output.send_to_all(PlayerGameTimeNotify {
                    uid,
                    is_home: false,
                    game_time: time.game_time,
                });
            }

            message_output.send(uid, rsp);
        }
    }
}

pub fn sync_scene_time_on_scene_init(
    mut events: EventReader<SceneInitFinishEvent>,
    message_output: Res<MessageOutput>,
    scene_id: Res<CurrentSceneID>,
    time: Res<SceneTime>,
) {
    use sakura_proto::{PlayerGameTimeNotify, SceneTimeNotify};

    for SceneInitFinishEvent(uid) in events.read() {
        message_output.send(
            *uid,
            ServerTimeNotify {
                server_time: time_util::unix_timestamp(),
            },
        );

        message_output.send(
            *uid,
            SceneTimeNotify {
                is_paused: false,
                scene_id: **scene_id,
                scene_time: time.scene_time,
            },
        );

        message_output.send(
            *uid,
            PlayerGameTimeNotify {
                is_home: false,
                uid: *uid,
                game_time: time.game_time,
            },
        )
    }
}
