use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use fight::{deal_damage_on_hit, notify_fight_properties_to_clients, EntityBeingHitEvent};
use sakura_message::event::ClientMessageEvent;
use sakura_proto::{CombatInvocationsNotify, EntityMoveInfo, EvtBeingHitInfo, Protobuf};
use movement::{entity_movement, track_player_position, EntityMoveEvent};
use tracing::instrument;

mod fight;
mod movement;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityMoveEvent>()
            .add_event::<EntityBeingHitEvent>()
            .add_systems(PreUpdate, combat_invocation_processor)
            .add_systems(Update, entity_movement)
            .add_systems(Update, deal_damage_on_hit)
            .add_systems(PostUpdate, track_player_position)
            .add_systems(PostUpdate, notify_fight_properties_to_clients);
    }
}

#[instrument(skip_all)]
fn combat_invocation_processor(
    mut events: EventReader<ClientMessageEvent>,
    mut movement_events: EventWriter<EntityMoveEvent>,
    mut hit_events: EventWriter<EntityBeingHitEvent>,
) {
    for message in events.read() {
        if let Some(notify) = message.decode::<CombatInvocationsNotify>() {
            for invoke in notify.invoke_list {
                use sakura_proto::CombatTypeArgument::*;

                match invoke.argument_type() {
                    EntityMove => {
                        if let Ok(info) = EntityMoveInfo::decode(invoke.combat_data.as_ref()) {
                            movement_events.send(EntityMoveEvent(message.sender_uid(), info));
                        }
                    }
                    EvtBeingHit => {
                        if let Ok(info) = EvtBeingHitInfo::decode(invoke.combat_data.as_ref()) {
                            if let Some(attack_result) = info.attack_result {
                                hit_events
                                    .send(EntityBeingHitEvent(message.sender_uid(), attack_result));
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
