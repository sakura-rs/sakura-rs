use std::collections::HashSet;

use bevy_ecs::prelude::*;
use sakura_entity::{
    avatar::{AvatarQueryReadOnly, CurrentPlayerAvatarMarker},
    common::{ToBeRemovedMarker, Visible},
    transform::Transform,
    EntityDisappearEvent,
};
use sakura_message::{event::ClientMessageEvent, output::MessageOutput};
use sakura_persistence::Players;
use sakura_proto::{
    AvatarTeam, AvatarTeamUpdateNotify, ChangeAvatarReq, ChangeAvatarRsp, Retcode,
    SetUpAvatarTeamReq, SetUpAvatarTeamRsp,
};
use tracing::{debug, instrument};

use crate::player_join_team::PlayerJoinTeamEvent;

#[derive(Event, Debug)]
pub struct PlayerAvatarTeamChanged {
    pub uid: u32,
    pub avatar_team_guid_list: Vec<u64>,
    pub cur_avatar_guid: u64,
}

pub fn change_avatar(
    mut client_messages: EventReader<ClientMessageEvent>,
    out: Res<MessageOutput>,
    mut commands: Commands,
    avatars: Query<(
        Entity,
        AvatarQueryReadOnly,
        Option<&CurrentPlayerAvatarMarker>,
    )>,
    mut disappear_events: EventWriter<EntityDisappearEvent>,
) {
    for message in client_messages.read() {
        if let Some(request) = message.decode::<ChangeAvatarReq>() {
            let (cur_entity, cur_avatar_data, _) = avatars
                .iter()
                .find(|(_, data, is_cur)| {
                    data.owner_player_uid.0 == message.sender_uid() && is_cur.is_some()
                })
                .unwrap();

            if cur_avatar_data.guid.0 != request.guid {
                if let Some((new_entity, _, _)) = avatars.iter().find(|(_, data, _)| {
                    data.owner_player_uid.0 == message.sender_uid() && data.guid.0 == request.guid
                }) {
                    disappear_events.send(EntityDisappearEvent(
                        cur_avatar_data.entity_id.0,
                        sakura_proto::VisionType::Replace,
                    ));

                    commands
                        .entity(cur_entity)
                        .remove::<CurrentPlayerAvatarMarker>()
                        .remove::<Visible>();

                    let transform = match (request.is_move, request.move_pos) {
                        (true, Some(move_pos)) => Transform {
                            position: move_pos.into(),
                            rotation: cur_avatar_data.transform.rotation,
                        },
                        _ => cur_avatar_data.transform.clone(),
                    };

                    commands
                        .entity(new_entity)
                        .insert(CurrentPlayerAvatarMarker)
                        .insert(Visible)
                        .insert(transform);

                    out.send(
                        message.sender_uid(),
                        ChangeAvatarRsp {
                            cur_guid: request.guid,
                            skill_id: request.skill_id,
                            retcode: Retcode::RetSucc.into(),
                        },
                    );
                }
            }
        }
    }
}

#[instrument(skip_all)]
pub fn set_up_avatar_team(
    mut client_messages: EventReader<ClientMessageEvent>,
    out: Res<MessageOutput>,
    mut players: ResMut<Players>,
    mut change_events: EventWriter<PlayerAvatarTeamChanged>,
) {
    for message in client_messages.read() {
        if let Some(request) = message.decode::<SetUpAvatarTeamReq>() {
            if !(1..=4).contains(&request.avatar_team_guid_list.len())
                || !request
                    .avatar_team_guid_list
                    .contains(&request.cur_avatar_guid)
            {
                out.send(
                    message.sender_uid(),
                    SetUpAvatarTeamRsp {
                        retcode: Retcode::RetFail.into(),
                        ..Default::default()
                    },
                );
                continue;
            }

            let mut avatar_set = HashSet::with_capacity(request.avatar_team_guid_list.len());
            for guid in request.avatar_team_guid_list.iter() {
                if !avatar_set.insert(*guid) {
                    debug!(
                        "duplicate guid {guid} in avatar team {:?}",
                        request.avatar_team_guid_list
                    );

                    out.send(
                        message.sender_uid(),
                        SetUpAvatarTeamRsp {
                            retcode: Retcode::RetFail.into(),
                            ..Default::default()
                        },
                    );
                    continue;
                }
            }

            let player = players.get_mut(message.sender_uid());
            if let Some(team) = player.avatar_module.team_map.get_mut(&request.team_id) {
                team.avatar_guid_list = request.avatar_team_guid_list.clone();

                change_events.send(PlayerAvatarTeamChanged {
                    uid: message.sender_uid(),
                    avatar_team_guid_list: request.avatar_team_guid_list.clone(),
                    cur_avatar_guid: request.cur_avatar_guid,
                });

                out.send(
                    message.sender_uid(),
                    SetUpAvatarTeamRsp {
                        retcode: Retcode::RetSucc.into(),
                        team_id: request.team_id,
                        cur_avatar_guid: request.cur_avatar_guid,
                        avatar_team_guid_list: request.avatar_team_guid_list.clone(),
                    },
                );
            } else {
                debug!("team_id {} doesn't exist", request.team_id);

                out.send(
                    message.sender_uid(),
                    SetUpAvatarTeamRsp {
                        retcode: Retcode::RetFail.into(),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

pub fn replace_avatar_team(
    mut events: EventReader<PlayerAvatarTeamChanged>,
    mut commands: Commands,
    avatars: Query<(Entity, AvatarQueryReadOnly)>,
    mut join_team_events: EventWriter<PlayerJoinTeamEvent>,
) {
    for event in events.read() {
        // TODO: multiple teams - check if modified team is active

        for (avatar_entity, avatar_data) in avatars
            .iter()
            .filter(|(_, a)| a.owner_player_uid.0 == event.uid)
        {
            commands.entity(avatar_entity).insert(ToBeRemovedMarker);
            commands
                .entity(avatar_data.equipment.weapon)
                .insert(ToBeRemovedMarker);
        }

        join_team_events.send(PlayerJoinTeamEvent {
            player_uid: event.uid,
            avatar_guid_list: event.avatar_team_guid_list.clone(),
            appear_avatar_guid: event.cur_avatar_guid,
        });
    }
}

#[instrument(skip_all)]
pub fn notify_avatar_team_update(
    mut events: EventReader<PlayerAvatarTeamChanged>,
    players: Res<Players>,
    out: Res<MessageOutput>,
) {
    for event in events.read() {
        debug!("{event:?}");

        let player = players.get(event.uid);

        out.send(
            event.uid,
            AvatarTeamUpdateNotify {
                temp_avatar_guid_list: Vec::with_capacity(0),
                avatar_team_map: player
                    .avatar_module
                    .team_map
                    .iter()
                    .map(|(idx, team)| {
                        (
                            *idx,
                            AvatarTeam {
                                team_name: team.name.clone(),
                                avatar_guid_list: team.avatar_guid_list.clone(),
                            },
                        )
                    })
                    .collect(),
            },
        );
    }
}
