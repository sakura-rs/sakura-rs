use std::{collections::HashMap, thread};

use crate::{command::LogicCommand, player_world::PlayerWorld};
use common::time_util;
use sakura_message::output::ClientOutput;
use sakura_persistence::player_information::PlayerInformation;
use sakura_proto::PacketHead;
use std::sync::mpsc;

#[derive(Clone)]
pub struct LogicSimulator(mpsc::Sender<LogicCommand>);

impl LogicSimulator {
    pub fn spawn(save_data_tx: tokio::sync::mpsc::Sender<(u32, serde_json::Value)>) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(|| simulation_loop(rx, save_data_tx));
        Self(tx)
    }

    pub fn create_world(&self, player_information: PlayerInformation, output: ClientOutput) {
        self.0
            .send(LogicCommand::CreateWorld {
                player_information,
                output,
            })
            .unwrap();
    }

    pub fn add_client_packet(
        &self,
        head: PacketHead,
        cmd_id: u16,
        data: Box<[u8]>,
        immediate_mode: bool,
    ) {
        self.0
            .send(LogicCommand::ClientInput {
                head,
                cmd_id,
                data,
                immediate_mode,
            })
            .unwrap();
    }

    pub fn update_world(&self, uid: u32) {
        self.0.send(LogicCommand::WorldUpdate(uid)).unwrap();
    }
}

fn simulation_loop(
    command_receiver: mpsc::Receiver<LogicCommand>,
    save_data_tx: tokio::sync::mpsc::Sender<(u32, serde_json::Value)>,
) {
    // client_player_uid -> world_owner_uid
    let mut player_uid_map: HashMap<u32, u32> = HashMap::new();
    let mut player_world_map: HashMap<u32, PlayerWorld> = HashMap::new();
    let mut player_save_time_map: HashMap<u32, u64> = HashMap::new();

    while let Ok(command) = command_receiver.recv() {
        use LogicCommand::*;
        match command {
            CreateWorld {
                player_information,
                output,
            } => {
                player_save_time_map.insert(player_information.uid, time_util::unix_timestamp());
                player_uid_map.insert(player_information.uid, player_information.uid);
                player_world_map.insert(
                    player_information.uid,
                    PlayerWorld::new(player_information, output),
                );
            }
            ClientInput {
                head,
                cmd_id,
                data,
                immediate_mode,
            } => {
                let uid = head.user_id;
                if let Some(world_owner_uid) = player_uid_map.get(&uid) {
                    if let Some(world) = player_world_map.get_mut(world_owner_uid) {
                        world.add_packet(head, cmd_id, data);
                        if immediate_mode {
                            world.update();
                        }

                        let save_time = player_save_time_map.get_mut(&uid).unwrap();
                        let cur_time = time_util::unix_timestamp();
                        if (cur_time - *save_time) >= 30 {
                            *save_time = cur_time;
                            let _ = save_data_tx
                                .blocking_send((uid, world.serialize_player_information(uid)));
                        }
                    }
                }
            }
            WorldUpdate(uid) => {
                if let Some(world_owner_uid) = player_uid_map.get(&uid) {
                    if let Some(world) = player_world_map.get_mut(world_owner_uid) {
                        world.update();
                    }
                }
            }
        }
    }
}
