use bevy_ecs::prelude::*;
use sakura_entity::common::{FightProperties, Guid, OwnerPlayerUID, ProtocolEntityID};
use sakura_message::output::MessageOutput;
use sakura_proto::{
    AttackResult, AvatarFightPropUpdateNotify, EntityFightPropUpdateNotify, ProtEntityType,
};
use tracing::{debug, instrument};

#[derive(Event)]
pub struct EntityBeingHitEvent(pub u32, pub AttackResult);

#[instrument(skip_all)]
pub fn deal_damage_on_hit(
    mut events: EventReader<EntityBeingHitEvent>,
    mut entities: Query<(
        &mut FightProperties,
        &ProtocolEntityID,
        Option<&OwnerPlayerUID>,
    )>,
) {
    for EntityBeingHitEvent(originator_uid, attack_result) in events.read() {
        if (attack_result.attacker_id >> 24) < ProtEntityType::Max as u32 {
            let Some((_, _, attacker_owner)) = entities
                .iter()
                .find(|(_, id, __)| id.0 == attack_result.attacker_id)
            else {
                debug!("attacker with id {} not found", attack_result.attacker_id);
                continue;
            };

            if let Some(owner_uid) = attacker_owner {
                if owner_uid.0 != *originator_uid {
                    debug!(
                        "fail: entity owner uid mismatch! owner uid: {}, event originator uid: {}",
                        owner_uid.0, originator_uid
                    );
                    continue;
                }
            }
        }

        let Some((mut defender_props, _, _)) = entities
            .iter_mut()
            .find(|(_, id, _)| id.0 == attack_result.defense_id)
        else {
            debug!("defender with id {} not found", attack_result.defense_id);
            continue;
        };

        defender_props.change_cur_hp(-attack_result.damage);
        debug!(
            "attacker (id: {}) dealt {} dmg to defender (id: {})",
            attack_result.attacker_id, attack_result.damage, attack_result.defense_id
        );
    }
}

pub fn notify_fight_properties_to_clients(
    changed_properties: Query<
        (&FightProperties, &ProtocolEntityID, Option<&Guid>),
        Changed<FightProperties>,
    >,
    message_output: Res<MessageOutput>,
) {
    for (properties, entity_id, guid) in changed_properties.iter() {
        let fight_prop_map = properties
            .0
            .iter()
            .map(|(ty, val)| (*ty as u32, *val))
            .collect();

        if entity_id.entity_type() == ProtEntityType::Avatar {
            message_output.send_to_all(AvatarFightPropUpdateNotify {
                avatar_guid: guid.map(|g| g.0).unwrap_or_default(),
                fight_prop_map,
            });
        } else {
            message_output.send_to_all(EntityFightPropUpdateNotify {
                entity_id: entity_id.0,
                fight_prop_map,
            });
        }
    }
}
