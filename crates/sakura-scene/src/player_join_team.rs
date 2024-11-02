use bevy_ecs::prelude::*;
use sakura_data::excel::{avatar_excel_config_collection, weapon_excel_config_collection};
use sakura_entity::{
    ability::Ability,
    avatar::{
        AvatarAppearance, AvatarBundle, AvatarID, BornTime, ControlPeer, CurrentPlayerAvatarMarker,
        Equipment, IndexInSceneTeam, InherentProudSkillList, SkillDepot, SkillLevelMap,
    },
    common::*,
    transform::Transform,
    util::to_protocol_entity_id,
    weapon::{AffixMap, PromoteLevel, WeaponBundle, WeaponID},
};
use sakura_persistence::{player_information::ItemInformation, Players};
use sakura_proto::ProtEntityType;

use crate::{common::ScenePeerManager, scene_team_update::SceneTeamUpdateEvent};

#[derive(Event)]
pub struct PlayerJoinTeamEvent {
    pub player_uid: u32,
    pub avatar_guid_list: Vec<u64>,
    pub appear_avatar_guid: u64,
}

pub fn player_join_team(
    mut events: EventReader<PlayerJoinTeamEvent>,
    mut commands: Commands,
    players: Res<Players>,
    peer_mgr: Res<ScenePeerManager>,
    mut entity_counter: ResMut<EntityCounter>,
    mut scene_team_update_events: EventWriter<SceneTeamUpdateEvent>,
) {
    let is_empty = events.is_empty();

    for event in events.read() {
        let uid = event.player_uid;
        let player_info = players.get(uid);

        for (idx, to_spawn_guid) in event.avatar_guid_list.iter().enumerate() {
            let to_spawn = player_info
                .avatar_module
                .avatar_map
                .get(to_spawn_guid)
                .unwrap();

            let ItemInformation::Weapon {
                weapon_id,
                level,
                exp: _,
                promote_level,
                affix_map,
                is_locked: _,
            } = player_info.item_map.get(&to_spawn.weapon_guid).unwrap();

            let weapon_config = weapon_excel_config_collection::iter()
                .find(|cfg| cfg.id == *weapon_id)
                .unwrap();

            let weapon_entity = commands
                .spawn(WeaponBundle {
                    weapon_id: WeaponID(*weapon_id),
                    entity_id: to_protocol_entity_id(ProtEntityType::Weapon, entity_counter.inc()),
                    level: Level(*level),
                    guid: Guid(to_spawn.weapon_guid),
                    gadget_id: GadgetID(weapon_config.gadget_id),
                    affix_map: AffixMap(affix_map.clone()),
                    promote_level: PromoteLevel(*promote_level),
                })
                .id();

            let mut avatar_entity = commands.spawn(AvatarBundle {
                avatar_id: AvatarID(to_spawn.avatar_id),
                entity_id: to_protocol_entity_id(ProtEntityType::Avatar, entity_counter.inc()),
                guid: Guid(to_spawn.guid),
                control_peer: ControlPeer(peer_mgr.get_peer_id_by_uid(uid)),
                skill_depot: SkillDepot(to_spawn.skill_depot_id),
                level: Level(to_spawn.level),
                break_level: BreakLevel(to_spawn.break_level),
                owner_player_uid: OwnerPlayerUID(player_info.uid),
                fight_properties: create_fight_props_with_weapon(
                    avatar_excel_config_collection::iter()
                        .find(|conf| conf.id == to_spawn.avatar_id)
                        .unwrap(),
                    to_spawn.cur_hp,
                    to_spawn.level,
                    to_spawn.break_level,
                    weapon_config,
                    *level,
                ),
                life_state: LifeState::Alive,
                equipment: Equipment {
                    weapon: weapon_entity,
                },
                appearance: AvatarAppearance {
                    flycloak_id: to_spawn.wearing_flycloak_id,
                    costume_id: 0, // TODO!
                },
                transform: Transform {
                    position: player_info.world_position.position.into(),
                    rotation: player_info.world_position.rotation.into(),
                },
                ability: Ability::new_for_avatar(to_spawn.avatar_id),
                born_time: BornTime(to_spawn.born_time),
                index_in_scene_team: IndexInSceneTeam(idx as u8),
                inherent_proud_skill_list: InherentProudSkillList(
                    to_spawn.inherent_proud_skill_list.clone(),
                ),
                skill_level_map: SkillLevelMap(to_spawn.skill_level_map.clone()),
            });

            if *to_spawn_guid == event.appear_avatar_guid {
                avatar_entity.insert(CurrentPlayerAvatarMarker);
            }
        }
    }

    if !is_empty {
        scene_team_update_events.send(SceneTeamUpdateEvent);
    }
}
