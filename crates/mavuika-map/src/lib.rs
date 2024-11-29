use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use mavuika_message::{event::ClientMessageEvent, output::MessageOutput};

mod mark;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, data_request_processor)
            .add_systems(PreUpdate, mark::mark_map);
    }
}

fn data_request_processor(
    mut events: EventReader<ClientMessageEvent>,
    message_output: Res<MessageOutput>,
) {
    use mavuika_proto::*;

    for message in events.read() {
        if let Some(request) = message.decode::<GetSceneAreaReq>() {
            message_output.send(
                message.sender_uid(),
                GetSceneAreaRsp {
                    retcode: Retcode::RetSucc.into(),
                    city_info_list: Vec::with_capacity(0),
                    scene_id: request.scene_id,
                    area_id_list: (1..=60).collect(),
                },
            )
        }
    }
}
