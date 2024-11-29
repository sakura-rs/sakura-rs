use mavuika_data_derive::FromBinary;

use crate::prop_type::FightPropType;

use super::common::GrowCurveType;

#[derive(Debug, FromBinary)]
pub struct WeaponProperty {
    pub prop_type: FightPropType,
    pub init_value: f32,
    pub r#type: GrowCurveType,
}

#[derive(Debug, FromBinary)]
pub struct WeaponExcelConfig {
    pub weapon_type: i32,
    pub rank_level: u32,
    pub material_type: i32,
    pub elem_type: i32,
    pub is_gold: bool,
    pub weapon_base_exp: u32,
    pub skill_affix: Vec<u32>,
    pub awaken_material: u32,
    pub weapon_prop: Vec<WeaponProperty>,
    pub awaken_texture: String,
    pub awaken_light_map_texture: String,
    pub awaken_icon: String,
    pub un_rotate: bool,
    pub weapon_promote_id: u32,
    pub story_id: u32,
    pub awaken_costs: Vec<u32>,
    pub gacha_card_name_hash: u64,
    pub enhance_rule: u32,
    pub destroy_rule: u32,
    pub destroy_return_material: Vec<u32>,
    pub destroy_return_material_count: Vec<u32>,
    pub initial_lock_state: u32,
    pub id: u32,
    pub name: u64,
    pub desc: u64,
    pub icon: String,
    pub item_type: i32,
    pub weight: u32,
    pub rank: u32,
    pub gadget_id: u32,
    pub droppable: bool,
    pub use_level: u32,
    pub global_item_limit: u32,
}
