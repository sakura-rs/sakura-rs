use bevy_ecs::prelude::*;

use crate::common::ProtocolEntityID;

#[derive(Component)]
pub struct AuthorityPeerId(pub u32);

#[derive(Component)]
pub struct MpLevelEntityMarker;

#[derive(Bundle)]
pub struct MpLevelBundle {
    pub authority_peer_id: AuthorityPeerId,
    pub entity_id: ProtocolEntityID,
    pub marker: MpLevelEntityMarker,
}
