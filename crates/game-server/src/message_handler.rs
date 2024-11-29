use mavuika_message::output::ClientOutput;
use mavuika_proto::{
    raw_packet::{make_raw_packet, RawPacket},
    CmdID, PacketHead, PlayerLoginReq, UnionCmdNotify,
};
use tokio::sync::mpsc;
use tracing::{debug, error, warn};

use crate::AppState;

pub async fn on_message(state: &'static AppState, data: Box<[u8]>) {
    let Ok(packet) = RawPacket::new(&data) else {
        debug!("received malformed packet, content: {}", hex::encode(&data));
        return;
    };

    debug!("received packet: {packet}");

    match packet.cmd_id() {
        PlayerLoginReq::CMD_ID => {
            let Ok(packet) = packet.decode_as::<PlayerLoginReq>() else {
                warn!(
                    "malformed login request received, content: {}",
                    hex::encode(&data)
                );
                return;
            };

            debug!(
                "received player login request, session id: {}, player uid: {}",
                packet.head.user_session_id, packet.head.user_id
            );

            player_login(
                state,
                packet.head.user_id,
                packet.head.user_session_id,
                packet.body,
            )
            .await;
        }
        UnionCmdNotify::CMD_ID => {
            let Ok(packet) = packet.decode_as::<UnionCmdNotify>() else {
                warn!(
                    "malformed union packet received, content: {}",
                    hex::encode(&data)
                );
                return;
            };

            for sub_cmd in packet.body.cmd_list {
                debug!(
                    "received union subcommand with cmd_id: {}",
                    sub_cmd.message_id
                );

                state.logic_simulator.add_client_packet(
                    packet.head.clone(),
                    sub_cmd.message_id as u16,
                    sub_cmd.body.into(),
                    false,
                );
            }

            state.logic_simulator.update_world(packet.head.user_id);
        }
        cmd_id => {
            debug!("received packet with cmd_id: {cmd_id}");
            state.logic_simulator.add_client_packet(
                packet.head(),
                cmd_id,
                packet.body().into(),
                true,
            );
        }
    }
}

async fn player_login(
    state: &'static AppState,
    user_id: u32,
    user_session_id: u32,
    _request: PlayerLoginReq,
) {
    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(packet_sink(state, user_id, user_session_id, rx));

    let Some(player_data) = state.db_handle.fetch(user_id).await else {
        error!("failed to get player data, uid: {user_id}");
        return;
    };

    state
        .logic_simulator
        .create_world(player_data, ClientOutput::new(tx));
}

async fn packet_sink(
    state: &'static AppState,
    user_id: u32,
    user_session_id: u32,
    mut rx: mpsc::Receiver<(u16, PacketHead, Box<[u8]>)>,
) {
    while let Some((cmd_id, head, body)) = rx.recv().await {
        state
            .gate_server_socket
            .send(make_raw_packet(
                cmd_id,
                PacketHead {
                    user_id,
                    user_session_id,
                    ..head
                },
                &body,
            ))
            .await;
    }
}
