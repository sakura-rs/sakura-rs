use bevy_ecs::prelude::*;
use mavuika_data::{
    excel::{avatar_excel_config_collection, weapon_excel_config_collection},
    prop_type::FightPropType,
};
use mavuika_entity::{
    avatar::{AvatarEquipChangeEvent, AvatarQueryReadOnly, Equipment},
    common::{
        create_fight_props_with_weapon, EntityCounter, FightProperties, GadgetID, Guid, Level,
        OwnerPlayerUID, ProtocolEntityID, ToBeRemovedMarker,
    },
    util::to_protocol_entity_id,
    weapon::{AffixMap, PromoteLevel, WeaponBundle, WeaponID, WeaponQueryReadOnly},
};
use mavuika_message::output::MessageOutput;
use mavuika_persistence::{player_information::ItemInformation, Players};
use mavuika_proto::{
    AbilitySyncStateInfo, AvatarEquipChangeNotify, EntityRendererChangedInfo, ProtEntityType,
    SceneWeaponInfo,
};

pub fn notify_avatar_equip_change(
    avatars: Query<AvatarQueryReadOnly, Changed<Equipment>>,
    weapons: Query<WeaponQueryReadOnly>,
    message_output: Res<MessageOutput>,
) {
    for avatar_data in avatars.iter() {
        let weapon_data = weapons.get(avatar_data.equipment.weapon).unwrap();

        message_output.send_to_all(AvatarEquipChangeNotify {
            avatar_guid: avatar_data.guid.0,
            equip_guid: weapon_data.guid.0,
            item_id: weapon_data.weapon_id.0,
            equip_type: 6,
            weapon: Some(SceneWeaponInfo {
                guid: weapon_data.guid.0,
                entity_id: weapon_data.entity_id.0,
                gadget_id: weapon_data.gadget_id.0,
                item_id: weapon_data.weapon_id.0,
                level: weapon_data.level.0,
                promote_level: weapon_data.promote_level.0,
                affix_map: weapon_data.affix_map.0.clone(),
                ability_info: Some(AbilitySyncStateInfo::default()),
                renderer_changed_info: Some(EntityRendererChangedInfo::default()),
            }),
            reliquary: None,
        })
    }
}

pub fn apply_equip_change_to_avatar_entity(
    mut events: EventReader<AvatarEquipChangeEvent>,
    mut commands: Commands,
    mut avatars: Query<(
        &ProtocolEntityID,
        &Guid,
        &OwnerPlayerUID,
        &mut Equipment,
        &mut FightProperties,
    )>,
    mut entity_counter: ResMut<EntityCounter>,
    players: Res<Players>,
) {
    for avatar_equip_change in events.read() {
        let Some((_, _, _, mut equipment, mut fight_props)) =
            avatars.iter_mut().find(|(_, guid, owner_uid, _, _)| {
                owner_uid.0 == avatar_equip_change.player_uid
                    && guid.0 == avatar_equip_change.avatar_guid
            })
        else {
            continue;
        };

        commands.entity(equipment.weapon).insert(ToBeRemovedMarker);

        let player_info = players.get(avatar_equip_change.player_uid);
        let avatar = player_info
            .avatar_module
            .avatar_map
            .get(&avatar_equip_change.avatar_guid)
            .unwrap();

        let ItemInformation::Weapon {
            weapon_id,
            level,
            exp: _,
            promote_level,
            affix_map,
            is_locked: _,
        } = player_info
            .item_map
            .get(&avatar_equip_change.weapon_guid)
            .unwrap();

        let weapon_config = weapon_excel_config_collection::iter()
            .find(|cfg| cfg.id == *weapon_id)
            .unwrap();

        let weapon_entity = commands
            .spawn(WeaponBundle {
                weapon_id: WeaponID(*weapon_id),
                entity_id: to_protocol_entity_id(ProtEntityType::Weapon, entity_counter.inc()),
                level: Level(*level),
                guid: Guid(avatar_equip_change.weapon_guid),
                gadget_id: GadgetID(weapon_config.gadget_id),
                affix_map: AffixMap(affix_map.clone()),
                promote_level: PromoteLevel(*promote_level),
            })
            .id();

        equipment.weapon = weapon_entity;

        let cur_hp = fight_props.get_property(FightPropType::CurHp);
        *fight_props = create_fight_props_with_weapon(
            avatar_excel_config_collection::iter()
                .find(|conf| conf.id == avatar.avatar_id)
                .unwrap(),
            cur_hp,
            avatar.level,
            avatar.break_level,
            weapon_config,
            *level,
        );
    }
}
