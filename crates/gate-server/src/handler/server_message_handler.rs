use mavuika_proto::{
    packet::normal_to_client,
    raw_packet::{make_raw_packet, RawPacket},
};
use tracing::{debug, warn};

use crate::{util, AppState};

pub async fn on_message(state: &'static AppState, data: Box<[u8]>) {
    debug!("on_message: {}", hex::encode(&data));

    let Ok(packet) = RawPacket::new(&data) else {
        return;
    };

    let head = packet.head();
    let session_id = head.user_session_id;

    if let Some(session) = state.sessions.get(&session_id) {
        match normal_to_client(packet.cmd_id(), packet.body()) {
            Ok((cmd_id, data)) => {
                let mut data = make_raw_packet(cmd_id, head, &data);
                util::xor_packet(
                    session.xorpad.get(),
                    state.initial_xorpad.as_ref(),
                    &mut data,
                );

                session.connection.send(data).await;
            }
            Err(err) => warn!("normal_to_client: conversion failed: {err}"),
        }
    } else {
        warn!("on_message: session with id: {session_id} not found");
    }
}
