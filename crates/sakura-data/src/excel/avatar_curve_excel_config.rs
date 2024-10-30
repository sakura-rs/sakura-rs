use sakura_data_derive::FromBinary;

use super::common::GrowCurveInfo;

#[derive(Debug, FromBinary)]
pub struct AvatarCurveExcelConfig {
    pub level: u32,
    pub curve_infos: Vec<GrowCurveInfo>,
}
