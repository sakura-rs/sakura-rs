use bevy_ecs::prelude::*;
use sakura_message::{event::ClientMessageEvent, output::MessageOutput};
use sakura_persistence::Players;
use sakura_proto::{
    AvatarFlycloakChangeNotify, AvatarWearFlycloakReq, AvatarWearFlycloakRsp, Retcode,
};
use tracing::{debug, instrument};

#[instrument(skip_all)]
pub fn handle_appearance_change_request(
    mut events: EventReader<ClientMessageEvent>,
    mut players: ResMut<Players>,
    message_output: Res<MessageOutput>,
) {
    for message in events.read() {
        if let Some(request) = message.decode::<AvatarWearFlycloakReq>() {
            let player = players.get_mut(message.sender_uid());

            let mut rsp = AvatarWearFlycloakRsp::default();
            if player
                .avatar_module
                .owned_flycloak_set
                .contains(&request.flycloak_id)
            {
                if let Some(avatar) = player
                    .avatar_module
                    .avatar_map
                    .get_mut(&request.avatar_guid)
                {
                    rsp.avatar_guid = request.avatar_guid;
                    rsp.flycloak_id = request.flycloak_id;

                    avatar.wearing_flycloak_id = request.flycloak_id;
                    debug!(
                        "wear flycloak_id: {}, avatar_guid: {}",
                        request.flycloak_id, request.avatar_guid
                    );

                    message_output.send_to_all(AvatarFlycloakChangeNotify {
                        avatar_guid: request.avatar_guid,
                        flycloak_id: request.flycloak_id,
                    });
                } else {
                    debug!("avatar with guid {} not found", request.avatar_guid);
                    rsp.retcode = Retcode::RetCanNotFindAvatar.into();
                }
            } else {
                debug!("flycloak id {} is not owned", request.flycloak_id);
                rsp.retcode = Retcode::RetNotHasFlycloak.into();
            }

            message_output.send(message.sender_uid(), rsp);
        }
    }
}
