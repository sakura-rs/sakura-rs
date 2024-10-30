use avatar::{
    change_avatar, notify_avatar_team_update, replace_avatar_team, set_up_avatar_team,
    PlayerAvatarTeamChanged,
};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::{
    CurrentSceneID, PlayerSceneState, PlayerSceneStates, ScenePeerManager, WorldOwnerUID,
};
use enter::EnterSceneStateSystems;
use sakura_entity::{
    ability::Ability,
    avatar::AvatarQueryReadOnly,
    common::{EntityCounter, OwnerPlayerUID, ToBeRemovedMarker},
    mp_level::{AuthorityPeerId, MpLevelBundle, MpLevelEntityMarker},
    play_team::{PlayTeamEntityBundle, PlayTeamEntityMarker},
    team::{TeamEntityBundle, TeamEntityMarker},
    transform::Vector3,
    util::to_protocol_entity_id,
};
use sakura_message::output::MessageOutput;
use sakura_persistence::Players;
use sakura_proto::{EnterType, ProtEntityType};
use player_join_team::PlayerJoinTeamEvent;
use scene_team_update::SceneTeamUpdateEvent;

pub use enter::{
    EnterSceneDoneEvent, EnterSceneReadyEvent, PostEnterSceneEvent, SceneInitFinishEvent,
};

pub use player_jump::ScenePlayerJumpEvent;

mod avatar;
mod enter;
mod player_join_team;
mod player_jump;
mod scene_team_update;
mod sync_enter_info;

pub mod common;

pub struct ScenePlugin;

#[derive(Event)]
pub struct BeginEnterSceneEvent {
    pub uid: u32,
    pub scene_id: u32,
    pub enter_type: EnterType,
    pub position: Vector3,
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BeginEnterSceneEvent>()
            .add_event::<EnterSceneReadyEvent>()
            .add_event::<SceneInitFinishEvent>()
            .add_event::<EnterSceneDoneEvent>()
            .add_event::<PostEnterSceneEvent>()
            .add_event::<PlayerJoinTeamEvent>()
            .add_event::<SceneTeamUpdateEvent>()
            .add_event::<PlayerAvatarTeamChanged>()
            .add_event::<ScenePlayerJumpEvent>()
            .init_resource::<EnterSceneStateSystems>()
            .insert_resource(WorldOwnerUID(0))
            .insert_resource(PlayerSceneStates::default())
            .insert_resource(ScenePeerManager::default())
            .insert_resource(CurrentSceneID::default())
            .add_systems(PostStartup, init_scene)
            .add_systems(PreUpdate, enter::handle_enter_scene_state_change)
            .add_systems(PreUpdate, (set_up_avatar_team, replace_avatar_team).chain())
            .add_systems(PreUpdate, change_avatar)
            .add_systems(Update, player_join_team::player_join_team)
            .add_systems(Update, player_jump::player_jump)
            .add_systems(
                PostUpdate,
                (
                    begin_enter_scene,
                    create_play_team_entity,
                    notify_player_enter_scene,
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                (
                    sync_enter_info::sync_enter_info,
                    sync_enter_info::sync_play_team_entity,
                    scene_team_update::notify_scene_team_update,
                    enter::scene_init_finish_send_rsp,
                    enter::enter_scene_done_send_rsp,
                    notify_avatar_team_update,
                )
                    .chain(),
            );
    }
}

fn init_scene(
    mut commands: Commands,
    players: Res<Players>,
    mut entity_counter: ResMut<EntityCounter>,
    mut current_scene_id: ResMut<CurrentSceneID>,
    mut enter_events: EventWriter<BeginEnterSceneEvent>,
) {
    commands.spawn(TeamEntityBundle {
        marker: TeamEntityMarker,
        entity_id: to_protocol_entity_id(ProtEntityType::Team, entity_counter.next()),
        ability: Ability::new_for_team(),
    });

    commands.spawn(MpLevelBundle {
        authority_peer_id: AuthorityPeerId(1),
        entity_id: to_protocol_entity_id(ProtEntityType::MpLevel, entity_counter.next()),
        marker: MpLevelEntityMarker,
    });

    **current_scene_id = 3;

    for uid in players.keys() {
        let uid = *uid;
        let player_info = players.get(uid);

        enter_events.send(BeginEnterSceneEvent {
            uid,
            scene_id: **current_scene_id,
            enter_type: sakura_proto::EnterType::EnterSelf,
            position: player_info.world_position.position.into(),
        });
    }
}

fn begin_enter_scene(
    mut events: EventReader<BeginEnterSceneEvent>,
    mut commands: Commands,
    mut player_scene_states: ResMut<PlayerSceneStates>,
    player_avatar_entities: Query<(Entity, AvatarQueryReadOnly)>,
) {
    for event in events.read() {
        for (entity, avatar_data) in player_avatar_entities
            .iter()
            .filter(|(_, data)| data.owner_player_uid.0 == event.uid)
        {
            commands.entity(entity).insert(ToBeRemovedMarker);
            commands
                .entity(avatar_data.equipment.weapon)
                .insert(ToBeRemovedMarker);
        }

        player_scene_states.insert(event.uid, PlayerSceneState::new());
    }
}

fn create_play_team_entity(
    mut events: EventReader<BeginEnterSceneEvent>,
    mut commands: Commands,
    mut entity_counter: ResMut<EntityCounter>,
    play_team_entities: Query<(Entity, &OwnerPlayerUID), With<PlayTeamEntityMarker>>,
) {
    for event in events.read() {
        if !play_team_entities
            .iter()
            .any(|(_, owner_uid)| owner_uid.0 == event.uid)
        {
            commands.spawn(PlayTeamEntityBundle {
                player_uid: OwnerPlayerUID(event.uid),
                entity_id: to_protocol_entity_id(
                    ProtEntityType::PlayTeamEntity,
                    entity_counter.next(),
                ),
                ability: Ability::default(),
                marker: PlayTeamEntityMarker,
            });
        }
    }
}

fn notify_player_enter_scene(
    mut events: EventReader<BeginEnterSceneEvent>,
    message_output: Res<MessageOutput>,
    current_scene_id: Res<CurrentSceneID>,
    player_scene_states: Res<PlayerSceneStates>,
) {
    for event in events.read() {
        message_output.send(
            event.uid,
            sakura_proto::PlayerEnterSceneNotify {
                scene_id: **current_scene_id,
                enter_scene_token: player_scene_states
                    .get(&event.uid)
                    .unwrap()
                    .enter_scene_token(),
                target_uid: event.uid,
                pos: Some(event.position.clone().into()),
                prev_pos: Some(Default::default()),
                scene_transaction: format!(
                    "{}-{}-{}-{}",
                    **current_scene_id,
                    event.uid,
                    ::common::time_util::unix_timestamp(),
                    179398
                ),
                r#type: event.enter_type.into(),
                ..Default::default()
            },
        );
    }
}
