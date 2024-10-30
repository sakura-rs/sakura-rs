use std::collections::HashMap;

use bevy_ecs::prelude::*;
use sakura_entity::{
    avatar::CurrentPlayerAvatarMarker,
    common::{OwnerPlayerUID, ProtocolEntityID, ToBeRemovedMarker},
};
use sakura_message::{event::ClientMessageEvent, output::MessageOutput};
use sakura_proto::{
    EvtAvatarLockChairReq, EvtAvatarLockChairRsp, EvtAvatarStandUpNotify, Retcode,
};
use tracing::{debug, instrument};

#[derive(Resource, Default)]
pub struct ChairLockMap(HashMap<u64, (u32, u32)>);

#[instrument(skip_all)]
pub fn avatar_lock_chair(
    mut events: EventReader<ClientMessageEvent>,
    out: Res<MessageOutput>,
    mut lock: ResMut<ChairLockMap>,
    active_entities: Query<
        (&OwnerPlayerUID, &ProtocolEntityID),
        (With<CurrentPlayerAvatarMarker>, Without<ToBeRemovedMarker>),
    >,
) {
    for message in events.read() {
        if let Some(request) = message.decode::<EvtAvatarLockChairReq>() {
            let uid = message.sender_uid();
            let mut rsp = EvtAvatarLockChairRsp::default();

            if lock.0.contains_key(&request.chair_id) {
                debug!("chair with id {} is already locked", request.chair_id);
                rsp.retcode = Retcode::RetFail.into();
            } else {
                let entity_id = active_entities
                    .iter()
                    .find(|(owner_uid, _)| owner_uid.0 == uid)
                    .unwrap()
                    .1;

                lock.0
                    .insert(request.chair_id, (message.sender_uid(), entity_id.0));

                rsp.chair_id = request.chair_id;
                rsp.entity_id = entity_id.0;
                rsp.direction = request.direction;
                rsp.position = request.position;

                debug!(
                    "chair id {} is now locked by player: {uid}",
                    request.chair_id
                );
            }

            out.send(message.sender_uid(), rsp);
        } else if let Some(_) = message.decode::<EvtAvatarStandUpNotify>() {
            let uid = message.sender_uid();

            lock.0.retain(|id, (locked_by, _)| {
                if *locked_by == uid {
                    debug!("chair id {id} is now unlocked");
                    false
                } else {
                    true
                }
            });
        }
    }
}