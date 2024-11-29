use std::collections::HashMap;

use crate::{common::*, int_prop_pair, transform::Transform};
use bevy_ecs::{prelude::*, query::QueryData};
use mavuika_message::output::MessageOutput;

#[derive(Component)]
pub struct MonsterID(pub u32);

#[derive(Bundle)]
pub struct MonsterBundle {
    pub monster_id: MonsterID,
    pub entity_id: ProtocolEntityID,
    pub level: Level,
    pub transform: Transform,
    pub fight_properties: FightProperties,
    pub life_state: LifeState,
}

#[derive(QueryData)]
pub struct MonsterQueryReadOnly {
    pub monster_id: &'static MonsterID,
    pub entity_id: &'static ProtocolEntityID,
    pub level: &'static Level,
    pub transform: &'static Transform,
    pub fight_properties: &'static FightProperties,
    pub life_state: &'static LifeState,
}

pub fn notify_appear_monster_entities(
    monsters: Query<MonsterQueryReadOnly, Added<Visible>>,
    out: Res<MessageOutput>,
) {
    use mavuika_proto::*;

    out.send_to_all(SceneEntityAppearNotify {
        appear_type: VisionType::Born.into(),
        param: 0,
        entity_list: monsters
            .iter()
            .map(|monster_data| SceneEntityInfo {
                entity_type: ProtEntityType::Monster.into(),
                entity_id: monster_data.entity_id.0,
                name: String::new(),
                motion_info: Some(MotionInfo {
                    pos: Some(monster_data.transform.position.into()),
                    rot: Some(monster_data.transform.rotation.into()),
                    speed: Some(Vector::default()),
                    ..Default::default()
                }),
                prop_list: vec![int_prop_pair!(PROP_LEVEL, monster_data.level.0)],
                fight_prop_list: monster_data
                    .fight_properties
                    .0
                    .iter()
                    .map(|(k, v)| FightPropPair {
                        prop_type: *k as u32,
                        prop_value: *v,
                    })
                    .collect(),
                life_state: *monster_data.life_state as u32,
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
                    born_pos: Some(monster_data.transform.position.into()),
                    client_extra_info: Some(EntityClientExtraInfo {
                        skill_anchor_position: Some(Vector::default()),
                    }),
                    renderer_changed_info: Some(EntityRendererChangedInfo::default()),
                    pose_para_list: Vec::with_capacity(0),
                    ..Default::default()
                }),
                tag_list: Vec::with_capacity(0),
                server_buff_list: Vec::with_capacity(0),
                entity: Some(scene_entity_info::Entity::Monster(SceneMonsterInfo {
                    monster_id: monster_data.monster_id.0,
                    group_id: 0,
                    config_id: 0,
                    weapon_list: Vec::with_capacity(0),
                    authority_peer_id: 0,
                    affix_list: Vec::with_capacity(0),
                    is_elite: false,
                    owner_entity_id: 0,
                    summoned_tag: 0,
                    summon_tag_map: HashMap::with_capacity(0),
                    pose_id: 0,
                    born_type: MonsterBornType::Default.into(),
                    block_id: 0,
                    mark_flag: 0,
                    title_id: 0,
                    special_name_id: 0,
                    attack_target_id: 0,
                    monster_route: None,
                    ai_config_id: 0,
                    level_route_id: 0,
                    init_pose_id: 0,
                    is_light: false,
                    kill_num: 0,
                    content: None,
                })),
            })
            .collect(),
    });
}

pub fn run_if_monster_entities_appeared(
    monsters: Query<MonsterQueryReadOnly, Added<Visible>>,
) -> bool {
    !monsters.is_empty()
}
