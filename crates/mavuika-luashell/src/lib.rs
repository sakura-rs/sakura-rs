use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use mavuika_message::output::MessageOutput;
use mavuika_proto::PlayerNormalLuaShellNotify;

pub struct LuaShellPlugin;

#[derive(Resource, Default)]
pub struct LuaShellSettings {
    pub startup_payloads: Vec<Box<[u8]>>,
}

impl Plugin for LuaShellPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LuaShellSettings::default())
            .add_systems(Startup, send_shell_payload_on_startup);
    }
}

fn send_shell_payload_on_startup(
    message_output: Res<MessageOutput>,
    settings: Res<LuaShellSettings>,
) {
    settings.startup_payloads.iter().for_each(|data| {
        message_output.send_to_all(PlayerNormalLuaShellNotify {
            config_id: 0,
            luashell: data.to_vec(),
        })
    });
}
