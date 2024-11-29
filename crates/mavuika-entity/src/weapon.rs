use std::collections::HashMap;

use bevy_ecs::{prelude::*, query::QueryData};

use super::common::*;

#[derive(Component)]
pub struct WeaponID(pub u32);

#[derive(Component, Default)]
pub struct AffixMap(pub HashMap<u32, u32>);

#[derive(Component, Default)]
pub struct PromoteLevel(pub u32);

#[derive(Bundle)]
pub struct WeaponBundle {
    pub weapon_id: WeaponID,
    pub entity_id: ProtocolEntityID,
    pub guid: Guid,
    pub level: Level,
    pub gadget_id: GadgetID,
    pub promote_level: PromoteLevel,
    pub affix_map: AffixMap,
}

#[derive(QueryData)]
pub struct WeaponQueryReadOnly {
    pub weapon_id: &'static WeaponID,
    pub entity_id: &'static ProtocolEntityID,
    pub guid: &'static Guid,
    pub level: &'static Level,
    pub gadget_id: &'static GadgetID,
    pub promote_level: &'static PromoteLevel,
    pub affix_map: &'static AffixMap,
}
