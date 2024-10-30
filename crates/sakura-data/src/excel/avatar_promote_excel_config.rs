use sakura_data_derive::FromBinary;

use super::common::{IdCountConfig, PropValConfig};

#[derive(Debug, FromBinary)]
pub struct AvatarPromoteExcelConfig {
    pub avatar_promote_id: u32,
    pub promote_level: u32,
    pub promote_audio: String,
    pub scoin_cost: u32,
    pub cost_items: Vec<IdCountConfig>,
    pub unlock_max_level: u32,
    pub add_props: Vec<PropValConfig>,
    pub required_player_level: u32,
}
