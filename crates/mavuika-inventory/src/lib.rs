use bevy_app::prelude::*;

mod equip;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, equip::change_avatar_equip);
    }
}
