use bevy_ecs::prelude::*;
use mavuika_entity::{
    ability::Ability,
    avatar::{AvatarQueryReadOnly, CurrentPlayerAvatarMarker},
    common::{OwnerPlayerUID, ProtocolEntityID},
    mp_level::{AuthorityPeerId, MpLevelEntityMarker},
    play_team::PlayTeamEntityMarker,
    team::TeamEntityMarker,
    weapon::WeaponQueryReadOnly,
};
use mavuika_message::output::MessageOutput;
use mavuika_proto::{
    AbilitySyncStateInfo, AvatarEnterSceneInfo, MpLevelEntityInfo, PlayTeamEntityInfo,
    PlayerEnterSceneInfoNotify, SyncScenePlayTeamEntityNotify, TeamEnterSceneInfo,
};

use crate::{
    common::{CurrentSceneID, PlayerSceneStates, ScenePeerManager},
    enter::SceneInitFinishEvent,
};

pub fn sync_enter_info(
    mut scene_init_events: EventReader<SceneInitFinishEvent>,
    message_output: Res<MessageOutput>,
    player_scene_states: Res<PlayerSceneStates>,
    team_entity_query: Query<(&ProtocolEntityID, &Ability), With<TeamEntityMarker>>,
    mp_level_entity_query: Query<(&ProtocolEntityID, &AuthorityPeerId), With<MpLevelEntityMarker>>,
    avatars: Query<(AvatarQueryReadOnly, Option<&CurrentPlayerAvatarMarker>)>,
    weapons: Query<WeaponQueryReadOnly>,
) {
    for SceneInitFinishEvent(uid) in scene_init_events.read() {
        let uid = *uid;

        let (team_entity_id, team_ability) = team_entity_query.single();
        let (mp_level_entity_id, authority_peer_id) = mp_level_entity_query.single();

        let cur_avatar_entity_id = avatars
            .iter()
            .find(|(data, cur)| data.owner_player_uid.0 == uid && cur.is_some())
            .map(|(data, _)| data.entity_id.0)
            .unwrap();

        let team_enter_info = TeamEnterSceneInfo {
            team_ability_info: Some(AbilitySyncStateInfo::default()),
            ability_control_block: Some(team_ability.build_control_block()),
            team_entity_id: team_entity_id.0,
        };

        let mp_level_entity_info = MpLevelEntityInfo {
            ability_info: Some(AbilitySyncStateInfo::default()),
            entity_id: mp_level_entity_id.0,
            authority_peer_id: authority_peer_id.0,
        };

        message_output.send(
            uid,
            PlayerEnterSceneInfoNotify {
                enter_scene_token: player_scene_states.get(&uid).unwrap().enter_scene_token(),
                cur_avatar_entity_id,
                team_enter_info: Some(team_enter_info),
                mp_level_entity_info: Some(mp_level_entity_info),
                avatar_enter_info: avatars
                    .iter()
                    .filter(|(data, _)| data.owner_player_uid.0 == uid)
                    .map(|(avatar_data, _)| {
                        let weapon_data = weapons.get(avatar_data.equipment.weapon).unwrap();

                        AvatarEnterSceneInfo {
                            avatar_guid: avatar_data.guid.0,
                            weapon_guid: weapon_data.guid.0,
                            avatar_entity_id: avatar_data.entity_id.0,
                            weapon_entity_id: weapon_data.entity_id.0,
                            avatar_ability_info: Some(AbilitySyncStateInfo::default()),
                            weapon_ability_info: Some(AbilitySyncStateInfo::default()),
                            buff_id_list: Vec::with_capacity(0),
                            server_buff_list: Vec::with_capacity(0),
                        }
                    })
                    .collect(),
            },
        );
    }
}

pub fn sync_play_team_entity(
    mut scene_init_events: EventReader<SceneInitFinishEvent>,
    message_output: Res<MessageOutput>,
    play_team_entities: Query<(&OwnerPlayerUID, &ProtocolEntityID), With<PlayTeamEntityMarker>>,
    cur_scene_id: Res<CurrentSceneID>,
    peer_mgr: Res<ScenePeerManager>,
) {
    for SceneInitFinishEvent(uid) in scene_init_events.read() {
        message_output.send(
            *uid,
            SyncScenePlayTeamEntityNotify {
                scene_id: **cur_scene_id,
                entity_info_list: play_team_entities
                    .iter()
                    .map(|(owner_uid, id)| PlayTeamEntityInfo {
                        entity_id: id.0,
                        player_uid: owner_uid.0,
                        authority_peer_id: peer_mgr.get_peer_id_by_uid(owner_uid.0),
                        gadget_config_id: 0,
                        ability_info: Some(AbilitySyncStateInfo::default()),
                    })
                    .collect(),
            },
        );
    }
}
