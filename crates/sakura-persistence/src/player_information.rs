// player persistent 'Data' definitions

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayerInformation {
    pub uid: u32,
    pub nick_name: String,
    pub guid_counter: u32,
    pub basic_module: BasicModuleInformation,
    pub avatar_module: AvatarModuleInformation,
    pub item_map: HashMap<u64, ItemInformation>,
    pub world_position: PlayerPositionInformation,
}

impl PlayerInformation {
    pub fn next_guid(&mut self) -> u64 {
        self.guid_counter += 1;
        ((self.uid as u64) << 32) | self.guid_counter as u64
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct BasicModuleInformation {
    pub level: u32,
    pub exp: u32,
    pub is_game_time_locked: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AvatarModuleInformation {
    pub avatar_map: HashMap<u64, AvatarInformation>,
    pub team_map: HashMap<u32, AvatarTeamInformation>,
    pub owned_flycloak_set: HashSet<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct AvatarTeamInformation {
    pub avatar_guid_list: Vec<u64>,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct AvatarInformation {
    pub avatar_id: u32,
    pub level: u32,
    pub break_level: u32,
    pub skill_depot_id: u32,
    pub guid: u64,
    pub born_time: u32,
    pub weapon_guid: u64,
    pub cur_hp: f32,
    pub skill_level_map: HashMap<u32, u32>,
    pub inherent_proud_skill_list: Vec<u32>,
    pub wearing_flycloak_id: u32,
}

#[derive(Serialize, Deserialize)]
pub enum ItemInformation {
    Weapon {
        weapon_id: u32,
        level: u32,
        exp: u32,
        promote_level: u32,
        affix_map: HashMap<u32, u32>,
        is_locked: bool,
    },
}

#[derive(Serialize, Deserialize)]
pub struct PlayerPositionInformation {
    pub scene_id: u32,
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
}
