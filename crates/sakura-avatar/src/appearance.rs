use bevy_ecs::prelude::*;
use sakura_data::excel::avatar_costume_excel_config_collection;
use sakura_entity::avatar::AvatarCostumeChangeEvent;
use sakura_message::{event::ClientMessageEvent, output::MessageOutput};
use sakura_persistence::{player_information::PlayerInformation, Players};
use sakura_proto::{
    AvatarChangeCostumeReq, AvatarChangeCostumeRsp, AvatarFlycloakChangeNotify,
    AvatarWearFlycloakReq, AvatarWearFlycloakRsp, Retcode,
};
use tracing::{debug, instrument};

#[instrument(skip_all)]
pub fn handle_appearance_change_request(
    mut events: EventReader<ClientMessageEvent>,
    mut players: ResMut<Players>,
    message_output: Res<MessageOutput>,
    mut costume_change_events: EventWriter<AvatarCostumeChangeEvent>,
) {
    for message in events.read() {
        if let Some(request) = message.decode::<AvatarWearFlycloakReq>() {
            let player = players.get_mut(message.sender_uid());
            let mut rsp = AvatarWearFlycloakRsp::default();

            if let Some(notify) = wear_flycloak(player, request, &mut rsp) {
                message_output.send_to_all(notify);
            }

            message_output.send(message.sender_uid(), rsp);
        } else if let Some(request) = message.decode::<AvatarChangeCostumeReq>() {
            let player = players.get_mut(message.sender_uid());
            let mut rsp = AvatarChangeCostumeRsp::default();

            if let Some(change_event) = change_costume(player, request, &mut rsp) {
                costume_change_events.send(change_event);
            }

            message_output.send(message.sender_uid(), rsp);
        }
    }
}

#[instrument(skip(player, response))]
fn wear_flycloak(
    player: &mut PlayerInformation,
    request: AvatarWearFlycloakReq,
    response: &mut AvatarWearFlycloakRsp,
) -> Option<AvatarFlycloakChangeNotify> {
    if !player
        .avatar_module
        .owned_flycloak_set
        .contains(&request.flycloak_id)
    {
        debug!("flycloak id {} is not owned", request.flycloak_id);
        response.retcode = Retcode::RetNotHasFlycloak.into();
        return None;
    }

    let Some(avatar) = player
        .avatar_module
        .avatar_map
        .get_mut(&request.avatar_guid)
    else {
        debug!("avatar with guid {} not found", request.avatar_guid);
        response.retcode = Retcode::RetCanNotFindAvatar.into();
        return None;
    };

    response.avatar_guid = request.avatar_guid;
    response.flycloak_id = request.flycloak_id;

    avatar.wearing_flycloak_id = request.flycloak_id;
    debug!(
        "wear flycloak_id: {}, avatar_guid: {}",
        request.flycloak_id, request.avatar_guid
    );

    Some(AvatarFlycloakChangeNotify {
        avatar_guid: request.avatar_guid,
        flycloak_id: request.flycloak_id,
    })
}

#[instrument(skip(player, response))]
fn change_costume(
    player: &mut PlayerInformation,
    request: AvatarChangeCostumeReq,
    response: &mut AvatarChangeCostumeRsp,
) -> Option<AvatarCostumeChangeEvent> {
    response.retcode = Retcode::RetFail.into();

    let config = (request.costume_id != 0)
        .then(|| {
            avatar_costume_excel_config_collection::iter().find(|c| c.skin_id == request.costume_id)
        })
        .flatten();

    if request.costume_id != 0 && config.is_none() {
        debug!("costume_id {} config doesn't exist", request.costume_id);
        return None;
    };

    if !player
        .avatar_module
        .owned_costume_set
        .contains(&request.costume_id)
        && config.is_some()
    {
        debug!("costume is not unlocked, id: {}", request.costume_id);
        response.retcode = Retcode::RetNotHasCostume.into();
        return None;
    }

    let Some(avatar) = player
        .avatar_module
        .avatar_map
        .get_mut(&request.avatar_guid)
    else {
        debug!("avatar guid {} doesn't exist", request.avatar_guid);
        return None;
    };

    if let Some(config) = config {
        if config.character_id != avatar.avatar_id {
            debug!(
                "avatar costume mismatch, config: {}, requested: {}",
                config.character_id, avatar.avatar_id
            );
            response.retcode = Retcode::RetCostumeAvatarError.into();
            return None;
        }
    }

    response.avatar_guid = request.avatar_guid;
    response.costume_id = request.costume_id;
    response.retcode = Retcode::RetSucc.into();
    avatar.costume_id = request.costume_id;

    debug!(
        "change costume for avatar {} to {}",
        avatar.avatar_id, request.costume_id
    );

    Some(AvatarCostumeChangeEvent(
        player.uid,
        request.avatar_guid,
        request.costume_id,
    ))
}
