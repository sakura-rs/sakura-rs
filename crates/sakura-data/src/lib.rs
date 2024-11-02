mod from_binary;
pub use from_binary::FromBinary;

mod util;

pub mod config;
pub mod excel;
pub mod prop_type;

#[cfg(test)]
mod tests {
    use std::{fs, io::Cursor, path::Path};

    use crate::config::AvatarConfig;
    use crate::FromBinary;
    use paste::paste;

    macro_rules! excel_test {
        ($($name:ident),*) => {
            $(paste! {
                #[test]
                pub fn [<decode_ $name:snake>]() -> std::io::Result<()> {
                    let data = fs::read(Path::new(
                        &format!("../../assets/ExcelBinOutput/{}Data", stringify!($name)),
                    ))
                    .unwrap();
                    let mut r = Cursor::new(&data);

                    let count = i32::from_binary(&mut r)? as usize;
                    for _ in 0..count {
                        let _ = crate::excel::$name::from_binary(&mut r)?;
                    }

                    Ok(())
                }
            })*
        };
    }

    #[allow(unused_macros)] // useful when defining new one
    macro_rules! excel_test_verbose {
        ($($name:ident),*) => {
            $(paste! {
                #[test]
                pub fn [<decode_ $name:snake>]() -> std::io::Result<()> {
                    let data = fs::read(Path::new(
                        &format!("../../assets/ExcelBinOutput/{}Data", stringify!($name)),
                    ))
                    .unwrap();
                    let mut r = Cursor::new(&data);

                    let count = i32::from_binary(&mut r)? as usize;
                    for _ in 0..count {
                        let cfg = crate::excel::$name::from_binary(&mut r)?;
                        println!("{cfg:?}");
                    }

                    Ok(())
                }
            })*
        };
    }

    #[test]
    pub fn decode_avatar_config() -> std::io::Result<()> {
        // HIGHLY EXPERIMENTAL!

        for p in fs::read_dir(Path::new("../../assets/BinOutput/Avatar/")).unwrap() {
            let path = p?.path();

            let data = fs::read(path)?;
            let mut r = Cursor::new(&data);

            let _ = AvatarConfig::from_binary(&mut r)?;
        }

        Ok(())
    }

    excel_test!(
        AvatarExcelConfig,
        AvatarFlycloakExcelConfig,
        AvatarSkillDepotExcelConfig,
        AvatarPromoteExcelConfig,
        AvatarCurveExcelConfig,
        MonsterExcelConfig,
        MonsterCurveExcelConfig,
        WeaponExcelConfig,
        OpenStateConfig,
        MapAreaConfig
    );
}
