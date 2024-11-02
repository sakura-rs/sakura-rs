use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use rand::RngCore;
use sakura_data::excel::monster_excel_config_collection;
use sakura_entity::{
    common::{EntityCounter, GrowCurveConfigType, Level, LifeState, Visible},
    monster::{MonsterBundle, MonsterID},
    transform::{Transform, Vector3},
    util::to_protocol_entity_id,
    ProtEntityType,
};
use sakura_persistence::Players;
use sakura_scene::ScenePlayerJumpEvent;
use tracing::{debug, instrument};
use util::create_fight_properties_by_monster_config;

mod util;

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DebugCommandEvent>()
            .add_systems(Update, debug_command_handler);
    }
}

#[derive(Event)]
pub struct DebugCommandEvent {
    pub executor_uid: u32,
    pub kind: CommandKind,
}

#[derive(Debug)]
pub enum CommandKind {
    SpawnMonster {
        monster_id: Option<u32>,
        position: (f32, f32),
    },
    QuickTravel {
        position: (f32, Option<f32>, f32),
    },
}

#[instrument(skip_all)]
pub fn debug_command_handler(
    mut events: EventReader<DebugCommandEvent>,
    mut commands: Commands,
    mut entity_counter: ResMut<EntityCounter>,
    players: Res<Players>,
    mut jump_events: EventWriter<ScenePlayerJumpEvent>,
) {
    for command in events.read() {
        debug!(
            "executor_uid: {}, kind: {:?}",
            command.executor_uid, command.kind
        );

        let player = players.get(command.executor_uid);

        match command.kind {
            CommandKind::SpawnMonster {
                monster_id,
                position,
            } => {
                // spawn random slime if not specified
                let monster_id = monster_id.unwrap_or_else(|| {
                    [20010101, 20010302, 20010502, 20010803, 20011002]
                        [rand::thread_rng().next_u32() as usize % 5]
                });
                let Some(config) =
                    monster_excel_config_collection::iter().find(|cfg| cfg.id == monster_id)
                else {
                    debug!("monster config for id {monster_id} not found");
                    continue;
                };

                let level = 90;

                let mut fight_properties = create_fight_properties_by_monster_config(config);
                for grow_curve in config.prop_grow_curves.iter() {
                    fight_properties.apply_grow_curve(
                        level,
                        grow_curve,
                        GrowCurveConfigType::Monster,
                    );
                }
                fight_properties.apply_base_values();

                commands
                    .spawn(MonsterBundle {
                        monster_id: MonsterID(monster_id),
                        entity_id: to_protocol_entity_id(
                            ProtEntityType::Monster,
                            entity_counter.inc(),
                        ),
                        level: Level(level),
                        transform: Transform {
                            // Take Y (height) from player's pos, spawn a bit above
                            position: (
                                position.0,
                                player.world_position.position.1 + 10.0,
                                position.1,
                            )
                                .into(),
                            rotation: Vector3::default(),
                        },
                        fight_properties,
                        life_state: LifeState::Alive,
                    })
                    .insert(Visible);
            }
            CommandKind::QuickTravel { position } => {
                let destination =
                    Vector3::from((position.0, position.1.unwrap_or(2000.0), position.2));
                jump_events.send(ScenePlayerJumpEvent(command.executor_uid, destination));
            }
        }
    }
}
