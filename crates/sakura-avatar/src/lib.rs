use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

mod appearance;
mod equip;

pub struct AvatarPlugin;

impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, appearance::handle_appearance_change_request)
            .add_systems(
                Update,
                (
                    equip::apply_equip_change_to_avatar_entity,
                    equip::notify_avatar_equip_change,
                )
                    .chain(),
            );
    }
}
