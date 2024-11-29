use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use mavuika_message::{event::ClientMessageEvent, output::MessageOutput};
use mavuika_proto::{
    PathfindingEnterSceneReq, PathfindingEnterSceneRsp, QueryPathReq, QueryPathRsp,
    QueryPathRspPathStatusType, Retcode,
};
use tracing::debug;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, pathfinding_packet_processor);
    }
}

fn pathfinding_packet_processor(
    mut events: EventReader<ClientMessageEvent>,
    message_output: Res<MessageOutput>,
) {
    for message in events.read() {
        if let Some(request) = message.decode::<PathfindingEnterSceneReq>() {
            debug!("PathfindingEnterScene: {request:?}");
            message_output.send(message.sender_uid(), PathfindingEnterSceneRsp::default())
        } else if let Some(request) = message.decode::<QueryPathReq>() {
            debug!("QueryPath: {request:?}");

            let mut corners = Vec::with_capacity(2);

            if let Some(source_pos) = request.source_pos {
                corners.push(source_pos);
            }

            if let Some(destination) = request.destination_pos.first() {
                corners.push(*destination);
            }

            message_output.send(
                message.sender_uid(),
                QueryPathRsp {
                    retcode: Retcode::RetSucc.into(),
                    query_status: QueryPathRspPathStatusType::PathStatusTypeSucc.into(),
                    query_id: request.query_id,
                    corners,
                },
            )
        }
    }
}
