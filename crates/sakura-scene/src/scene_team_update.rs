use std::collections::HashMap;

use crate::common::CurrentSceneID;
use bevy_ecs::prelude::*;
use sakura_entity::{
    avatar::{AvatarQueryReadOnly, CurrentPlayerAvatarMarker, IndexInSceneTeam},
    common::ToBeRemovedMarker,
    int_prop_pair,
    weapon::WeaponQueryReadOnly,
};
use sakura_message::output::MessageOutput;
use sakura_proto::{
    scene_entity_info, AbilitySyncStateInfo, AnimatorParameterValueInfo,
    AnimatorParameterValueInfoPair, AvatarExcelInfo, EntityAuthorityInfo, EntityClientData,
    EntityClientExtraInfo, EntityRendererChangedInfo, FightPropPair, MotionInfo, ProtEntityType,
    SceneAvatarInfo, SceneEntityInfo, SceneTeamAvatar, SceneTeamUpdateNotify, SceneWeaponInfo,
    Vector,
};

#[derive(Event)]
pub struct SceneTeamUpdateEvent;

pub fn notify_scene_team_update(
    mut scene_team_update_events: EventReader<SceneTeamUpdateEvent>,
    avatar_query: Query<
        (
            AvatarQueryReadOnly,
            &IndexInSceneTeam,
            Option<&CurrentPlayerAvatarMarker>,
        ),
        Without<ToBeRemovedMarker>,
    >,
    weapon_query: Query<WeaponQueryReadOnly>,
    cur_scene_id: Res<CurrentSceneID>,
    message_output: Res<MessageOutput>,
) {
    for _ in scene_team_update_events.read() {
        message_output.send_to_all(SceneTeamUpdateNotify {
            scene_team_avatar_list: avatar_query
                .iter()
                .sort::<&IndexInSceneTeam>()
                .map(|(avatar_data, _, is_cur)| {
                    let weapon_data = weapon_query.get(avatar_data.equipment.weapon).unwrap();

                    SceneTeamAvatar {
                        is_on_scene: true,
                        is_player_cur_avatar: is_cur.is_some(),
                        is_reconnect: false,
                        avatar_guid: avatar_data.guid.0,
                        weapon_guid: weapon_data.guid.0,
                        entity_id: avatar_data.entity_id.0,
                        weapon_entity_id: weapon_data.entity_id.0,
                        avatar_info: None,
                        scene_avatar_info: None,
                        scene_id: **cur_scene_id,
                        player_uid: avatar_data.owner_player_uid.0,
                        server_buff_list: Vec::with_capacity(0),
                        ability_control_block: Some(avatar_data.ability.build_control_block()),
                        avatar_ability_info: Some(AbilitySyncStateInfo::default()),
                        weapon_ability_info: Some(AbilitySyncStateInfo::default()),
                        scene_entity_info: Some(SceneEntityInfo {
                            entity_type: ProtEntityType::Avatar.into(),
                            entity_id: avatar_data.entity_id.0,
                            name: String::with_capacity(0),
                            motion_info: Some(MotionInfo {
                                pos: Some(avatar_data.transform.position.into()),
                                rot: Some(avatar_data.transform.rotation.into()),
                                speed: Some(Vector::default()),
                                ..Default::default()
                            }),
                            prop_list: vec![
                                int_prop_pair!(PROP_LEVEL, avatar_data.level.0),
                                int_prop_pair!(PROP_BREAK_LEVEL, avatar_data.break_level.0),
                            ],
                            fight_prop_list: avatar_data
                                .fight_properties
                                .0
                                .iter()
                                .map(|(k, v)| FightPropPair {
                                    prop_type: *k as u32,
                                    prop_value: *v,
                                })
                                .collect(),
                            life_state: *avatar_data.life_state as u32,
                            animator_para_list: vec![AnimatorParameterValueInfoPair {
                                name_id: 0,
                                animator_para: Some(AnimatorParameterValueInfo::default()),
                            }],
                            last_move_scene_time_ms: 0,
                            last_move_reliable_seq: 0,
                            entity_client_data: Some(EntityClientData::default()),
                            entity_environment_info_list: Vec::with_capacity(0),
                            entity_authority_info: Some(EntityAuthorityInfo {
                                ability_info: Some(AbilitySyncStateInfo::default()),
                                born_pos: Some(Vector::default()),
                                client_extra_info: Some(EntityClientExtraInfo {
                                    skill_anchor_position: Some(Vector::default()),
                                }),
                                ..Default::default()
                            }),
                            tag_list: Vec::with_capacity(0),
                            server_buff_list: Vec::with_capacity(0),
                            entity: Some(scene_entity_info::Entity::Avatar(SceneAvatarInfo {
                                uid: avatar_data.owner_player_uid.0,
                                avatar_id: avatar_data.avatar_id.0,
                                guid: avatar_data.guid.0,
                                peer_id: avatar_data.control_peer.0,
                                equip_id_list: vec![weapon_data.weapon_id.0],
                                skill_depot_id: avatar_data.skill_depot.0,
                                talent_id_list: vec![],
                                weapon: Some(SceneWeaponInfo {
                                    guid: weapon_data.guid.0,
                                    entity_id: weapon_data.entity_id.0,
                                    gadget_id: weapon_data.gadget_id.0,
                                    item_id: weapon_data.weapon_id.0,
                                    level: weapon_data.level.0,
                                    promote_level: weapon_data.promote_level.0,
                                    affix_map: weapon_data.affix_map.0.clone(),
                                    ability_info: Some(AbilitySyncStateInfo::default()),
                                    renderer_changed_info: Some(
                                        EntityRendererChangedInfo::default(),
                                    ),
                                }),
                                reliquary_list: Vec::with_capacity(0),
                                core_proud_skill_level: 0,
                                inherent_proud_skill_list: avatar_data
                                    .inherent_proud_skill_list
                                    .0
                                    .clone(),
                                skill_level_map: avatar_data.skill_level_map.0.clone(),
                                proud_skill_extra_level_map: HashMap::with_capacity(0),
                                server_buff_list: Vec::with_capacity(0),
                                team_resonance_list: Vec::with_capacity(0),
                                wearing_flycloak_id: avatar_data.appearance.flycloak_id,
                                born_time: avatar_data.born_time.0,
                                costume_id: avatar_data.appearance.costume_id,
                                cur_vehicle_info: None,
                                excel_info: Some(AvatarExcelInfo::default()),
                                anim_hash: 0,
                            })),
                        }),
                    }
                })
                .collect(),
            is_in_mp: false,
        });
    }
}
