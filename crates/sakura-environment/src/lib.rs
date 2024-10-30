use bevy_app::prelude::*;
use chair::{avatar_lock_chair, ChairLockMap};

mod chair;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChairLockMap::default())
            .add_systems(PreUpdate, avatar_lock_chair);
    }
}
