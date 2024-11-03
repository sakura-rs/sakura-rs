mod get_player_token;
mod protocol_util;
pub mod server_message_handler;

use std::sync::{Arc, OnceLock};

use anyhow::Result;
use protocol_util::convert_union_cmd_notify_data;
use sakura_encryption::xor::MhyXorpad;
use sakura_proto::{
    packet::{self, NetPacket},
    raw_packet::{make_raw_packet, RawPacket},
    CmdID, GetPlayerTokenReq, PacketHead, PingReq, PingRsp, Protobuf, Retcode, UnionCmdNotify,
};
use tokio::sync::mpsc;
use tracing::{debug, warn};

use crate::{net::Connection, util, AppState};

enum InputItem {
    NewConnection(Connection),
    DropConnection(u32),
    Packet(u32, Box<[u8]>),
}

#[derive(Clone)]
pub struct PacketHandler(mpsc::UnboundedSender<InputItem>);

pub struct Session {
    pub connection: Connection,
    pub account_uid: OnceLock<String>,
    pub player_uid: OnceLock<u32>,
    pub xorpad: OnceLock<MhyXorpad>,
}

impl PacketHandler {
    pub fn new(state: &'static AppState) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(async move { packet_handler_loop(rx, state).await });

        Self(tx)
    }

    pub fn add_connection(&self, conn: Connection) {
        self.0.send(InputItem::NewConnection(conn)).unwrap();
    }

    pub fn remove_connection(&self, id: u32) {
        self.0.send(InputItem::DropConnection(id)).unwrap();
    }

    pub fn enqueue(&self, conn_id: u32, buf: Box<[u8]>) {
        self.0.send(InputItem::Packet(conn_id, buf)).unwrap();
    }
}

async fn packet_handler_loop(mut rx: mpsc::UnboundedReceiver<InputItem>, state: &'static AppState) {
    while let Some(item) = rx.recv().await {
        match item {
            InputItem::NewConnection(conn) => {
                state.sessions.insert(
                    conn.conv,
                    Arc::new(Session {
                        connection: conn,
                        account_uid: OnceLock::new(),
                        player_uid: OnceLock::new(),
                        xorpad: OnceLock::new(),
                    }),
                );
            }
            InputItem::DropConnection(id) => {
                state.sessions.remove(&id);
            }
            InputItem::Packet(id, buf) => {
                if let Some(session) = state.sessions.get(&id) {
                    if let Err(err) = handle_packet(state, &session, buf).await {
                        warn!("handle_packet(connection_id: {id}) failed, error: {err}");
                    }
                }
            }
        }
    }
}

async fn handle_packet(
    state: &'static AppState,
    session: &Arc<Session>,
    mut data: Box<[u8]>,
) -> Result<()> {
    util::xor_packet(
        session.xorpad.get(),
        state.initial_xorpad.as_ref(),
        &mut data,
    );

    debug!("received packet: {}", hex::encode(&data));

    let packet = RawPacket::new(&data)?;
    let head = packet.head();
    let (cmd_id, body) = packet::client_to_normal(packet.cmd_id(), packet.body())?;

    match cmd_id {
        GetPlayerTokenReq::CMD_ID => {
            let request = GetPlayerTokenReq::decode(body.as_ref())?;
            let session = session.clone();

            // Run in separate task because db fetch operation may take a while
            tokio::spawn(async move {
                let Ok(uid) =
                    util::fetch_user_uid(&state.db_connection, &request.account_uid).await
                else {
                    return;
                };

                let rsp = get_player_token::process_message(state, &session, request, uid);
                let (cmd_id, body) =
                    packet::normal_to_client(rsp.get_cmd_id(), &rsp.encode_to_vec()).unwrap();

                let mut data = make_raw_packet(cmd_id, PacketHead::default(), &body);
                util::xor_packet(None, state.initial_xorpad.as_ref(), &mut data);

                session.connection.send(data).await;
            });
        }
        PingReq::CMD_ID => {
            let request = PingReq::decode(body.as_ref())?;

            let (cmd_id, body) = packet::normal_to_client(
                PingRsp::CMD_ID,
                &PingRsp {
                    retcode: Retcode::RetSucc.into(),
                    client_time: request.client_time,
                    seq: request.seq,
                }
                .encode_to_vec(),
            )
            .unwrap();

            let mut data = make_raw_packet(cmd_id, PacketHead::default(), &body);
            util::xor_packet(
                session.xorpad.get(),
                state.initial_xorpad.as_ref(),
                &mut data,
            );

            session.connection.send(data).await;
        }
        UnionCmdNotify::CMD_ID => {
            let union_cmd_notify = UnionCmdNotify::decode(body.as_ref())?;
            let mut new_packet = NetPacket::new(convert_union_cmd_notify_data(union_cmd_notify));

            new_packet.head.user_session_id = session.connection.conv;
            new_packet.head.user_id = session.player_uid.get().copied().unwrap_or(0);

            state.game_server_socket.send(new_packet.encode()).await;
        }
        _ => {
            state
                .game_server_socket
                .send(make_raw_packet(
                    cmd_id,
                    PacketHead {
                        user_session_id: session.connection.conv,
                        user_id: session.player_uid.get().copied().unwrap_or_default(),
                        ..head
                    },
                    &body,
                ))
                .await
        }
    }

    Ok(())
}
