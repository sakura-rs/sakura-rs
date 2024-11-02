use std::collections::HashMap;

use common::time_util;
use sakura_data::excel::{
    avatar_excel_config_collection, avatar_flycloak_excel_config_collection,
    avatar_skill_depot_excel_config_collection, weapon_excel_config_collection, AvatarExcelConfig,
    AvatarUseType,
};

use sakura_persistence::player_information::*;

pub fn create_default_player_information(uid: u32, nick_name: String) -> PlayerInformation {
    const DEFAULT_TEAM: [u32; 1] = [10000058];
    const DEFAULT_LEVEL: u32 = 60;

    let mut player = PlayerInformation {
        uid,
        nick_name,
        guid_counter: 0,
        basic_module: BasicModuleInformation {
            level: DEFAULT_LEVEL,
            exp: 0,
            is_game_time_locked: false,
        },
        avatar_module: AvatarModuleInformation {
            avatar_map: HashMap::new(),
            team_map: HashMap::new(),
            owned_flycloak_set: avatar_flycloak_excel_config_collection::iter()
                .map(|c| c.flycloak_id)
                .collect(),
        },
        item_map: HashMap::new(),
        world_position: PlayerPositionInformation {
            scene_id: 3,
            position: (2336.789, 249.98996, -751.3081),
            rotation: (0.0, 0.0, 0.0),
        },
    };

    avatar_excel_config_collection::iter()
        .filter(|avatar| avatar.use_type == AvatarUseType::Formal)
        .for_each(|avatar| add_avatar_and_weapon(&mut player, avatar));

    player.avatar_module.team_map.insert(
        1,
        AvatarTeamInformation {
            avatar_guid_list: player
                .avatar_module
                .avatar_map
                .iter()
                .filter(|(_, av)| DEFAULT_TEAM.contains(&av.avatar_id))
                .map(|(guid, _)| *guid)
                .collect(),
            name: String::new(),
        },
    );

    // Add bunch of weapons to inventory
    weapon_excel_config_collection::iter().for_each(|weapon| {
        let guid = player.next_guid();
        player.item_map.insert(
            guid,
            ItemInformation::Weapon {
                weapon_id: weapon.id,
                level: 90,
                exp: 0,
                promote_level: 6,
                affix_map: HashMap::with_capacity(0),
                is_locked: false,
            },
        );
    });

    player
}

fn add_avatar_and_weapon(player: &mut PlayerInformation, avatar: &AvatarExcelConfig) {
    const DEFAULT_AVATAR_LEVEL: u32 = 90;
    const DEFAULT_AVATAR_BREAK_LEVEL: u32 = 6;
    const DEFAULT_WEAPON_LEVEL: u32 = 90;
    const DEFAULT_WEAPON_PROMOTE_LEVEL: u32 = 6;
    const DEFAULT_FLYCLOAK_ID: u32 = 140001;

    let avatar_guid = player.next_guid();
    let weapon_guid = player.next_guid();

    let mut skill_level_map = HashMap::new();
    let mut inherent_proud_skill_list = Vec::new();

    if let Some(skill_depot) =
        avatar_skill_depot_excel_config_collection::iter().find(|c| c.id == avatar.skill_depot_id)
    {
        skill_depot
            .skills
            .iter()
            .filter(|id| **id != 0)
            .for_each(|id| {
                skill_level_map.insert(*id, 14);
            });

        skill_depot
            .sub_skills
            .iter()
            .filter(|id| **id != 0)
            .for_each(|id| {
                skill_level_map.insert(*id, 14);
            });

        skill_level_map.insert(skill_depot.energy_skill, 14);

        skill_depot
            .inherent_proud_skill_opens
            .iter()
            .filter(|s| s.proud_skill_group_id != 0)
            .for_each(|s| inherent_proud_skill_list.push(s.proud_skill_group_id * 100 + 1));
    }

    player.avatar_module.avatar_map.insert(
        avatar_guid,
        AvatarInformation {
            avatar_id: avatar.id,
            level: DEFAULT_AVATAR_LEVEL,
            break_level: DEFAULT_AVATAR_BREAK_LEVEL,
            skill_depot_id: avatar.skill_depot_id,
            born_time: time_util::unix_timestamp() as u32,
            guid: avatar_guid,
            weapon_guid,
            cur_hp: avatar.hp_base,
            skill_level_map,
            inherent_proud_skill_list,
            wearing_flycloak_id: DEFAULT_FLYCLOAK_ID,
        },
    );

    player.item_map.insert(
        weapon_guid,
        ItemInformation::Weapon {
            weapon_id: avatar.initial_weapon,
            level: DEFAULT_WEAPON_LEVEL,
            exp: 0,
            promote_level: DEFAULT_WEAPON_PROMOTE_LEVEL,
            affix_map: HashMap::with_capacity(0),
            is_locked: false,
        },
    );
}
