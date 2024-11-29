use mavuika_data_derive::FromBinary;

use crate::prop_type::FightPropType;

#[repr(i32)]
#[derive(Default, FromBinary, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrowCurveType {
    #[default]
    None = 0,
    Hp = 1,
    Attack = 2,
    Stamina = 3,
    Strike = 4,
    AntiStrike = 5,
    AntiStrike1 = 6,
    AntiStrike2 = 7,
    AntiStrike3 = 8,
    StrikeHurt = 9,
    Element = 10,
    KillExp = 11,
    Defense = 12,
    AttackBomb = 13,
    HpLittlemonster = 14,
    ElementMastery = 15,
    Progression = 16,
    Defending = 17,
    Mhp = 18,
    Matk = 19,
    Toweratk = 20,
    HpS5 = 21,
    HpS4 = 22,
    Hp2 = 23,
    Attack2 = 24,
    HpEnvironment = 25,
    AttackS5 = 31,
    AttackS4 = 32,
    AttackS3 = 33,
    StrikeS5 = 34,
    DefenseS5 = 41,
    DefenseS4 = 42,
    Attack101 = 1101,
    Attack102 = 1102,
    Attack103 = 1103,
    Attack104 = 1104,
    Attack105 = 1105,
    Attack201 = 1201,
    Attack202 = 1202,
    Attack203 = 1203,
    Attack204 = 1204,
    Attack205 = 1205,
    Attack301 = 1301,
    Attack302 = 1302,
    Attack303 = 1303,
    Attack304 = 1304,
    Attack305 = 1305,
    Critical101 = 2101,
    Critical102 = 2102,
    Critical103 = 2103,
    Critical104 = 2104,
    Critical105 = 2105,
    Critical201 = 2201,
    Critical202 = 2202,
    Critical203 = 2203,
    Critical204 = 2204,
    Critical205 = 2205,
    Critical301 = 2301,
    Critical302 = 2302,
    Critical303 = 2303,
    Critical304 = 2304,
    Critical305 = 2305,
    ActivityAttack1 = 5201,
    ActivityHp1 = 5202,
}

#[derive(Default, Debug, FromBinary, Clone, Copy)]
pub enum GrowCurveArith {
    #[default]
    None = 0,
    Add = 1,
    Multi = 2,
    Sub = 3,
    Divide = 4,
}

#[derive(Debug, FromBinary)]
pub struct IdCountConfig {
    pub id: u32,
    pub count: u32,
}

#[derive(Debug, FromBinary)]
pub struct PropValConfig {
    pub prop_type: FightPropType,
    pub value: f32,
}

#[derive(Debug, FromBinary)]
pub struct PropGrowCurve {
    pub ty: FightPropType,
    pub grow_curve: GrowCurveType,
}

#[derive(Debug, FromBinary)]
pub struct GrowCurveInfo {
    pub grow_curve_type: GrowCurveType,
    pub arith: GrowCurveArith,
    pub value: f32,
}

impl GrowCurveInfo {
    pub fn apply(&self, val: f32) -> f32 {
        use GrowCurveArith::*;
        match self.arith {
            None => val,
            Add => val + self.value,
            Multi => val * self.value,
            Sub => val - self.value,
            Divide => val / self.value,
        }
    }
}
