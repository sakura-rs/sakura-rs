use std::collections::HashMap;

use bevy_ecs::component::Component;
use common::string_util;
use mavuika_data::{config, excel::avatar_excel_config_collection};
use mavuika_proto::{AbilityControlBlock, AbilityEmbryo};
use tracing::warn;

#[derive(Component, Default)]
pub struct Ability {
    pub target_ability_map: HashMap<u32, AbilityData>,
}

pub struct AbilityData {
    pub ability_name_hash: u32,
    pub ability_override_name_hash: u32,
}

const COMMON_AVATAR_ABILITIES: [&str; 10] = [
    "Avatar_DefaultAbility_VisionReplaceDieInvincible",
    "Avatar_DefaultAbility_AvartarInShaderChange",
    "Avatar_SprintBS_Invincible",
    "Avatar_Freeze_Duration_Reducer",
    "Avatar_Attack_ReviveEnergy",
    "Avatar_Component_Initializer",
    "Avatar_HDMesh_Controller",
    "Avatar_Trampoline_Jump_Controller",
    "Avatar_ArkheGrade_CD_Controller",
    "Avatar_TriggerNyxInstant",
];

impl Ability {
    pub fn new_for_avatar(id: u32) -> Self {
        let avatar = avatar_excel_config_collection::iter()
            .find(|a| a.id == id)
            .unwrap();
        let name = avatar.icon_name.replace("UI_AvatarIcon_", "");

        if let Some(config) = config::get_avatar_config(&name) {
            Self {
                target_ability_map: COMMON_AVATAR_ABILITIES
                    .iter()
                    .map(|name| AbilityData::new(name, "Default"))
                    .chain(config.abilities.iter().map(|ability| {
                        AbilityData::new(&ability.ability_name, &ability.ability_override)
                    }))
                    .enumerate()
                    .map(|(idx, data)| (idx as u32 + 1, data))
                    .collect(),
            }
        } else {
            warn!("missing ConfigAvatar for {name}");

            Self {
                target_ability_map: HashMap::with_capacity(0),
            }
        }
    }

    pub fn new_for_team() -> Self {
        Self {
            target_ability_map: HashMap::from([(
                1,
                AbilityData::new("TeamAbility_Reset_Crystal_Mark", "Default"),
            )]),
        }
    }

    pub fn build_control_block(&self) -> AbilityControlBlock {
        AbilityControlBlock {
            ability_embryo_list: self
                .target_ability_map
                .iter()
                .map(|(id, data)| AbilityEmbryo {
                    ability_id: *id,
                    ability_name_hash: data.ability_name_hash,
                    ability_override_name_hash: data.ability_override_name_hash,
                })
                .collect(),
        }
    }
}

impl AbilityData {
    pub fn new(name: &str, override_name: &str) -> Self {
        Self {
            ability_name_hash: string_util::get_string_hash(name),
            ability_override_name_hash: string_util::get_string_hash(override_name),
        }
    }
}
