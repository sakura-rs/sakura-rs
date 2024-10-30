use std::{collections::HashMap, fs};

use bevy_app::prelude::*;
use sakura_avatar::AvatarPlugin;
use sakura_combat::CombatPlugin;
use sakura_command::CommandPlugin;
use sakura_entity::EntityPlugin;
use sakura_environment::EnvironmentPlugin;
use sakura_inventory::InventoryPlugin;
use sakura_luashell::{LuaShellPlugin, LuaShellSettings};
use sakura_map::MapPlugin;
use sakura_message::{
    event::ClientMessageEvent,
    output::{ClientOutput, MessageOutput},
};
use sakura_pathfinding::PathfindingPlugin;
use sakura_persistence::{player_information::PlayerInformation, Players};
use sakura_proto::PlayerLoginRsp;
use sakura_scene::{common::WorldOwnerUID, ScenePlugin};
use sakura_time::TimePlugin;
use tracing::debug;

use crate::player_data_sync::PlayerDataSyncPlugin;

pub struct PlayerWorld(App);

impl PlayerWorld {
    pub fn new(player_information: PlayerInformation, output: ClientOutput) -> Self {
        let uid = player_information.uid;

        let message_out = MessageOutput::new(HashMap::from([(uid, output.clone())]));
        let players = Players::from(HashMap::from([(uid, player_information)]));

        let mut app = App::new();
        app.insert_resource(message_out)
            .insert_resource(players)
            .add_event::<ClientMessageEvent>();

        app.add_plugins(PlayerDataSyncPlugin)
            .add_plugins(EntityPlugin)
            .add_plugins(ScenePlugin)
            .add_plugins(AvatarPlugin)
            .add_plugins(InventoryPlugin)
            .add_plugins(EnvironmentPlugin)
            .add_plugins(PathfindingPlugin)
            .add_plugins(CombatPlugin)
            .add_plugins(TimePlugin)
            .add_plugins(CommandPlugin)
            .add_plugins(MapPlugin)
            .add_plugins(LuaShellPlugin);

        app.world_mut()
            .get_resource_mut::<WorldOwnerUID>()
            .unwrap()
            .0 = uid;

        app.insert_resource(LuaShellSettings {
            startup_payloads: vec![fs::read("assets/luashell/wm.bin")
                .unwrap()
                .into_boxed_slice()],
        });

        app.finish();
        app.cleanup();
        app.update();

        output.push(PlayerLoginRsp::default());
        debug!("created world for player: {uid}");

        Self(app)
    }

    pub fn add_packet(&mut self, player_uid: u32, cmd_id: u16, data: Box<[u8]>) {
        self.0
            .world_mut()
            .send_event(ClientMessageEvent::new(player_uid, cmd_id, data));
    }

    pub fn update(&mut self) {
        self.0.update();
    }

    pub fn serialize_player_information(&mut self, uid: u32) -> serde_json::Value {
        let players = self.0.world_mut().get_resource::<Players>().unwrap();
        serde_json::to_value(players.get(uid)).unwrap()
    }
}
