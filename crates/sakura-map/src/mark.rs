use bevy_ecs::prelude::*;

use sakura_command::{CommandKind, DebugCommandEvent};
use sakura_message::event::ClientMessageEvent;
use sakura_proto::{MapMarkPointType, MarkMapReq, Operation};
use tracing::{debug, instrument};

#[instrument(skip_all)]
pub fn mark_map(
    mut events: EventReader<ClientMessageEvent>,
    mut debug_events: EventWriter<DebugCommandEvent>,
) {
    for message in events.read() {
        if let Some(request) = message.decode::<MarkMapReq>() {
            debug!(
                "operation: {:?}, mark: {:?}, old: {:?}",
                request.op, request.mark, request.old
            );

            if let (Operation::Add, Some(mark), _) = (request.op(), request.mark, request.old) {
                match mark.point_type() {
                    MapMarkPointType::Npc => {
                        debug_events.send(DebugCommandEvent {
                            executor_uid: message.sender_uid(),
                            kind: CommandKind::SpawnMonster {
                                monster_id: mark
                                    .name
                                    .split(' ')
                                    .next()
                                    .and_then(|s| s.parse::<u32>().ok()),
                                position: (
                                    mark.pos.unwrap_or_default().x,
                                    mark.pos.unwrap_or_default().z,
                                ),
                            },
                        });
                    }
                    MapMarkPointType::Special => {
                        debug_events.send(DebugCommandEvent {
                            executor_uid: message.sender_uid(),
                            kind: CommandKind::QuickTravel {
                                position: (
                                    mark.pos.unwrap_or_default().x,
                                    mark.name
                                        .split(' ')
                                        .next()
                                        .and_then(|s| s.parse::<f32>().ok()),
                                    mark.pos.unwrap_or_default().z,
                                ),
                            },
                        });
                    }
                    _ => (),
                }
            }
        }
    }
}
