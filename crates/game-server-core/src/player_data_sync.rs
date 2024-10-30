use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::time_util;
use sakura_data::excel;
use sakura_entity::{common::create_fight_props, int_prop_map};
use sakura_message::output::MessageOutput;
use sakura_persistence::{player_information::ItemInformation, Players};
use sakura_proto::*;

pub struct PlayerDataSyncPlugin;

impl Plugin for PlayerDataSyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                sync_player_data,
                sync_player_store,
                sync_open_state_map,
                sync_avatar_data,
            )
                .chain(),
        );
    }
}

pub fn sync_player_data(players: Res<Players>, out: Res<MessageOutput>) {
    for uid in players.keys() {
        let player_info = players.get(*uid);
        out.send(
            *uid,
            PlayerDataNotify {
                nick_name: player_info.nick_name.clone(),
                prop_map: int_prop_map! {
                    PROP_PLAYER_LEVEL: player_info.basic_module.level;
                    PROP_PLAYER_EXP: player_info.basic_module.exp;
                    PROP_IS_GAME_TIME_LOCKED: player_info.basic_module.is_game_time_locked as i64;
                    PROP_IS_FLYABLE: 1;
                    PROP_IS_TRANSFERABLE: 1;
                    PROP_IS_SPRING_AUTO_USE: 1;
                    PROP_SPRING_AUTO_USE_PERCENT: 50;
                    PROP_MAX_STAMINA: 10000;
                    PROP_CUR_PERSIST_STAMINA: 10000;
                },
                server_time: time_util::unix_timestamp(),
                is_first_login_today: false,
                region_id: 0,
            },
        )
    }
}

pub fn sync_player_store(players: Res<Players>, out: Res<MessageOutput>) {
    for uid in players.keys() {
        let player_info = players.get(*uid);

        out.send(
            *uid,
            PlayerStoreNotify {
                store_type: StoreType::Pack.into(),
                weight_limit: 30_000,
                item_list: player_info
                    .item_map
                    .iter()
                    .map(|(guid, item)| match item {
                        ItemInformation::Weapon {
                            weapon_id,
                            level,
                            exp,
                            promote_level,
                            affix_map,
                            is_locked,
                        } => Item {
                            item_id: *weapon_id,
                            guid: *guid,
                            detail: Some(item::Detail::Equip(Equip {
                                is_locked: *is_locked,
                                detail: Some(equip::Detail::Weapon(Weapon {
                                    level: *level,
                                    exp: *exp,
                                    promote_level: *promote_level,
                                    affix_map: affix_map.clone(),
                                })),
                            })),
                        },
                    })
                    .collect(),
            },
        );
    }
}

pub fn sync_avatar_data(players: Res<Players>, out: Res<MessageOutput>) {
    for uid in players.keys() {
        let player_info = players.get(*uid);

        out.send(
            *uid,
            AvatarDataNotify {
                avatar_list: player_info
                    .avatar_module
                    .avatar_map
                    .values()
                    .map(|a| {
                        AvatarInfo {
                            avatar_id: a.avatar_id,
                            guid: a.guid,
                            equip_guid_list: vec![a.weapon_guid],
                            skill_depot_id: a.skill_depot_id,
                            born_time: a.born_time,
                            life_state: 1,               // TODO!
                            avatar_type: 1,              // TODO!
                            wearing_flycloak_id: 140001, // TODO!
                            fetter_info: Some(AvatarFetterInfo::default()),
                            skill_level_map: a.skill_level_map.clone(),
                            inherent_proud_skill_list: a.inherent_proud_skill_list.clone(),
                            prop_map: int_prop_map! {
                                PROP_LEVEL: a.level;
                                PROP_BREAK_LEVEL: a.break_level;
                            },
                            fight_prop_map: create_fight_props(
                                excel::avatar_excel_config_collection::iter()
                                    .find(|conf| conf.id == a.avatar_id)
                                    .unwrap(),
                                a.cur_hp,
                                a.level,
                                a.break_level,
                            )
                            .0
                            .iter()
                            .map(|(ty, val)| (*ty as u32, *val))
                            .collect(),
                            ..Default::default()
                        }
                    })
                    .collect(),
                avatar_team_map: player_info
                    .avatar_module
                    .team_map
                    .iter()
                    .map(|(idx, team)| {
                        (
                            *idx,
                            AvatarTeam {
                                team_name: team.name.clone(),
                                avatar_guid_list: team.avatar_guid_list.clone(),
                            },
                        )
                    })
                    .collect(),
                cur_avatar_team_id: 1,
                ..Default::default()
            },
        );
    }
}

pub fn sync_open_state_map(players: Res<Players>, out: Res<MessageOutput>) {
    for uid in players.keys() {
        out.send(
            *uid,
            OpenStateUpdateNotify {
                open_state_map: excel::open_state_config_collection::iter()
                    .map(|c| (c.id, 1))
                    .collect(),
            },
        );
    }
}
