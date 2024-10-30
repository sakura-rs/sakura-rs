use sakura_data_derive::FromBinary;

use super::common::PropGrowCurve;

#[derive(Debug, PartialEq, Default, FromBinary)]
pub enum AvatarUseType {
    #[default]
    Test = 0,
    SyncTest = 1,
    Formal = 2,
    Abandon = 3,
}

#[derive(Debug, Default, FromBinary)]
pub enum AvatarBodyType {
    #[default]
    None = 0,
    Boy = 1,
    Girl = 2,
    Lady = 3,
    Male = 4,
    Loli = 5,
}

#[derive(Debug, Default, FromBinary)]
pub enum QualityType {
    #[default]
    None = 0,
    White = 1,
    Green = 2,
    Blue = 3,
    Purple = 4,
    Orange = 5,
    OrangeSp = 105,
}

#[derive(Debug, Default, FromBinary)]
pub enum WeaponType {
    #[default]
    None = 0,
    SwordOneHand = 1,
    Crossbow = 2,
    Staff = 3,
    DoubleDagger = 4,
    Katana = 5,
    Shuriken = 6,
    Stick = 7,
    Spear = 8,
    ShieldSmall = 9,
    Catalyst = 10,
    Claymore = 11,
    Bow = 12,
    Pole = 13,
}

#[derive(Debug, Default, FromBinary)]
pub enum AvatarIdentityType {
    #[default]
    Master = 0,
    Normal = 1,
}

#[derive(Debug, FromBinary)]
pub struct AvatarExcelConfig {
    pub use_type: AvatarUseType,
    pub body_type: AvatarBodyType,
    pub script_data_path_hash: u64,
    pub icon_name: String,
    pub side_icon_name: String,
    pub quality_type: QualityType,
    pub charge_efficiency: f32,
    pub heal_add: f32,
    pub healed_add: f32,
    pub combat_config_hash: u64,
    pub is_range_attack: bool,
    pub initial_weapon: u32,
    pub weapon_type: WeaponType,
    pub manekin_path_hash: u64,
    pub image_name: String,
    pub gacha_card_name_hash: u64,
    pub gacha_image_name_hash: u64,
    pub coop_pic_name_hash: u64,
    pub cutscene_show: String,
    pub skill_depot_id: u32,
    pub stamina_recover_speed: f32,
    pub cand_skill_depot_ids: Vec<u32>,
    pub manekin_json_config_hash: u64,
    pub manekin_motion_config: u64,
    pub desc: u64,
    pub avatar_identity_type: AvatarIdentityType,
    pub avatar_promote_id: u32,
    pub avatar_promote_reward_level_list: Vec<u32>,
    pub avatar_promote_reward_id_list: Vec<u32>,
    pub feature_tag_group_id: u32,
    pub info_desc: u64,
    pub unk_1: u64,
    pub unk_2: u64,
    pub lnhcphodcah: u64,
    pub pnnhdfbldbl: u64,
    pub unk_3: u64,
    pub hp_base: f32,
    pub attack_base: f32,
    pub defense_base: f32,
    pub critical: f32,
    pub anti_critical: f32,
    pub critical_hurt: f32,
    pub fire_sub_hurt: f32,
    pub grass_sub_hurt: f32,
    pub water_sub_hurt: f32,
    pub elec_sub_hurt: f32,
    pub wind_sub_hurt: f32,
    pub ice_sub_hurt: f32,
    pub rock_sub_hurt: f32,
    pub fire_add_hurt: f32,
    pub grass_add_hurt: f32,
    pub water_add_hurt: f32,
    pub elec_add_hurt: f32,
    pub wind_add_hurt: f32,
    pub ice_add_hurt: f32,
    pub rock_add_hurt: f32,
    pub prop_grow_curves: Vec<PropGrowCurve>,
    pub element_mastery: f32,
    pub physical_sub_hurt: f32,
    pub physical_add_hurt: f32,
    pub prefab_path_ragdoll_hash: u64,
    pub deformation_mesh_path_hash: u64,
    pub id: u32,
    pub name_text_map_hash: u64,
    pub prefab_path_hash: u64,
    pub prefab_path_remote_hash: u64,
    pub controller_path_hash: u64,
    pub controller_path_remote_hash: u64,
    pub lod_pattern_name: String,
    pub unk_4: u64,
}
