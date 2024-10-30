use bevy_ecs::prelude::*;

use crate::{
    ability::Ability,
    common::{OwnerPlayerUID, ProtocolEntityID},
};

#[derive(Bundle)]
pub struct PlayTeamEntityBundle {
    pub marker: PlayTeamEntityMarker,
    pub player_uid: OwnerPlayerUID,
    pub entity_id: ProtocolEntityID,
    pub ability: Ability,
}

#[derive(Component)]
pub struct PlayTeamEntityMarker;
