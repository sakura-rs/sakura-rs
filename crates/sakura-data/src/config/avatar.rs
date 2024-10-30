use std::io::SeekFrom;

use byteorder::ReadBytesExt;
use varint_rs::VarintReader;

use crate::FromBinary;

#[derive(Debug)]
pub struct AvatarConfig {
    pub abilities: Vec<AvatarAbility>,
}

#[derive(Debug)]
pub struct AvatarAbility {
    pub ability_id: String,
    pub ability_name: String,
    pub ability_override: String,
}

impl FromBinary for AvatarConfig {
    fn from_binary<R: std::io::Read + std::io::Seek>(r: &mut R) -> std::io::Result<Self> {
        // Skip to ability list

        let mut abilities_count;
        let mut prev_byte = 0;
        loop {
            let mut b = [0u8; 7];
            let v = r.read_u8()?;
            if v == 7 {
                r.read_exact(&mut b)?;
                if &b[..6] == b"equip0" {
                    for _ in 0..((prev_byte * 2) - 1) {
                        let l = r.read_usize_varint()? as i64;
                        r.seek(SeekFrom::Current(l))?;
                    }

                    abilities_count = r.read_usize_varint()?;
                    if abilities_count == 1 {
                        match r.read_u32_varint()? {
                            0x1B => {
                                // Extra case #1: weaponAwayFromHandStates
                                _ = String::from_binary(r)?;
                                _ = f32::from_binary(r)?;
                                _ = f32::from_binary(r)?;
                                _ = f32::from_binary(r)?;
                            }
                            0x07 => {
                                // Extra case #2: subEquipController
                                _ = String::from_binary(r)?;
                                _ = f32::from_binary(r)?;
                                _ = f32::from_binary(r)?;
                                _ = r.read_u8()?;
                                _ = String::from_binary(r)?;
                                _ = String::from_binary(r)?;
                                _ = u32::from_binary(r)?;
                            }
                            unk => todo!("unknown type between equip and abilities: {unk}"),
                        }

                        abilities_count = r.read_usize_varint()?;
                    }

                    break;
                }
            } else {
                prev_byte = v;
            }
        }

        let mut abilities = Vec::with_capacity(abilities_count);

        for _ in 0..abilities_count {
            abilities.push(AvatarAbility::from_binary(r)?);
        }

        Ok(Self { abilities })
    }
}

impl AvatarAbility {
    pub const TYPE_IDENTIFIER: u32 = 7;
    pub const DEFAULT_OVERRIDE: &str = "Default";
}

impl FromBinary for AvatarAbility {
    fn from_binary<R: std::io::Read + std::io::Seek>(r: &mut R) -> std::io::Result<Self> {
        assert_eq!(r.read_u32_varint()?, Self::TYPE_IDENTIFIER);

        Ok(Self {
            ability_id: String::from_binary(r)?,
            ability_name: String::from_binary(r)?,
            ability_override: String::from_binary(r).map(|s| {
                if s.is_empty() {
                    String::from(Self::DEFAULT_OVERRIDE)
                } else {
                    s
                }
            })?,
        })
    }
}
