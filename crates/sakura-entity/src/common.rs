use std::collections::HashMap;

use bevy_ecs::prelude::*;

use sakura_data::{
    excel::{
        avatar_curve_excel_config_collection, avatar_promote_excel_config_collection,
        common::PropGrowCurve, monster_curve_excel_config_collection,
        weapon_curve_excel_config_collection, AvatarExcelConfig, WeaponExcelConfig,
    },
    prop_type::FightPropType,
};
use sakura_proto::ProtEntityType;

use crate::fight_props;

#[derive(Component)]
pub struct Level(pub u32);

#[derive(Component)]
pub struct BreakLevel(pub u32);

#[derive(Component)]
pub struct Guid(pub u64);

#[derive(Component)]
pub struct OwnerPlayerUID(pub u32);

#[derive(Component)]
pub struct ProtocolEntityID(pub u32);

#[derive(Component)]
pub struct FightProperties(pub HashMap<FightPropType, f32>);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LifeState {
    Alive = 1,
    Dead = 2,
}

#[derive(Component)]
pub struct Visible;

#[derive(Component)]
pub struct ToBeRemovedMarker;

#[derive(Component)]
pub struct GadgetID(pub u32);

#[derive(Resource, Default)]
pub struct EntityCounter(u32);

impl EntityCounter {
    pub fn inc(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}

pub enum GrowCurveConfigType {
    Avatar,
    Monster,
}

impl FightProperties {
    pub fn apply_grow_curve(
        &mut self,
        level: u32,
        prop_grow_curve: &PropGrowCurve,
        config_type: GrowCurveConfigType,
    ) {
        let curve_info = match config_type {
            GrowCurveConfigType::Avatar => avatar_curve_excel_config_collection::iter()
                .find(|c| c.level == level)
                .and_then(|c| {
                    c.curve_infos
                        .iter()
                        .find(|c| c.grow_curve_type == prop_grow_curve.grow_curve)
                }),
            GrowCurveConfigType::Monster => monster_curve_excel_config_collection::iter()
                .find(|c| c.level == level)
                .and_then(|c| {
                    c.curve_infos
                        .iter()
                        .find(|c| c.grow_curve_type == prop_grow_curve.grow_curve)
                }),
        };

        if let Some(curve_info) = curve_info {
            let val = curve_info.apply(self.get_property(prop_grow_curve.ty));
            self.set_property(prop_grow_curve.ty, val);
        }
    }

    pub fn apply_base_values(&mut self) {
        use FightPropType::*;

        let base_hp = self.get_property(BaseHp);
        let base_attack = self.get_property(BaseAttack);
        let base_defense = self.get_property(BaseDefense);

        self.set_property(Hp, base_hp);
        self.set_property(MaxHp, base_hp);
        self.set_property(CurHp, base_hp);
        self.set_property(Attack, base_attack);
        self.set_property(CurAttack, base_attack);
        self.set_property(Defense, base_defense);
        self.set_property(CurDefense, base_defense);
    }

    pub fn get_property(&self, ty: FightPropType) -> f32 {
        self.0.get(&ty).copied().unwrap_or_default()
    }

    pub fn set_property(&mut self, ty: FightPropType, val: f32) {
        self.0.insert(ty, val);
    }

    pub fn change_property(&mut self, ty: FightPropType, delta: f32) {
        *self.0.entry(ty).or_default() += delta;
    }

    pub fn change_cur_hp(&mut self, delta: f32) {
        self.change_property(FightPropType::CurHp, delta);
        self.clamp_property(FightPropType::CurHp, FightPropType::MaxHp);
    }

    pub fn clamp_property(&mut self, ty: FightPropType, max_ty: FightPropType) {
        let max = self.0.get(&max_ty).copied().unwrap_or_default();
        let cur = self.0.entry(ty).or_default();

        *cur = cur.clamp(0.0, max);
    }
}

impl ProtocolEntityID {
    pub fn entity_type(&self) -> ProtEntityType {
        ProtEntityType::try_from((self.0 >> 24) as i32).unwrap_or_default()
    }
}

pub fn create_fight_props(
    config: &AvatarExcelConfig,
    cur_hp: f32,
    level: u32,
    break_level: u32,
) -> FightProperties {
    let mut props = fight_props! {
        BaseHp: config.hp_base,
        Hp: config.hp_base,
        BaseAttack: config.attack_base,
        Attack: config.attack_base,
        BaseDefense: config.defense_base,
        Defense: config.defense_base,
        CurHp: cur_hp,
        MaxHp: config.hp_base,
        CurAttack: config.attack_base,
        CurDefense: config.defense_base,
        ElementMastery: config.element_mastery,
        PhysicalAddHurt: config.physical_add_hurt,
        PhysicalSubHurt: config.physical_sub_hurt,
        AntiCritical: config.anti_critical,
        Critical: config.critical,
        CriticalHurt: config.critical_hurt,
        FireSubHurt: config.fire_sub_hurt,
        FireAddHurt: config.fire_add_hurt,
        GrassSubHurt: config.grass_sub_hurt,
        GrassAddHurt: config.grass_add_hurt,
        WaterSubHurt: config.water_sub_hurt,
        WaterAddHurt: config.water_add_hurt,
        ElecSubHurt: config.elec_sub_hurt,
        ElecAddHurt: config.elec_add_hurt,
        WindSubHurt: config.wind_sub_hurt,
        WindAddHurt: config.wind_add_hurt,
        IceSubHurt: config.ice_sub_hurt,
        IceAddHurt: config.ice_add_hurt,
        RockSubHurt: config.rock_sub_hurt,
        RockAddHurt: config.rock_add_hurt,
        CurWindEnergy: 100.0,
        CurGrassEnergy: 100.0,
        CurIceEnergy: 100.0,
        CurFireEnergy: 100.0,
        CurElecEnergy: 100.0,
        CurWaterEnergy: 100.0,
        CurRockEnergy: 100.0,
        MaxWindEnergy: 100.0,
        MaxGrassEnergy: 100.0,
        MaxIceEnergy: 100.0,
        MaxFireEnergy: 100.0,
        MaxElecEnergy: 100.0,
        MaxWaterEnergy: 100.0,
        MaxRockEnergy: 100.0
    };

    for prop_grow_curve in config.prop_grow_curves.iter() {
        props.apply_grow_curve(level, prop_grow_curve, GrowCurveConfigType::Avatar);
    }

    if let Some(promote_config) = avatar_promote_excel_config_collection::iter()
        .find(|c| c.avatar_promote_id == config.avatar_promote_id && c.promote_level == break_level)
    {
        for add_prop in promote_config.add_props.iter() {
            props.change_property(add_prop.prop_type, add_prop.value);
        }
    }

    props.apply_base_values();
    props
}

pub fn create_fight_props_with_weapon(
    config: &AvatarExcelConfig,
    cur_hp: f32,
    level: u32,
    break_level: u32,
    weapon: &WeaponExcelConfig,
    weapon_level: u32,
) -> FightProperties {
    let mut props = create_fight_props(config, cur_hp, level, break_level);
    add_fight_props_from_weapon(&mut props, weapon, weapon_level);
    props.apply_base_values();
    props
}

pub fn add_fight_props_from_weapon(
    props: &mut FightProperties,
    weapon: &WeaponExcelConfig,
    level: u32,
) {
    if let Some(weapon_curve_config) =
        weapon_curve_excel_config_collection::iter().find(|c| c.level == level)
    {
        for weapon_property in weapon.weapon_prop.iter() {
            if let Some(curve_info) = weapon_curve_config
                .curve_infos
                .iter()
                .find(|c| c.grow_curve_type == weapon_property.r#type)
            {
                let val = curve_info.apply(weapon_property.init_value);
                props.change_property(weapon_property.prop_type, val);
            }
        }
    }
}
