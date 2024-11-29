use bevy_ecs::prelude::*;

use crate::common::ProtocolEntityID;

use super::ability::Ability;

#[derive(Bundle)]
pub struct TeamEntityBundle {
    pub marker: TeamEntityMarker,
    pub entity_id: ProtocolEntityID,
    pub ability: Ability,
}

#[derive(Component)]
pub struct TeamEntityMarker;
