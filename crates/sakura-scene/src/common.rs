use std::collections::HashMap;

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;

use crate::enter::EnterSceneState;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CurrentSceneID(u32);

#[derive(Resource)]
pub struct WorldOwnerUID(pub u32);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct PlayerSceneStates(HashMap<u32, PlayerSceneState>);

pub struct PlayerSceneState {
    enter_scene_token: u32,
    cur_state: Option<EnterSceneState>,
}

impl PlayerSceneState {
    pub fn new() -> Self {
        use rand::RngCore;

        Self {
            enter_scene_token: rand::thread_rng().next_u32() % 100_000,
            cur_state: None,
        }
    }

    pub fn enter_scene_token(&self) -> u32 {
        self.enter_scene_token
    }

    pub fn enter_state(&self) -> Option<EnterSceneState> {
        self.cur_state
    }

    pub fn change_enter_state(&mut self, state: EnterSceneState) -> bool {
        if match self.cur_state {
            None => state == EnterSceneState::Ready,
            Some(EnterSceneState::Post) => false,
            Some(EnterSceneState::Ready) => state == EnterSceneState::InitFinish,
            Some(EnterSceneState::InitFinish) => state == EnterSceneState::Done,
            Some(EnterSceneState::Done) => state == EnterSceneState::Post,
        } {
            self.cur_state = Some(state);
            true
        } else {
            false
        }
    }
}

#[derive(Resource, Default)]
pub struct ScenePeerManager {
    host_peer_id: u32,
    peer_map: HashMap<u32, u32>,
}

impl ScenePeerManager {
    pub fn get_or_add_peer(&mut self, player_uid: u32) -> u32 {
        if let Some((peer_id, _)) = self.peer_map.iter().find(|(_, uid)| **uid == player_uid) {
            return *peer_id;
        }

        let mut peer_id = 1;
        self.peer_map.keys().for_each(|id| {
            if peer_id == *id {
                peer_id += 1;
            }
        });

        self.peer_map.insert(peer_id, player_uid);
        peer_id
    }

    pub fn peer_count(&self) -> usize {
        self.peer_map.len()
    }

    pub fn make_host(&mut self, peer_id: u32) {
        self.host_peer_id = peer_id;
    }

    pub fn host_peer_id(&self) -> u32 {
        self.host_peer_id
    }

    pub fn get_peer_id_by_uid(&self, player_uid: u32) -> u32 {
        *self
            .peer_map
            .iter()
            .find(|(_, uid)| **uid == player_uid)
            .unwrap()
            .0
    }
}
