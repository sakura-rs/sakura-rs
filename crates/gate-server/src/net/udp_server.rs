use std::{collections::HashMap, fmt, net::SocketAddr, sync::Arc};

use kcp::KCP_OVERHEAD;
use rand::RngCore;
use tokio::{net::UdpSocket, sync::mpsc};
use tracing::debug;

use crate::{handler::PacketHandler, AppState};

use super::{
    control_packet::{ControlPacket, ControlPacketType, CONTROL_PACKET_SIZE},
    kcp_connection::{self, NetEvent},
};

pub struct UdpServer {
    socket: Arc<UdpSocket>,
    packet_handler: PacketHandler,
}

impl UdpServer {
    pub async fn bind(addr: SocketAddr, state: &'static AppState) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(addr).await?;
        let packet_handler = PacketHandler::new(state);

        Ok(Self {
            socket: Arc::new(socket),
            packet_handler,
        })
    }

    pub async fn serve(self) {
        let mut conn_mgr = ConnectionManager::default();
        let mut buf = [0u8; 1400];

        loop {
            let Ok((len, addr)) = self.socket.recv_from(&mut buf).await else {
                continue;
            };

            match len {
                CONTROL_PACKET_SIZE => {
                    if let Some(rsp_control_packet) = self
                        .handle_control_packet(
                            &mut conn_mgr,
                            buf[..CONTROL_PACKET_SIZE].try_into().unwrap(),
                            addr,
                        )
                        .await
                    {
                        let _ = self
                            .socket
                            .send_to(rsp_control_packet.as_slice(), addr)
                            .await;
                    }
                }
                KCP_OVERHEAD.. => {
                    let buf = &buf[..len];
                    let (conv, token) = (kcp::get_conv(buf), kcp::get_token(buf));

                    if let Some(connection) = conn_mgr.get(conv, token) {
                        let _ = connection.event_tx.send(NetEvent::Recv(buf.into())).await;
                    }
                }
                _ => (),
            }
        }
    }

    async fn handle_control_packet(
        &self,
        conn_mgr: &mut ConnectionManager,
        pk: [u8; CONTROL_PACKET_SIZE],
        addr: SocketAddr,
    ) -> Option<ControlPacket> {
        let packet = ControlPacket::try_from(pk)
            .inspect_err(|err| debug!("failed to decode ControlPacket: {err}"))
            .ok()?;

        match packet.get_type() {
            ControlPacketType::Connect => {
                let conn = conn_mgr.create(self.socket.clone(), addr, self.packet_handler.clone());
                self.packet_handler.add_connection(conn.clone());
                debug!("new connection from {addr}, id: {conn}");

                Some(ControlPacket::build(
                    ControlPacketType::Establish,
                    conn.conv,
                    conn.token,
                    0,
                ))
            }
            ControlPacketType::Disconnect => {
                if let Some(id) = conn_mgr.remove(packet.get_conv(), packet.get_token()) {
                    self.packet_handler.remove_connection(id.conv);
                    debug!("client from {addr}, id: {id} disconnected");
                    Some(ControlPacket::build(
                        ControlPacketType::Disconnect,
                        id.conv,
                        id.token,
                        packet.get_data(),
                    ))
                } else {
                    None
                }
            }
            ControlPacketType::Establish => None,
        }
    }
}

#[derive(Clone)]
pub struct Connection {
    pub conv: u32,
    pub token: u32,
    event_tx: mpsc::Sender<NetEvent>,
}

impl Connection {
    pub async fn send(&self, data: Box<[u8]>) {
        let _ = self.event_tx.send(NetEvent::Send(data)).await;
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}|{}]", self.conv, self.token)
    }
}

#[derive(Default)]
struct ConnectionManager {
    connections: HashMap<u32, Connection>,
    connection_counter: u32,
}

impl ConnectionManager {
    fn create(
        &mut self,
        socket: Arc<UdpSocket>,
        addr: SocketAddr,
        handler: PacketHandler,
    ) -> &Connection {
        self.connection_counter += 1;
        let (conv, token) = (self.connection_counter, rand::thread_rng().next_u32());
        let event_tx = kcp_connection::start(conv, token, socket, addr, handler);

        let id = Connection {
            conv,
            token,
            event_tx,
        };

        self.connections.entry(id.conv).or_insert(id)
    }

    pub fn get(&self, conv: u32, token: u32) -> Option<&Connection> {
        self.connections
            .get(&conv)
            .and_then(|c| (c.token == token).then_some(c))
    }

    pub fn remove(&mut self, conv: u32, token: u32) -> Option<Connection> {
        if let Some(id) = self.connections.get(&conv) {
            if id.token == token {
                return self.connections.remove(&conv);
            }
        }

        None
    }
}
