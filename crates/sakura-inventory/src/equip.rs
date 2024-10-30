use bevy_ecs::prelude::*;
use sakura_entity::avatar::AvatarEquipChangeEvent;
use sakura_message::{event::ClientMessageEvent, output::MessageOutput};
use sakura_persistence::{player_information::ItemInformation, Players};
use sakura_proto::{Retcode, WearEquipReq, WearEquipRsp};
use tracing::{debug, instrument};

#[instrument(skip_all)]
pub fn change_avatar_equip(
    mut events: EventReader<ClientMessageEvent>,
    mut equip_change_events: EventWriter<AvatarEquipChangeEvent>,
    mut players: ResMut<Players>,
    message_output: Res<MessageOutput>,
) {
    for message in events.read() {
        if let Some(request) = message.decode::<WearEquipReq>() {
            let player_info = players.get_mut(message.sender_uid());
            if !player_info
                .item_map
                .get(&request.equip_guid)
                .map(|item| matches!(item, ItemInformation::Weapon { .. }))
                .unwrap_or(false)
            {
                debug!("weapon with guid {} doesn't exist", request.equip_guid);
                continue;
            }

            let Some(avatar) = player_info
                .avatar_module
                .avatar_map
                .get_mut(&request.avatar_guid)
            else {
                debug!("avatar with guid {} doesn't exist", request.avatar_guid);
                continue;
            };

            avatar.weapon_guid = request.equip_guid;

            equip_change_events.send(AvatarEquipChangeEvent {
                player_uid: message.sender_uid(),
                avatar_guid: request.avatar_guid,
                weapon_guid: request.equip_guid,
            });

            message_output.send(
                message.sender_uid(),
                WearEquipRsp {
                    retcode: Retcode::RetSucc.into(),
                    avatar_guid: request.avatar_guid,
                    equip_guid: request.equip_guid,
                },
            );
        }
    }
}
