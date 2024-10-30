use avatar::AvatarEquipChangeEvent;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::{EntityCounter, FightProperties, ProtocolEntityID, ToBeRemovedMarker};
use sakura_data::prop_type::FightPropType;
use sakura_message::output::MessageOutput;

pub mod ability;
pub mod avatar;
pub mod common;
pub mod monster;
pub mod mp_level;
pub mod play_team;
pub mod team;
pub mod transform;
pub mod util;
pub mod weapon;

pub use sakura_proto::ProtEntityType;
use sakura_proto::{LifeStateChangeNotify, VisionType};

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntityCounter::default())
            .add_event::<AvatarEquipChangeEvent>()
            .add_event::<EntityDisappearEvent>()
            .add_systems(
                Last,
                (
                    update_entity_life_state,
                    notify_disappear_entities,
                    remove_marked_entities,
                    avatar::notify_appear_avatar_entities
                        .run_if(avatar::run_if_avatar_entities_appeared),
                    monster::notify_appear_monster_entities
                        .run_if(monster::run_if_monster_entities_appeared),
                )
                    .chain(),
            );
    }
}

#[derive(Event)]
pub struct EntityDisappearEvent(pub u32, pub VisionType);

fn update_entity_life_state(
    mut commands: Commands,
    entities: Query<(Entity, &ProtocolEntityID, &FightProperties), Changed<FightProperties>>,
    mut disappear_events: EventWriter<EntityDisappearEvent>,
    message_output: Res<MessageOutput>,
) {
    for (entity, id, fight_props) in entities.iter() {
        if fight_props.get_property(FightPropType::CurHp) <= 0.0 {
            commands.entity(entity).insert(ToBeRemovedMarker);
            disappear_events.send(EntityDisappearEvent(id.0, VisionType::Die));

            message_output.send_to_all(LifeStateChangeNotify {
                entity_id: id.0,
                life_state: 2,
                ..Default::default()
            });
        }
    }
}

fn notify_disappear_entities(
    mut events: EventReader<EntityDisappearEvent>,
    message_output: Res<MessageOutput>,
) {
    use sakura_proto::*;

    events
        .read()
        .for_each(|EntityDisappearEvent(id, disappear_type)| {
            message_output.send_to_all(SceneEntityDisappearNotify {
                disappear_type: disappear_type.clone().into(),
                param: 0,
                entity_list: vec![*id],
            })
        });
}

fn remove_marked_entities(
    mut commands: Commands,
    entities: Query<Entity, With<ToBeRemovedMarker>>,
) {
    entities
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());
}
