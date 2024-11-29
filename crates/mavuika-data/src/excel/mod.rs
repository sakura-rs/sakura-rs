mod avatar_costume_excel_config;
mod avatar_curve_excel_config;
mod avatar_excel_config;
mod avatar_flycloak_excel_config;
mod avatar_promote_excel_config;
mod avatar_skill_depot_excel_config;
mod avatar_trace_effect_excel_config;
mod map_area_config;
mod monster_curve_excel_config;
mod monster_excel_config;
mod open_state_config;
mod weapon_curve_excel_config;
mod weapon_excel_config;

pub mod common;
pub use avatar_costume_excel_config::*;
pub use avatar_curve_excel_config::*;
pub use avatar_excel_config::*;
pub use avatar_flycloak_excel_config::*;
pub use avatar_promote_excel_config::*;
pub use avatar_skill_depot_excel_config::*;
pub use avatar_trace_effect_excel_config::*;
pub use map_area_config::*;
pub use monster_curve_excel_config::*;
pub use monster_excel_config::*;
pub use open_state_config::*;
use paste::paste;
pub use weapon_curve_excel_config::*;
pub use weapon_excel_config::*;

macro_rules! excel_loader {
    ($($name:ident;)*) => {
        $(paste! {
            pub mod [<$name:snake _collection>] {
                pub const PATH: &str = stringify!([<$name Data>]);

                static DATA: ::std::sync::OnceLock<Vec<super::$name>> = ::std::sync::OnceLock::new();
                pub fn load_from_binary<R: ::std::io::Read + ::std::io::Seek>(r: &mut R) -> std::io::Result<()> {
                    use crate::FromBinary;

                    let entries_count = i32::from_binary(r)? as usize;
                    let mut data = Vec::with_capacity(entries_count);

                    for _ in 0..entries_count {
                        data.push(super::$name::from_binary(r)?);
                    }

                    let _ = DATA.set(data);
                    Ok(())
                }

                pub fn iter() -> ::std::slice::Iter<'static, super::$name> {
                    DATA.get().unwrap().iter()
                }
            }
        })*

        pub fn load_all(excel_bin_output_path: &str) -> ::std::io::Result<()> {
            $(paste!{
                let data = ::std::fs::read(&format!("{excel_bin_output_path}/{}", [<$name:snake _collection>]::PATH))?;
                let mut r = ::std::io::Cursor::new(&data);
                [<$name:snake _collection>]::load_from_binary(&mut r)?;
            })*

            Ok(())
        }
    };
}

excel_loader! {
    AvatarExcelConfig;
    AvatarCostumeExcelConfig;
    AvatarTraceEffectExcelConfig;
    AvatarFlycloakExcelConfig;
    AvatarSkillDepotExcelConfig;
    AvatarCurveExcelConfig;
    AvatarPromoteExcelConfig;
    WeaponExcelConfig;
    WeaponCurveExcelConfig;
    OpenStateConfig;
    MonsterExcelConfig;
    MonsterCurveExcelConfig;
    MapAreaConfig;
}
