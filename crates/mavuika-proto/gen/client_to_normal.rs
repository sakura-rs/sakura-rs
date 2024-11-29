impl From<crate::client::MarkMapRsp> for MarkMapRsp {
    fn from(value: crate::client::MarkMapRsp) -> Self {
        Self {
            mark_list: value.mark_list.into_iter().map(|v| v.into()).collect(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::WeeklyBossResinDiscountInfo> for WeeklyBossResinDiscountInfo {
    fn from(value: crate::client::WeeklyBossResinDiscountInfo) -> Self {
        Self {
            discount_num: value.discount_num.into(),
            discount_num_limit: value.discount_num_limit.into(),
            resin_cost: value.resin_cost.into(),
            original_resin_cost: value.original_resin_cost.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SetUpAvatarTeamRsp> for SetUpAvatarTeamRsp {
    fn from(value: crate::client::SetUpAvatarTeamRsp) -> Self {
        Self {
            cur_avatar_guid: value.cur_avatar_guid.into(),
            avatar_team_guid_list: value
                .avatar_team_guid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            team_id: value.team_id.into(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AbilityScalarValueEntry> for AbilityScalarValueEntry {
    fn from(value: crate::client::AbilityScalarValueEntry) -> Self {
        Self {
            key: value.key.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::EntityAuthorityInfo> for EntityAuthorityInfo {
    fn from(value: crate::client::EntityAuthorityInfo) -> Self {
        Self {
            ability_info: value.ability_info.map(|v| v.into()),
            renderer_changed_info: value.renderer_changed_info.map(|v| v.into()),
            ai_info: value.ai_info.map(|v| v.into()),
            born_pos: value.born_pos.map(|v| v.into()),
            pose_para_list: value.pose_para_list.into_iter().map(|v| v.into()).collect(),
            client_extra_info: value.client_extra_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarInfo> for AvatarInfo {
    fn from(value: crate::client::AvatarInfo) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            guid: value.guid.into(),
            prop_map: value
                .prop_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            life_state: value.life_state.into(),
            equip_guid_list: value
                .equip_guid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            talent_id_list: value.talent_id_list.into_iter().map(|v| v.into()).collect(),
            fight_prop_map: value
                .fight_prop_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            trial_avatar_info: value.trial_avatar_info.map(|v| v.into()),
            skill_map: value
                .skill_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            skill_depot_id: value.skill_depot_id.into(),
            fetter_info: value.fetter_info.map(|v| v.into()),
            core_proud_skill_level: value.core_proud_skill_level.into(),
            inherent_proud_skill_list: value
                .inherent_proud_skill_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            skill_level_map: value
                .skill_level_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            expedition_state: value.expedition_state.into(),
            proud_skill_extra_level_map: value
                .proud_skill_extra_level_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            is_focus: value.is_focus.into(),
            avatar_type: value.avatar_type.into(),
            team_resonance_list: value
                .team_resonance_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            wearing_flycloak_id: value.wearing_flycloak_id.into(),
            equip_affix_list: value
                .equip_affix_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            born_time: value.born_time.into(),
            pending_promote_reward_list: value
                .pending_promote_reward_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            costume_id: value.costume_id.into(),
            excel_info: value.excel_info.map(|v| v.into()),
            anim_hash: value.anim_hash.into(),
            mirror_avatar_info: value.mirror_avatar_info.map(|v| v.into()),
            trace_effect_id: value.trace_effect_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::CustomCommonNodeInfo> for CustomCommonNodeInfo {
    fn from(value: crate::client::CustomCommonNodeInfo) -> Self {
        Self {
            parent_index: value.parent_index.into(),
            config_id: value.config_id.into(),
            slot_identifier: value.slot_identifier.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EchoShellInfo> for EchoShellInfo {
    fn from(value: crate::client::EchoShellInfo) -> Self {
        Self {
            shell_id: value.shell_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneEntityInfo> for SceneEntityInfo {
    fn from(value: crate::client::SceneEntityInfo) -> Self {
        Self {
            entity_type: value.entity_type.into(),
            entity_id: value.entity_id.into(),
            name: value.name.into(),
            motion_info: value.motion_info.map(|v| v.into()),
            prop_list: value.prop_list.into_iter().map(|v| v.into()).collect(),
            fight_prop_list: value
                .fight_prop_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            life_state: value.life_state.into(),
            animator_para_list: value
                .animator_para_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            last_move_scene_time_ms: value.last_move_scene_time_ms.into(),
            last_move_reliable_seq: value.last_move_reliable_seq.into(),
            entity_client_data: value.entity_client_data.map(|v| v.into()),
            entity_environment_info_list: value
                .entity_environment_info_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            entity_authority_info: value.entity_authority_info.map(|v| v.into()),
            tag_list: value.tag_list.into_iter().map(|v| v.into()).collect(),
            server_buff_list: value
                .server_buff_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            entity: value.entity.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::scene_entity_info::Entity> for scene_entity_info::Entity {
    fn from(value: crate::client::scene_entity_info::Entity) -> Self {
        match value {
            crate::client::scene_entity_info::Entity::Avatar(v) => Self::Avatar(v.into()),
            crate::client::scene_entity_info::Entity::Monster(v) => {
                Self::Monster(v.into())
            }
            crate::client::scene_entity_info::Entity::Npc(v) => Self::Npc(v.into()),
            crate::client::scene_entity_info::Entity::Gadget(v) => Self::Gadget(v.into()),
            _ => unreachable!(),
        }
    }
}
impl From<crate::client::SceneGadgetInfo> for SceneGadgetInfo {
    fn from(value: crate::client::SceneGadgetInfo) -> Self {
        Self {
            gadget_id: value.gadget_id.into(),
            group_id: value.group_id.into(),
            config_id: value.config_id.into(),
            owner_entity_id: value.owner_entity_id.into(),
            born_type: value.born_type.into(),
            gadget_state: value.gadget_state.into(),
            gadget_type: value.gadget_type.into(),
            is_show_cutscene: value.is_show_cutscene.into(),
            authority_peer_id: value.authority_peer_id.into(),
            is_enable_interact: value.is_enable_interact.into(),
            interact_id: value.interact_id.into(),
            mark_flag: value.mark_flag.into(),
            prop_owner_entity_id: value.prop_owner_entity_id.into(),
            platform: value.platform.map(|v| v.into()),
            interact_uid_list: value
                .interact_uid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            draft_id: value.draft_id.into(),
            gadget_talk_state: value.gadget_talk_state.into(),
            play_info: value.play_info.map(|v| v.into()),
            ugc_tower_level_up_gadget_info: value
                .ugc_tower_level_up_gadget_info
                .map(|v| v.into()),
            journey_gear_operator_info: value
                .journey_gear_operator_info
                .map(|v| v.into()),
            ugc_v2_special_gadget_info: value
                .ugc_v2_special_gadget_info
                .map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::Weapon> for Weapon {
    fn from(value: crate::client::Weapon) -> Self {
        Self {
            level: value.level.into(),
            exp: value.exp.into(),
            promote_level: value.promote_level.into(),
            affix_map: value
                .affix_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AbilityAppliedModifier> for AbilityAppliedModifier {
    fn from(value: crate::client::AbilityAppliedModifier) -> Self {
        Self {
            modifier_local_id: value.modifier_local_id.into(),
            parent_ability_entity_id: value.parent_ability_entity_id.into(),
            parent_ability_name: value.parent_ability_name.map(|v| v.into()),
            parent_ability_override: value.parent_ability_override.map(|v| v.into()),
            instanced_ability_id: value.instanced_ability_id.into(),
            instanced_modifier_id: value.instanced_modifier_id.into(),
            exist_duration: value.exist_duration.into(),
            attached_instanced_modifier: value
                .attached_instanced_modifier
                .map(|v| v.into()),
            apply_entity_id: value.apply_entity_id.into(),
            is_attached_parent_ability: value.is_attached_parent_ability.into(),
            modifier_durability: value.modifier_durability.map(|v| v.into()),
            sbuff_uid: value.sbuff_uid.into(),
            is_serverbuff_modifier: value.is_serverbuff_modifier.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::MpPlayRewardInfo> for MpPlayRewardInfo {
    fn from(value: crate::client::MpPlayRewardInfo) -> Self {
        Self {
            resin: value.resin.into(),
            remain_uid_list: value
                .remain_uid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            qualify_uid_list: value
                .qualify_uid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::WeatherInfo> for WeatherInfo {
    fn from(value: crate::client::WeatherInfo) -> Self {
        Self {
            weather_area_id: value.weather_area_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::GetPlayerTokenRsp> for GetPlayerTokenRsp {
    fn from(value: crate::client::GetPlayerTokenRsp) -> Self {
        Self {
            secret_key_seed: value.secret_key_seed.into(),
            retcode: value.retcode.into(),
            account_uid: value.account_uid.into(),
            uid: value.uid.into(),
            server_rand_key: value.server_rand_key.into(),
            key_id: value.key_id.into(),
            sign: value.sign.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::GadgetCrucibleInfo> for GadgetCrucibleInfo {
    fn from(value: crate::client::GadgetCrucibleInfo) -> Self {
        Self {
            mp_play_id: value.mp_play_id.into(),
            prepare_end_time: value.prepare_end_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::MarkMapReq> for MarkMapReq {
    fn from(value: crate::client::MarkMapReq) -> Self {
        Self {
            old: value.old.map(|v| v.into()),
            mark: value.mark.map(|v| v.into()),
            op: value.op.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::Equip> for Equip {
    fn from(value: crate::client::Equip) -> Self {
        Self {
            is_locked: value.is_locked.into(),
            detail: value.detail.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::equip::Detail> for equip::Detail {
    fn from(value: crate::client::equip::Detail) -> Self {
        match value {
            crate::client::equip::Detail::Reliquary(v) => Self::Reliquary(v.into()),
            crate::client::equip::Detail::Weapon(v) => Self::Weapon(v.into()),
            _ => unreachable!(),
        }
    }
}
impl From<crate::client::MaterialDeleteInfo> for MaterialDeleteInfo {
    fn from(value: crate::client::MaterialDeleteInfo) -> Self {
        Self {
            has_delete_config: value.has_delete_config.into(),
            delete_info: value.delete_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::material_delete_info::DeleteInfo>
for material_delete_info::DeleteInfo {
    fn from(value: crate::client::material_delete_info::DeleteInfo) -> Self {
        match value {
            crate::client::material_delete_info::DeleteInfo::CountDownDelete(v) => {
                Self::CountDownDelete(v.into())
            }
            crate::client::material_delete_info::DeleteInfo::DateDelete(v) => {
                Self::DateDelete(v.into())
            }
            crate::client::material_delete_info::DeleteInfo::DelayWeekCountDownDelete(
                v,
            ) => Self::DelayWeekCountDownDelete(v.into()),
            _ => unreachable!(),
        }
    }
}
impl From<crate::client::CountDownDelete> for CountDownDelete {
    fn from(value: crate::client::CountDownDelete) -> Self {
        Self {
            delete_time_num_map: value
                .delete_time_num_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            config_count_down_time: value.config_count_down_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::DateTimeDelete> for DateTimeDelete {
    fn from(value: crate::client::DateTimeDelete) -> Self {
        Self {
            delete_time: value.delete_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::DelayWeekCountDownDelete> for DelayWeekCountDownDelete {
    fn from(value: crate::client::DelayWeekCountDownDelete) -> Self {
        Self {
            delete_time_num_map: value
                .delete_time_num_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            config_delay_week: value.config_delay_week.into(),
            config_count_down_time: value.config_count_down_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::DeshretObeliskGadgetInfo> for DeshretObeliskGadgetInfo {
    fn from(value: crate::client::DeshretObeliskGadgetInfo) -> Self {
        Self {
            argument_list: value.argument_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::StatueGadgetInfo> for StatueGadgetInfo {
    fn from(value: crate::client::StatueGadgetInfo) -> Self {
        Self {
            opened_statue_uid_list: value
                .opened_statue_uid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::UgcSpecialGadgetInfo> for UgcSpecialGadgetInfo {
    fn from(value: crate::client::UgcSpecialGadgetInfo) -> Self {
        Self {
            group_id: value.group_id.into(),
            guid: value.guid.into(),
            ugc_gadget_config_id: value.ugc_gadget_config_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::CombatInvokeEntry> for CombatInvokeEntry {
    fn from(value: crate::client::CombatInvokeEntry) -> Self {
        Self {
            combat_data: value.combat_data.into_iter().map(|v| v.into()).collect(),
            argument_type: value.argument_type.into(),
            forward_type: value.forward_type.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AbilityGadgetInfo> for AbilityGadgetInfo {
    fn from(value: crate::client::AbilityGadgetInfo) -> Self {
        Self {
            camp_id: value.camp_id.into(),
            camp_target_type: value.camp_target_type.into(),
            target_entity_id: value.target_entity_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarEquipAffixInfo> for AvatarEquipAffixInfo {
    fn from(value: crate::client::AvatarEquipAffixInfo) -> Self {
        Self {
            equip_affix_id: value.equip_affix_id.into(),
            left_cd_time: value.left_cd_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AttackResult> for AttackResult {
    fn from(value: crate::client::AttackResult) -> Self {
        Self {
            damage: value.damage.into(),
            hit_retreat_angle_compat: value.hit_retreat_angle_compat.into(),
            resolved_dir: value.resolved_dir.map(|v| v.into()),
            is_crit: value.is_crit.into(),
            element_type: value.element_type.into(),
            attacker_id: value.attacker_id.into(),
            defense_id: value.defense_id.into(),
            anim_event_id: value.anim_event_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneTeamAvatar> for SceneTeamAvatar {
    fn from(value: crate::client::SceneTeamAvatar) -> Self {
        Self {
            avatar_ability_info: value.avatar_ability_info.map(|v| v.into()),
            scene_entity_info: value.scene_entity_info.map(|v| v.into()),
            scene_avatar_info: value.scene_avatar_info.map(|v| v.into()),
            avatar_guid: value.avatar_guid.into(),
            weapon_entity_id: value.weapon_entity_id.into(),
            is_reconnect: value.is_reconnect.into(),
            server_buff_list: value
                .server_buff_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            is_player_cur_avatar: value.is_player_cur_avatar.into(),
            avatar_info: value.avatar_info.map(|v| v.into()),
            weapon_ability_info: value.weapon_ability_info.map(|v| v.into()),
            ability_control_block: value.ability_control_block.map(|v| v.into()),
            player_uid: value.player_uid.into(),
            scene_id: value.scene_id.into(),
            entity_id: value.entity_id.into(),
            weapon_guid: value.weapon_guid.into(),
            is_on_scene: value.is_on_scene.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::GetSceneAreaReq> for GetSceneAreaReq {
    fn from(value: crate::client::GetSceneAreaReq) -> Self {
        Self {
            scene_id: value.scene_id.into(),
            belong_uid: value.belong_uid.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarFetterInfo> for AvatarFetterInfo {
    fn from(value: crate::client::AvatarFetterInfo) -> Self {
        Self {
            exp_number: value.exp_number.into(),
            exp_level: value.exp_level.into(),
            open_id_list: value.open_id_list.into_iter().map(|v| v.into()).collect(),
            finish_id_list: value.finish_id_list.into_iter().map(|v| v.into()).collect(),
            rewarded_fetter_level_list: value
                .rewarded_fetter_level_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            fetter_list: value.fetter_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::UgcTowerLevelUpGadgetInfo> for UgcTowerLevelUpGadgetInfo {
    fn from(value: crate::client::UgcTowerLevelUpGadgetInfo) -> Self {
        Self {
            tower_level: value.tower_level.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarWearFlycloakRsp> for AvatarWearFlycloakRsp {
    fn from(value: crate::client::AvatarWearFlycloakRsp) -> Self {
        Self {
            flycloak_id: value.flycloak_id.into(),
            retcode: value.retcode.into(),
            avatar_guid: value.avatar_guid.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneFishInfo> for SceneFishInfo {
    fn from(value: crate::client::SceneFishInfo) -> Self {
        Self {
            fish_id: value.fish_id.into(),
            fish_pool_entity_id: value.fish_pool_entity_id.into(),
            fish_pool_pos: value.fish_pool_pos.map(|v| v.into()),
            fish_pool_gadget_id: value.fish_pool_gadget_id.into(),
            last_shock_time: value.last_shock_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AbilityAppliedAbility> for AbilityAppliedAbility {
    fn from(value: crate::client::AbilityAppliedAbility) -> Self {
        Self {
            ability_name: value.ability_name.map(|v| v.into()),
            ability_override: value.ability_override.map(|v| v.into()),
            override_map: value.override_map.into_iter().map(|v| v.into()).collect(),
            instanced_ability_id: value.instanced_ability_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::ScreenInfo> for ScreenInfo {
    fn from(value: crate::client::ScreenInfo) -> Self {
        Self {
            live_id: value.live_id.into(),
            projector_entity_id: value.projector_entity_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::MassivePropParam> for MassivePropParam {
    fn from(value: crate::client::MassivePropParam) -> Self {
        Self {
            r#type: value.r#type.into(),
            reaction_info_list: value
                .reaction_info_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            param_list: value.param_list.into_iter().map(|v| v.into()).collect(),
            sync_flag: value.sync_flag.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneTeamUpdateNotify> for SceneTeamUpdateNotify {
    fn from(value: crate::client::SceneTeamUpdateNotify) -> Self {
        Self {
            is_in_mp: value.is_in_mp.into(),
            scene_team_avatar_list: value
                .scene_team_avatar_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::UnionCmdNotify> for UnionCmdNotify {
    fn from(value: crate::client::UnionCmdNotify) -> Self {
        Self {
            cmd_list: value.cmd_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AbilitySyncStateInfo> for AbilitySyncStateInfo {
    fn from(value: crate::client::AbilitySyncStateInfo) -> Self {
        Self {
            is_inited: value.is_inited.into(),
            dynamic_value_map: value
                .dynamic_value_map
                .into_iter()
                .map(|v| v.into())
                .collect(),
            applied_abilities: value
                .applied_abilities
                .into_iter()
                .map(|v| v.into())
                .collect(),
            applied_modifiers: value
                .applied_modifiers
                .into_iter()
                .map(|v| v.into())
                .collect(),
            mixin_recover_infos: value
                .mixin_recover_infos
                .into_iter()
                .map(|v| v.into())
                .collect(),
            sgv_dynamic_value_map: value
                .sgv_dynamic_value_map
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AbilityEmbryo> for AbilityEmbryo {
    fn from(value: crate::client::AbilityEmbryo) -> Self {
        Self {
            ability_id: value.ability_id.into(),
            ability_name_hash: value.ability_name_hash.into(),
            ability_override_name_hash: value.ability_override_name_hash.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::GetScenePointRsp> for GetScenePointRsp {
    fn from(value: crate::client::GetScenePointRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            scene_id: value.scene_id.into(),
            belong_uid: value.belong_uid.into(),
            is_relogin: value.is_relogin.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::FishPoolInfo> for FishPoolInfo {
    fn from(value: crate::client::FishPoolInfo) -> Self {
        Self {
            pool_id: value.pool_id.into(),
            fish_area_list: value.fish_area_list.into_iter().map(|v| v.into()).collect(),
            today_fish_num: value.today_fish_num.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneInitFinishRsp> for SceneInitFinishRsp {
    fn from(value: crate::client::SceneInitFinishRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            enter_scene_token: value.enter_scene_token.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarEnterSceneInfo> for AvatarEnterSceneInfo {
    fn from(value: crate::client::AvatarEnterSceneInfo) -> Self {
        Self {
            server_buff_list: value
                .server_buff_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            weapon_entity_id: value.weapon_entity_id.into(),
            avatar_guid: value.avatar_guid.into(),
            weapon_guid: value.weapon_guid.into(),
            avatar_ability_info: value.avatar_ability_info.map(|v| v.into()),
            avatar_entity_id: value.avatar_entity_id.into(),
            weapon_ability_info: value.weapon_ability_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::AbilityControlBlock> for AbilityControlBlock {
    fn from(value: crate::client::AbilityControlBlock) -> Self {
        Self {
            ability_embryo_list: value
                .ability_embryo_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarFightPropUpdateNotify> for AvatarFightPropUpdateNotify {
    fn from(value: crate::client::AvatarFightPropUpdateNotify) -> Self {
        Self {
            avatar_guid: value.avatar_guid.into(),
            fight_prop_map: value
                .fight_prop_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::BreakoutVector2> for BreakoutVector2 {
    fn from(value: crate::client::BreakoutVector2) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::JourneyGearGadgetInfo> for JourneyGearGadgetInfo {
    fn from(value: crate::client::JourneyGearGadgetInfo) -> Self {
        Self {
            level_id: value.level_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::ServerBuff> for ServerBuff {
    fn from(value: crate::client::ServerBuff) -> Self {
        Self {
            instanced_modifier_id: value.instanced_modifier_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::BreakoutPhysicalObject> for BreakoutPhysicalObject {
    fn from(value: crate::client::BreakoutPhysicalObject) -> Self {
        Self {
            id: value.id.into(),
            index: value.index.into(),
            is_active: value.is_active.into(),
            pos: value.pos.map(|v| v.into()),
            move_dir: value.move_dir.map(|v| v.into()),
            speed: value.speed.into(),
            init_peer_id: value.init_peer_id.into(),
            state: value.state.into(),
            element_type: value.element_type.into(),
            element_reaction_buff: value.element_reaction_buff.into(),
            modifier_list: value.modifier_list.into_iter().map(|v| v.into()).collect(),
            total_rotation: value.total_rotation.into(),
            info_list: value.info_list.into_iter().map(|v| v.into()).collect(),
            last_hit_peer_id: value.last_hit_peer_id.into(),
            speed_increase_count: value.speed_increase_count.into(),
            offset: value.offset.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::FightPropPair> for FightPropPair {
    fn from(value: crate::client::FightPropPair) -> Self {
        Self {
            prop_type: value.prop_type.into(),
            prop_value: value.prop_value.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::PostEnterSceneReq> for PostEnterSceneReq {
    fn from(value: crate::client::PostEnterSceneReq) -> Self {
        Self {
            enter_scene_token: value.enter_scene_token.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::VehicleMember> for VehicleMember {
    fn from(value: crate::client::VehicleMember) -> Self {
        Self {
            uid: value.uid.into(),
            avatar_guid: value.avatar_guid.into(),
            pos: value.pos.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneInitFinishReq> for SceneInitFinishReq {
    fn from(value: crate::client::SceneInitFinishReq) -> Self {
        Self {
            enter_scene_token: value.enter_scene_token.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::BreakoutSnapShot> for BreakoutSnapShot {
    fn from(value: crate::client::BreakoutSnapShot) -> Self {
        Self {
            client_game_time: value.client_game_time.into(),
            server_game_time: value.server_game_time.into(),
            ball_list: value.ball_list.into_iter().map(|v| v.into()).collect(),
            physical_object_list: value
                .physical_object_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            action_list: value.action_list.into_iter().map(|v| v.into()).collect(),
            wave_index: value.wave_index.into(),
            is_finish: value.is_finish.into(),
            score: value.score.into(),
            combo: value.combo.into(),
            max_combo: value.max_combo.into(),
            life_count: value.life_count.into(),
            wave_suite_index: value.wave_suite_index.into(),
            spawn_point_list: value
                .spawn_point_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            remaining_boss_hp: value.remaining_boss_hp.into(),
            brick_element_reaction_list: value
                .brick_element_reaction_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ball_element_reaction_list: value
                .ball_element_reaction_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            uid_info_list: value.uid_info_list.into_iter().map(|v| v.into()).collect(),
            dynamic_object_list: value
                .dynamic_object_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            id_index_list: value.id_index_list.into_iter().map(|v| v.into()).collect(),
            raw_client_game_time: value.raw_client_game_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::Item> for Item {
    fn from(value: crate::client::Item) -> Self {
        Self {
            item_id: value.item_id.into(),
            guid: value.guid.into(),
            detail: value.detail.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::item::Detail> for item::Detail {
    fn from(value: crate::client::item::Detail) -> Self {
        match value {
            crate::client::item::Detail::Material(v) => Self::Material(v.into()),
            crate::client::item::Detail::Equip(v) => Self::Equip(v.into()),
            crate::client::item::Detail::Furniture(v) => Self::Furniture(v.into()),
            _ => unreachable!(),
        }
    }
}
impl From<crate::client::PlayerLoginRsp> for PlayerLoginRsp {
    fn from(value: crate::client::PlayerLoginRsp) -> Self {
        Self {
            target_uid: value.target_uid.into(),
            login_rand: value.login_rand.into(),
            retcode: value.retcode.into(),
            is_relogin: value.is_relogin.into(),
            total_tick_time: value.total_tick_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::LifeStateChangeNotify> for LifeStateChangeNotify {
    fn from(value: crate::client::LifeStateChangeNotify) -> Self {
        Self {
            attack_tag: value.attack_tag.into(),
            move_reliable_seq: value.move_reliable_seq.into(),
            source_entity_id: value.source_entity_id.into(),
            entity_id: value.entity_id.into(),
            die_type: value.die_type.into(),
            life_state: value.life_state.into(),
            server_buff_list: value
                .server_buff_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::RoutePoint> for RoutePoint {
    fn from(value: crate::client::RoutePoint) -> Self {
        Self {
            position: value.position.map(|v| v.into()),
            arrive_range: value.arrive_range.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::ChangeAvatarRsp> for ChangeAvatarRsp {
    fn from(value: crate::client::ChangeAvatarRsp) -> Self {
        Self {
            skill_id: value.skill_id.into(),
            cur_guid: value.cur_guid.into(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneEntityAppearNotify> for SceneEntityAppearNotify {
    fn from(value: crate::client::SceneEntityAppearNotify) -> Self {
        Self {
            param: value.param.into(),
            entity_list: value.entity_list.into_iter().map(|v| v.into()).collect(),
            appear_type: value.appear_type.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::PostEnterSceneRsp> for PostEnterSceneRsp {
    fn from(value: crate::client::PostEnterSceneRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            enter_scene_token: value.enter_scene_token.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::BreakoutElementReactionCounter>
for BreakoutElementReactionCounter {
    fn from(value: crate::client::BreakoutElementReactionCounter) -> Self {
        Self {
            element_reaction: value.element_reaction.into(),
            count: value.count.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EntityRendererChangedInfo> for EntityRendererChangedInfo {
    fn from(value: crate::client::EntityRendererChangedInfo) -> Self {
        Self {
            changed_renderers: value
                .changed_renderers
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            visibility_count: value.visibility_count.into(),
            is_cached: value.is_cached.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EvtBeingHitInfo> for EvtBeingHitInfo {
    fn from(value: crate::client::EvtBeingHitInfo) -> Self {
        Self {
            peer_id: value.peer_id.into(),
            attack_result: value.attack_result.map(|v| v.into()),
            frame_num: value.frame_num.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarChangeCostumeRsp> for AvatarChangeCostumeRsp {
    fn from(value: crate::client::AvatarChangeCostumeRsp) -> Self {
        Self {
            costume_id: value.costume_id.into(),
            avatar_guid: value.avatar_guid.into(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::WearEquipReq> for WearEquipReq {
    fn from(value: crate::client::WearEquipReq) -> Self {
        Self {
            avatar_guid: value.avatar_guid.into(),
            equip_guid: value.equip_guid.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::ModifierDurability> for ModifierDurability {
    fn from(value: crate::client::ModifierDurability) -> Self {
        Self {
            reduce_ratio: value.reduce_ratio.into(),
            remaining_durability: value.remaining_durability.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::PlatformInfo> for PlatformInfo {
    fn from(value: crate::client::PlatformInfo) -> Self {
        Self {
            route_id: value.route_id.into(),
            start_index: value.start_index.into(),
            start_route_time: value.start_route_time.into(),
            start_scene_time: value.start_scene_time.into(),
            start_pos: value.start_pos.map(|v| v.into()),
            is_started: value.is_started.into(),
            start_rot: value.start_rot.map(|v| v.into()),
            stop_scene_time: value.stop_scene_time.into(),
            pos_offset: value.pos_offset.map(|v| v.into()),
            rot_offset: value.rot_offset.map(|v| v.into()),
            moving_platform_type: value.moving_platform_type.into(),
            is_active: value.is_active.into(),
            route: value.route.map(|v| v.into()),
            point_id: value.point_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AnimatorParameterValueInfoPair>
for AnimatorParameterValueInfoPair {
    fn from(value: crate::client::AnimatorParameterValueInfoPair) -> Self {
        Self {
            name_id: value.name_id.into(),
            animator_para: value.animator_para.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::BreakoutPhysicalObjectModifier>
for BreakoutPhysicalObjectModifier {
    fn from(value: crate::client::BreakoutPhysicalObjectModifier) -> Self {
        Self {
            r#type: value.r#type.into(),
            id: value.id.into(),
            param1: value.param1.into(),
            param2: value.param2.into(),
            param3: value.param3.into(),
            param4: value.param4.into(),
            param5: value.param5.into(),
            param6: value.param6.into(),
            bool1: value.bool1.into(),
            duration: value.duration.into(),
            end_time: value.end_time.into(),
            combo: value.combo.into(),
            peer_id: value.peer_id.into(),
            skill_type: value.skill_type.into(),
            level: value.level.into(),
            choose_player_count: value.choose_player_count.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::BossChestInfo> for BossChestInfo {
    fn from(value: crate::client::BossChestInfo) -> Self {
        Self {
            monster_config_id: value.monster_config_id.into(),
            resin: value.resin.into(),
            remain_uid_list: value
                .remain_uid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            qualify_uid_list: value
                .qualify_uid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            uid_discount_map: value
                .uid_discount_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AbilityString> for AbilityString {
    fn from(value: crate::client::AbilityString) -> Self {
        Self { ..Default::default() }
    }
}
impl From<crate::client::BlossomChestInfo> for BlossomChestInfo {
    fn from(value: crate::client::BlossomChestInfo) -> Self {
        Self {
            resin: value.resin.into(),
            qualify_uid_list: value
                .qualify_uid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            remain_uid_list: value
                .remain_uid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::CoinCollectOperatorInfo> for CoinCollectOperatorInfo {
    fn from(value: crate::client::CoinCollectOperatorInfo) -> Self {
        Self {
            level_id: value.level_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::Vector> for Vector {
    fn from(value: crate::client::Vector) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
            z: value.z.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::PlayerEnterSceneInfoNotify> for PlayerEnterSceneInfoNotify {
    fn from(value: crate::client::PlayerEnterSceneInfoNotify) -> Self {
        Self {
            mp_level_entity_info: value.mp_level_entity_info.map(|v| v.into()),
            enter_scene_token: value.enter_scene_token.into(),
            cur_avatar_entity_id: value.cur_avatar_entity_id.into(),
            team_enter_info: value.team_enter_info.map(|v| v.into()),
            avatar_enter_info: value
                .avatar_enter_info
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EvtAvatarLockChairRsp> for EvtAvatarLockChairRsp {
    fn from(value: crate::client::EvtAvatarLockChairRsp) -> Self {
        Self {
            entity_id: value.entity_id.into(),
            chair_id: value.chair_id.into(),
            position: value.position.map(|v| v.into()),
            retcode: value.retcode.into(),
            direction: value.direction.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::BreakoutAction> for BreakoutAction {
    fn from(value: crate::client::BreakoutAction) -> Self {
        Self {
            action_type: value.action_type.into(),
            client_game_time: value.client_game_time.into(),
            server_game_time: value.server_game_time.into(),
            is_failed: value.is_failed.into(),
            pre_index: value.pre_index.into(),
            new_index: value.new_index.into(),
            pos: value.pos.map(|v| v.into()),
            move_dir: value.move_dir.map(|v| v.into()),
            speed: value.speed.into(),
            peer_id: value.peer_id.into(),
            element_type: value.element_type.into(),
            element_reaction_buff: value.element_reaction_buff.into(),
            speed_increase_count: value.speed_increase_count.into(),
            has_extra_ball: value.has_extra_ball.into(),
            extra_ball_dir: value.extra_ball_dir.map(|v| v.into()),
            extra_ball_index: value.extra_ball_index.into(),
            offset: value.offset.into(),
            execution_game_time: value.execution_game_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::TrialAvatarInfo> for TrialAvatarInfo {
    fn from(value: crate::client::TrialAvatarInfo) -> Self {
        Self {
            trial_avatar_id: value.trial_avatar_id.into(),
            trial_equip_list: value
                .trial_equip_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            grant_record: value.grant_record.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneWeaponInfo> for SceneWeaponInfo {
    fn from(value: crate::client::SceneWeaponInfo) -> Self {
        Self {
            entity_id: value.entity_id.into(),
            gadget_id: value.gadget_id.into(),
            item_id: value.item_id.into(),
            guid: value.guid.into(),
            level: value.level.into(),
            promote_level: value.promote_level.into(),
            ability_info: value.ability_info.map(|v| v.into()),
            affix_map: value
                .affix_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            renderer_changed_info: value.renderer_changed_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::TrialAvatarGrantRecord> for TrialAvatarGrantRecord {
    fn from(value: crate::client::TrialAvatarGrantRecord) -> Self {
        Self {
            grant_reason: value.grant_reason.into(),
            from_parent_quest_id: value.from_parent_quest_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EntityClientData> for EntityClientData {
    fn from(value: crate::client::EntityClientData) -> Self {
        Self {
            wind_change_scene_time: value.wind_change_scene_time.into(),
            windmill_sync_angle: value.windmill_sync_angle.into(),
            wind_change_target_level: value.wind_change_target_level.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AnimatorParameterValueInfo> for AnimatorParameterValueInfo {
    fn from(value: crate::client::AnimatorParameterValueInfo) -> Self {
        Self {
            para_type: value.para_type.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::PlayerEnterSceneNotify> for PlayerEnterSceneNotify {
    fn from(value: crate::client::PlayerEnterSceneNotify) -> Self {
        Self {
            scene_begin_time: value.scene_begin_time.into(),
            r#type: value.r#type.into(),
            pos: value.pos.map(|v| v.into()),
            scene_id: value.scene_id.into(),
            scene_tag_id_list: value
                .scene_tag_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            prev_pos: value.prev_pos.map(|v| v.into()),
            target_uid: value.target_uid.into(),
            enter_scene_token: value.enter_scene_token.into(),
            scene_transaction: value.scene_transaction.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::CombatInvocationsNotify> for CombatInvocationsNotify {
    fn from(value: crate::client::CombatInvocationsNotify) -> Self {
        Self {
            invoke_list: value.invoke_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::MassivePropSyncInfo> for MassivePropSyncInfo {
    fn from(value: crate::client::MassivePropSyncInfo) -> Self {
        Self {
            id: value.id.into(),
            prop_list: value.prop_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::ServerTimeNotify> for ServerTimeNotify {
    fn from(value: crate::client::ServerTimeNotify) -> Self {
        Self {
            server_time: value.server_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::MpLevelEntityInfo> for MpLevelEntityInfo {
    fn from(value: crate::client::MpLevelEntityInfo) -> Self {
        Self {
            authority_peer_id: value.authority_peer_id.into(),
            ability_info: value.ability_info.map(|v| v.into()),
            entity_id: value.entity_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::BreakoutSpawnPoint> for BreakoutSpawnPoint {
    fn from(value: crate::client::BreakoutSpawnPoint) -> Self {
        Self {
            id: value.id.into(),
            brick_suite_id: value.brick_suite_id.into(),
            spawned_brick_list: value
                .spawned_brick_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EvtBeingHitNotify> for EvtBeingHitNotify {
    fn from(value: crate::client::EvtBeingHitNotify) -> Self {
        Self {
            being_hit_info: value.being_hit_info.map(|v| v.into()),
            forward_type: value.forward_type.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::NightCrowGadgetInfo> for NightCrowGadgetInfo {
    fn from(value: crate::client::NightCrowGadgetInfo) -> Self {
        Self {
            argument_list: value.argument_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneTimeNotify> for SceneTimeNotify {
    fn from(value: crate::client::SceneTimeNotify) -> Self {
        Self {
            scene_id: value.scene_id.into(),
            is_paused: value.is_paused.into(),
            scene_time: value.scene_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::Reliquary> for Reliquary {
    fn from(value: crate::client::Reliquary) -> Self {
        Self {
            level: value.level.into(),
            exp: value.exp.into(),
            promote_level: value.promote_level.into(),
            main_prop_id: value.main_prop_id.into(),
            append_prop_id_list: value
                .append_prop_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::FetterData> for FetterData {
    fn from(value: crate::client::FetterData) -> Self {
        Self {
            fetter_id: value.fetter_id.into(),
            fetter_state: value.fetter_state.into(),
            cond_index_list: value
                .cond_index_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::PlayerStoreNotify> for PlayerStoreNotify {
    fn from(value: crate::client::PlayerStoreNotify) -> Self {
        Self {
            item_list: value.item_list.into_iter().map(|v| v.into()).collect(),
            weight_limit: value.weight_limit.into(),
            store_type: value.store_type.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::BreakoutSyncConnectUidInfo> for BreakoutSyncConnectUidInfo {
    fn from(value: crate::client::BreakoutSyncConnectUidInfo) -> Self {
        Self {
            uid: value.uid.into(),
            skill_id_list: value.skill_id_list.into_iter().map(|v| v.into()).collect(),
            skill_level_list: value
                .skill_level_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::PlayerLoginReq> for PlayerLoginReq {
    fn from(value: crate::client::PlayerLoginReq) -> Self {
        Self {
            platform: value.platform.into(),
            account_uid: value.account_uid.into(),
            login_rand: value.login_rand.into(),
            target_uid: value.target_uid.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::ClientSetGameTimeRsp> for ClientSetGameTimeRsp {
    fn from(value: crate::client::ClientSetGameTimeRsp) -> Self {
        Self {
            game_time: value.game_time.into(),
            retcode: value.retcode.into(),
            client_game_time: value.client_game_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EvtAvatarLockChairReq> for EvtAvatarLockChairReq {
    fn from(value: crate::client::EvtAvatarLockChairReq) -> Self {
        Self {
            position: value.position.map(|v| v.into()),
            chair_id: value.chair_id.into(),
            direction: value.direction.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::GadgetPlayInfo> for GadgetPlayInfo {
    fn from(value: crate::client::GadgetPlayInfo) -> Self {
        Self {
            play_type: value.play_type.into(),
            duration: value.duration.into(),
            progress_stage_list: value
                .progress_stage_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            start_cd: value.start_cd.into(),
            start_time: value.start_time.into(),
            progress: value.progress.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EnterSceneReadyRsp> for EnterSceneReadyRsp {
    fn from(value: crate::client::EnterSceneReadyRsp) -> Self {
        Self {
            enter_scene_token: value.enter_scene_token.into(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::GetScenePointReq> for GetScenePointReq {
    fn from(value: crate::client::GetScenePointReq) -> Self {
        Self {
            scene_id: value.scene_id.into(),
            belong_uid: value.belong_uid.into(),
            is_relogin: value.is_relogin.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::FishtankFishInfo> for FishtankFishInfo {
    fn from(value: crate::client::FishtankFishInfo) -> Self {
        Self {
            fish_distance_from_water: value.fish_distance_from_water.into(),
            fish_scale: value.fish_scale.into(),
            initial_rotation_y: value.initial_rotation_y.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::MirrorAvatarInfo> for MirrorAvatarInfo {
    fn from(value: crate::client::MirrorAvatarInfo) -> Self {
        Self {
            copy_from_avatar_type: value.copy_from_avatar_type.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::WorktopInfo> for WorktopInfo {
    fn from(value: crate::client::WorktopInfo) -> Self {
        Self {
            option_list: value.option_list.into_iter().map(|v| v.into()).collect(),
            is_guest_can_operate: value.is_guest_can_operate.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EnterSceneDoneRsp> for EnterSceneDoneRsp {
    fn from(value: crate::client::EnterSceneDoneRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            enter_scene_token: value.enter_scene_token.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::PropPair> for PropPair {
    fn from(value: crate::client::PropPair) -> Self {
        Self {
            r#type: value.r#type.into(),
            prop_value: value.prop_value.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::ServantInfo> for ServantInfo {
    fn from(value: crate::client::ServantInfo) -> Self {
        Self {
            master_entity_id: value.master_entity_id.into(),
            born_slot_index: value.born_slot_index.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarFlycloakChangeNotify> for AvatarFlycloakChangeNotify {
    fn from(value: crate::client::AvatarFlycloakChangeNotify) -> Self {
        Self {
            flycloak_id: value.flycloak_id.into(),
            avatar_guid: value.avatar_guid.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarChangeTraceEffectRsp> for AvatarChangeTraceEffectRsp {
    fn from(value: crate::client::AvatarChangeTraceEffectRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            trace_effect_id: value.trace_effect_id.into(),
            avatar_guid: value.avatar_guid.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::MathQuaternion> for MathQuaternion {
    fn from(value: crate::client::MathQuaternion) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
            z: value.z.into(),
            w: value.w.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EnterScenePeerNotify> for EnterScenePeerNotify {
    fn from(value: crate::client::EnterScenePeerNotify) -> Self {
        Self {
            enter_scene_token: value.enter_scene_token.into(),
            peer_id: value.peer_id.into(),
            dest_scene_id: value.dest_scene_id.into(),
            host_peer_id: value.host_peer_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarExcelInfo> for AvatarExcelInfo {
    fn from(value: crate::client::AvatarExcelInfo) -> Self {
        Self {
            prefab_path_hash: value.prefab_path_hash.into(),
            prefab_path_remote_hash: value.prefab_path_remote_hash.into(),
            controller_path_hash: value.controller_path_hash.into(),
            controller_path_remote_hash: value.controller_path_remote_hash.into(),
            combat_config_hash: value.combat_config_hash.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarSkillInfo> for AvatarSkillInfo {
    fn from(value: crate::client::AvatarSkillInfo) -> Self {
        Self { ..Default::default() }
    }
}
impl From<crate::client::AvatarTeam> for AvatarTeam {
    fn from(value: crate::client::AvatarTeam) -> Self {
        Self {
            team_name: value.team_name.into(),
            avatar_guid_list: value
                .avatar_guid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EntityFightPropUpdateNotify> for EntityFightPropUpdateNotify {
    fn from(value: crate::client::EntityFightPropUpdateNotify) -> Self {
        Self {
            fight_prop_map: value
                .fight_prop_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            entity_id: value.entity_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarDataNotify> for AvatarDataNotify {
    fn from(value: crate::client::AvatarDataNotify) -> Self {
        Self {
            avatar_rename_list: value
                .avatar_rename_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            owned_trace_effect_list: value
                .owned_trace_effect_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            avatar_team_map: value
                .avatar_team_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            backup_avatar_team_order_list: value
                .backup_avatar_team_order_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            choose_avatar_guid: value.choose_avatar_guid.into(),
            owned_costume_list: value
                .owned_costume_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            temp_avatar_guid_list: value
                .temp_avatar_guid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            cur_avatar_team_id: value.cur_avatar_team_id.into(),
            owned_flycloak_list: value
                .owned_flycloak_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::PlayerDataNotify> for PlayerDataNotify {
    fn from(value: crate::client::PlayerDataNotify) -> Self {
        Self {
            is_first_login_today: value.is_first_login_today.into(),
            prop_map: value
                .prop_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            server_time: value.server_time.into(),
            nick_name: value.nick_name.into(),
            region_id: value.region_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarEquipChangeNotify> for AvatarEquipChangeNotify {
    fn from(value: crate::client::AvatarEquipChangeNotify) -> Self {
        Self {
            item_id: value.item_id.into(),
            equip_type: value.equip_type.into(),
            reliquary: value.reliquary.map(|v| v.into()),
            weapon: value.weapon.map(|v| v.into()),
            avatar_guid: value.avatar_guid.into(),
            equip_guid: value.equip_guid.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::VehicleInfo> for VehicleInfo {
    fn from(value: crate::client::VehicleInfo) -> Self {
        Self {
            member_list: value.member_list.into_iter().map(|v| v.into()).collect(),
            owner_uid: value.owner_uid.into(),
            cur_stamina: value.cur_stamina.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::PingReq> for PingReq {
    fn from(value: crate::client::PingReq) -> Self {
        Self {
            total_tick_time: value.total_tick_time.into(),
            client_time: value.client_time.into(),
            seq: value.seq.into(),
            ue_time: value.ue_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::OpenStateUpdateNotify> for OpenStateUpdateNotify {
    fn from(value: crate::client::OpenStateUpdateNotify) -> Self {
        Self {
            open_state_map: value
                .open_state_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::GetPlayerTokenReq> for GetPlayerTokenReq {
    fn from(value: crate::client::GetPlayerTokenReq) -> Self {
        Self {
            client_rand_key: value.client_rand_key.into(),
            account_uid: value.account_uid.into(),
            key_id: value.key_id.into(),
            uid: value.uid.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarWearFlycloakReq> for AvatarWearFlycloakReq {
    fn from(value: crate::client::AvatarWearFlycloakReq) -> Self {
        Self {
            avatar_guid: value.avatar_guid.into(),
            flycloak_id: value.flycloak_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::GadgetGeneralRewardInfo> for GadgetGeneralRewardInfo {
    fn from(value: crate::client::GadgetGeneralRewardInfo) -> Self {
        Self {
            resin: value.resin.into(),
            remain_uid_list: value
                .remain_uid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            qualify_uid_list: value
                .qualify_uid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneMonsterInfo> for SceneMonsterInfo {
    fn from(value: crate::client::SceneMonsterInfo) -> Self {
        Self {
            monster_id: value.monster_id.into(),
            group_id: value.group_id.into(),
            config_id: value.config_id.into(),
            weapon_list: value.weapon_list.into_iter().map(|v| v.into()).collect(),
            authority_peer_id: value.authority_peer_id.into(),
            affix_list: value.affix_list.into_iter().map(|v| v.into()).collect(),
            is_elite: value.is_elite.into(),
            owner_entity_id: value.owner_entity_id.into(),
            summoned_tag: value.summoned_tag.into(),
            summon_tag_map: value
                .summon_tag_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            pose_id: value.pose_id.into(),
            born_type: value.born_type.into(),
            block_id: value.block_id.into(),
            mark_flag: value.mark_flag.into(),
            title_id: value.title_id.into(),
            special_name_id: value.special_name_id.into(),
            attack_target_id: value.attack_target_id.into(),
            monster_route: value.monster_route.map(|v| v.into()),
            ai_config_id: value.ai_config_id.into(),
            level_route_id: value.level_route_id.into(),
            init_pose_id: value.init_pose_id.into(),
            is_light: value.is_light.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::PathfindingEnterSceneRsp> for PathfindingEnterSceneRsp {
    fn from(value: crate::client::PathfindingEnterSceneRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarRenameInfo> for AvatarRenameInfo {
    fn from(value: crate::client::AvatarRenameInfo) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneAvatarInfo> for SceneAvatarInfo {
    fn from(value: crate::client::SceneAvatarInfo) -> Self {
        Self {
            uid: value.uid.into(),
            avatar_id: value.avatar_id.into(),
            guid: value.guid.into(),
            peer_id: value.peer_id.into(),
            equip_id_list: value.equip_id_list.into_iter().map(|v| v.into()).collect(),
            skill_depot_id: value.skill_depot_id.into(),
            talent_id_list: value.talent_id_list.into_iter().map(|v| v.into()).collect(),
            weapon: value.weapon.map(|v| v.into()),
            reliquary_list: value.reliquary_list.into_iter().map(|v| v.into()).collect(),
            core_proud_skill_level: value.core_proud_skill_level.into(),
            inherent_proud_skill_list: value
                .inherent_proud_skill_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            skill_level_map: value
                .skill_level_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            proud_skill_extra_level_map: value
                .proud_skill_extra_level_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            server_buff_list: value
                .server_buff_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            team_resonance_list: value
                .team_resonance_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            wearing_flycloak_id: value.wearing_flycloak_id.into(),
            born_time: value.born_time.into(),
            costume_id: value.costume_id.into(),
            cur_vehicle_info: value.cur_vehicle_info.map(|v| v.into()),
            excel_info: value.excel_info.map(|v| v.into()),
            anim_hash: value.anim_hash.into(),
            trace_effect_id: value.trace_effect_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::MapMarkPoint> for MapMarkPoint {
    fn from(value: crate::client::MapMarkPoint) -> Self {
        Self {
            scene_id: value.scene_id.into(),
            name: value.name.into(),
            pos: value.pos.map(|v| v.into()),
            point_type: value.point_type.into(),
            monster_id: value.monster_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarChangeTraceEffectNotify>
for AvatarChangeTraceEffectNotify {
    fn from(value: crate::client::AvatarChangeTraceEffectNotify) -> Self {
        Self {
            entity_info: value.entity_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::PlayerGameTimeNotify> for PlayerGameTimeNotify {
    fn from(value: crate::client::PlayerGameTimeNotify) -> Self {
        Self {
            game_time: value.game_time.into(),
            is_home: value.is_home.into(),
            uid: value.uid.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::ClientSetGameTimeReq> for ClientSetGameTimeReq {
    fn from(value: crate::client::ClientSetGameTimeReq) -> Self {
        Self {
            game_time: value.game_time.into(),
            is_force_set: value.is_force_set.into(),
            client_game_time: value.client_game_time.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::ClientGadgetInfo> for ClientGadgetInfo {
    fn from(value: crate::client::ClientGadgetInfo) -> Self {
        Self {
            camp_id: value.camp_id.into(),
            camp_type: value.camp_type.into(),
            guid: value.guid.into(),
            owner_entity_id: value.owner_entity_id.into(),
            target_entity_id: value.target_entity_id.into(),
            async_load: value.async_load.into(),
            is_peer_id_from_player: value.is_peer_id_from_player.into(),
            target_entity_id_list: value
                .target_entity_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            target_lock_point_index_list: value
                .target_lock_point_index_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::PathfindingEnterSceneReq> for PathfindingEnterSceneReq {
    fn from(value: crate::client::PathfindingEnterSceneReq) -> Self {
        Self {
            scene_id: value.scene_id.into(),
            version: value.version.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::OfferingInfo> for OfferingInfo {
    fn from(value: crate::client::OfferingInfo) -> Self {
        Self {
            offering_id: value.offering_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::CityInfo> for CityInfo {
    fn from(value: crate::client::CityInfo) -> Self {
        Self {
            crystal_num: value.crystal_num.into(),
            level: value.level.into(),
            city_id: value.city_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::CurVehicleInfo> for CurVehicleInfo {
    fn from(value: crate::client::CurVehicleInfo) -> Self {
        Self {
            entity_id: value.entity_id.into(),
            pos: value.pos.into(),
            gadget_id: value.gadget_id.into(),
            enter_pos: value.enter_pos.map(|v| v.into()),
            vehicle_type: value.vehicle_type.into(),
            enter_rot: value.enter_rot.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::PlayerNormalLuaShellNotify> for PlayerNormalLuaShellNotify {
    fn from(value: crate::client::PlayerNormalLuaShellNotify) -> Self {
        Self {
            config_id: value.config_id.into(),
            luashell: value.luashell.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::BreakoutBrickInfo> for BreakoutBrickInfo {
    fn from(value: crate::client::BreakoutBrickInfo) -> Self {
        Self {
            hp: value.hp.into(),
            element_type: value.element_type.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarChangeCostumeNotify> for AvatarChangeCostumeNotify {
    fn from(value: crate::client::AvatarChangeCostumeNotify) -> Self {
        Self {
            entity_info: value.entity_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::EvtAvatarStandUpNotify> for EvtAvatarStandUpNotify {
    fn from(value: crate::client::EvtAvatarStandUpNotify) -> Self {
        Self {
            direction: value.direction.into(),
            entity_id: value.entity_id.into(),
            perform_id: value.perform_id.into(),
            chair_id: value.chair_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EnterSceneReadyReq> for EnterSceneReadyReq {
    fn from(value: crate::client::EnterSceneReadyReq) -> Self {
        Self {
            enter_scene_token: value.enter_scene_token.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::ChangeAvatarReq> for ChangeAvatarReq {
    fn from(value: crate::client::ChangeAvatarReq) -> Self {
        Self {
            move_pos: value.move_pos.map(|v| v.into()),
            guid: value.guid.into(),
            is_move: value.is_move.into(),
            skill_id: value.skill_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::UnionCmd> for UnionCmd {
    fn from(value: crate::client::UnionCmd) -> Self {
        Self {
            body: value.body.into_iter().map(|v| v.into()).collect(),
            message_id: value.message_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::Furniture> for Furniture {
    fn from(value: crate::client::Furniture) -> Self {
        Self {
            count: value.count.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::QueryPathReq> for QueryPathReq {
    fn from(value: crate::client::QueryPathReq) -> Self {
        Self {
            destination_pos: value
                .destination_pos
                .into_iter()
                .map(|v| v.into())
                .collect(),
            source_pos: value.source_pos.map(|v| v.into()),
            query_id: value.query_id.into(),
            scene_id: value.scene_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::RoguelikeGadgetInfo> for RoguelikeGadgetInfo {
    fn from(value: crate::client::RoguelikeGadgetInfo) -> Self {
        Self {
            cell_config_id: value.cell_config_id.into(),
            cell_type: value.cell_type.into(),
            cell_state: value.cell_state.into(),
            cell_id: value.cell_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneReliquaryInfo> for SceneReliquaryInfo {
    fn from(value: crate::client::SceneReliquaryInfo) -> Self {
        Self {
            item_id: value.item_id.into(),
            guid: value.guid.into(),
            level: value.level.into(),
            promote_level: value.promote_level.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::CustomGadgetTreeInfo> for CustomGadgetTreeInfo {
    fn from(value: crate::client::CustomGadgetTreeInfo) -> Self {
        Self {
            node_list: value.node_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EntityClientExtraInfo> for EntityClientExtraInfo {
    fn from(value: crate::client::EntityClientExtraInfo) -> Self {
        Self {
            skill_anchor_position: value.skill_anchor_position.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::GetSceneAreaRsp> for GetSceneAreaRsp {
    fn from(value: crate::client::GetSceneAreaRsp) -> Self {
        Self {
            scene_id: value.scene_id.into(),
            city_info_list: value.city_info_list.into_iter().map(|v| v.into()).collect(),
            retcode: value.retcode.into(),
            area_id_list: value.area_id_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarChangeCostumeReq> for AvatarChangeCostumeReq {
    fn from(value: crate::client::AvatarChangeCostumeReq) -> Self {
        Self {
            avatar_guid: value.avatar_guid.into(),
            costume_id: value.costume_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarChangeTraceEffectReq> for AvatarChangeTraceEffectReq {
    fn from(value: crate::client::AvatarChangeTraceEffectReq) -> Self {
        Self {
            trace_effect_id: value.trace_effect_id.into(),
            avatar_guid: value.avatar_guid.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::MotionInfo> for MotionInfo {
    fn from(value: crate::client::MotionInfo) -> Self {
        Self {
            pos: value.pos.map(|v| v.into()),
            rot: value.rot.map(|v| v.into()),
            speed: value.speed.map(|v| v.into()),
            state: value.state.into(),
            params: value.params.into_iter().map(|v| v.into()).collect(),
            ref_pos: value.ref_pos.map(|v| v.into()),
            ref_id: value.ref_id.into(),
            scene_time: value.scene_time.into(),
            interval_velocity: value.interval_velocity.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneNpcInfo> for SceneNpcInfo {
    fn from(value: crate::client::SceneNpcInfo) -> Self {
        Self {
            npc_id: value.npc_id.into(),
            room_id: value.room_id.into(),
            parent_quest_id: value.parent_quest_id.into(),
            block_id: value.block_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EntityEnvironmentInfo> for EntityEnvironmentInfo {
    fn from(value: crate::client::EntityEnvironmentInfo) -> Self {
        Self {
            json_climate_type: value.json_climate_type.into(),
            climate_area_id: value.climate_area_id.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::Route> for Route {
    fn from(value: crate::client::Route) -> Self {
        Self {
            route_points: value.route_points.into_iter().map(|v| v.into()).collect(),
            route_type: value.route_type.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SceneEntityAiInfo> for SceneEntityAiInfo {
    fn from(value: crate::client::SceneEntityAiInfo) -> Self {
        Self {
            skill_cd_map: value
                .skill_cd_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            servant_info: value.servant_info.map(|v| v.into()),
            ai_threat_map: value
                .ai_threat_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            skill_group_cd_map: value
                .skill_group_cd_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            cur_tactic: value.cur_tactic.into(),
            is_entered_combat: value.is_entered_combat.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EntityMoveInfo> for EntityMoveInfo {
    fn from(value: crate::client::EntityMoveInfo) -> Self {
        Self {
            entity_id: value.entity_id.into(),
            motion_info: value.motion_info.map(|v| v.into()),
            scene_time: value.scene_time.into(),
            reliable_seq: value.reliable_seq.into(),
            is_reliable: value.is_reliable.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::MonsterRoute> for MonsterRoute {
    fn from(value: crate::client::MonsterRoute) -> Self {
        Self {
            route_points: value.route_points.into_iter().map(|v| v.into()).collect(),
            speed_level: value.speed_level.into(),
            route_type: value.route_type.into(),
            arrive_range: value.arrive_range.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AbilityMixinRecoverInfo> for AbilityMixinRecoverInfo {
    fn from(value: crate::client::AbilityMixinRecoverInfo) -> Self {
        Self {
            local_id: value.local_id.into(),
            data_list: value.data_list.into_iter().map(|v| v.into()).collect(),
            is_serverbuff_modifier: value.is_serverbuff_modifier.into(),
            massive_prop_list: value
                .massive_prop_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            breakout_snap_shot: value.breakout_snap_shot.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::GatherGadgetInfo> for GatherGadgetInfo {
    fn from(value: crate::client::GatherGadgetInfo) -> Self {
        Self {
            item_id: value.item_id.into(),
            is_forbid_guest: value.is_forbid_guest.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::AbilityAttachedModifier> for AbilityAttachedModifier {
    fn from(value: crate::client::AbilityAttachedModifier) -> Self {
        Self {
            is_invalid: value.is_invalid.into(),
            owner_entity_id: value.owner_entity_id.into(),
            instanced_modifier_id: value.instanced_modifier_id.into(),
            is_serverbuff_modifier: value.is_serverbuff_modifier.into(),
            attach_name_hash: value.attach_name_hash.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::SetUpAvatarTeamReq> for SetUpAvatarTeamReq {
    fn from(value: crate::client::SetUpAvatarTeamReq) -> Self {
        Self {
            avatar_team_guid_list: value
                .avatar_team_guid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            team_id: value.team_id.into(),
            cur_avatar_guid: value.cur_avatar_guid.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::PropValue> for PropValue {
    fn from(value: crate::client::PropValue) -> Self {
        Self {
            r#type: value.r#type.into(),
            val: value.val.into(),
            value: value.value.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::prop_value::Value> for prop_value::Value {
    fn from(value: crate::client::prop_value::Value) -> Self {
        match value {
            crate::client::prop_value::Value::Ival(v) => Self::Ival(v.into()),
            crate::client::prop_value::Value::Fval(v) => Self::Fval(v.into()),
            _ => unreachable!(),
        }
    }
}
impl From<crate::client::QueryPathRsp> for QueryPathRsp {
    fn from(value: crate::client::QueryPathRsp) -> Self {
        Self {
            corners: value.corners.into_iter().map(|v| v.into()).collect(),
            query_id: value.query_id.into(),
            query_status: value.query_status.into(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::Material> for Material {
    fn from(value: crate::client::Material) -> Self {
        Self {
            count: value.count.into(),
            delete_info: value.delete_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::AvatarTeamUpdateNotify> for AvatarTeamUpdateNotify {
    fn from(value: crate::client::AvatarTeamUpdateNotify) -> Self {
        Self {
            avatar_team_map: value
                .avatar_team_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            temp_avatar_guid_list: value
                .temp_avatar_guid_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
impl From<crate::client::TeamEnterSceneInfo> for TeamEnterSceneInfo {
    fn from(value: crate::client::TeamEnterSceneInfo) -> Self {
        Self {
            team_entity_id: value.team_entity_id.into(),
            ability_control_block: value.ability_control_block.map(|v| v.into()),
            team_ability_info: value.team_ability_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
impl From<crate::client::PingRsp> for PingRsp {
    fn from(value: crate::client::PingRsp) -> Self {
        Self {
            client_time: value.client_time.into(),
            seq: value.seq.into(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::EnterSceneDoneReq> for EnterSceneDoneReq {
    fn from(value: crate::client::EnterSceneDoneReq) -> Self {
        Self {
            enter_scene_token: value.enter_scene_token.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::FoundationInfo> for FoundationInfo {
    fn from(value: crate::client::FoundationInfo) -> Self {
        Self {
            status: value.status.into(),
            uid_list: value.uid_list.into_iter().map(|v| v.into()).collect(),
            current_building_id: value.current_building_id.into(),
            locked_by_uid: value.locked_by_uid.into(),
            ..Default::default()
        }
    }
}
impl From<crate::client::WearEquipRsp> for WearEquipRsp {
    fn from(value: crate::client::WearEquipRsp) -> Self {
        Self {
            avatar_guid: value.avatar_guid.into(),
            retcode: value.retcode.into(),
            equip_guid: value.equip_guid.into(),
            ..Default::default()
        }
    }
}
