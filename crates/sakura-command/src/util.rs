use sakura_data::excel::MonsterExcelConfig;
use sakura_entity::{common::FightProperties, fight_props};

pub fn create_fight_properties_by_monster_config(config: &MonsterExcelConfig) -> FightProperties {
    fight_props! {
        Hp: config.hp_base,
        BaseHp: config.hp_base,
        CurHp: config.hp_base,
        MaxHp: config.hp_base,
        Attack: config.attack_base,
        BaseAttack: config.attack_base,
        CurAttack: config.attack_base,
        Defense: config.defense_base,
        BaseDefense: config.defense_base,
        CurDefense: config.defense_base,
        ElementMastery: config.element_mastery,
        Critical: config.critical,
        CriticalHurt: config.critical_hurt,
        AntiCritical: config.anti_critical,
        PhysicalSubHurt: config.physical_sub_hurt,
        PhysicalAddHurt: config.physical_add_hurt,
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
        RockAddHurt: config.rock_add_hurt
    }
}
