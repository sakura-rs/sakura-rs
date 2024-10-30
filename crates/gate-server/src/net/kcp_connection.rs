use common::time_util;
use kcp::Kcp;
use std::{net::SocketAddr, sync::Arc, task};
use tokio::{io::AsyncWrite, net::UdpSocket, sync::mpsc};

use crate::handler::PacketHandler;

struct UdpOutput {
    peer_addr: SocketAddr,
    socket: Arc<UdpSocket>,
}

pub enum NetEvent {
    Recv(Box<[u8]>),
    Send(Box<[u8]>),
}

pub fn start(
    conv: u32,
    token: u32,
    socket: Arc<UdpSocket>,
    peer_addr: SocketAddr,
    handler: PacketHandler,
) -> mpsc::Sender<NetEvent> {
    let (tx, rx) = mpsc::channel(32);
    let kcp = Kcp::new(
        conv,
        token,
        time_util::unix_timestamp_ms(),
        false,
        UdpOutput { peer_addr, socket },
    );

    tokio::spawn(async move { kcp_loop(kcp, rx, handler).await });
    tx
}

async fn kcp_loop(
    mut kcp: Kcp<UdpOutput>,
    mut rx: mpsc::Receiver<NetEvent>,
    handler: PacketHandler,
) {
    let mut recv_buf = [0u8; 16384];
    while let Some(event) = rx.recv().await {
        match event {
            NetEvent::Recv(buf) => {
                kcp.input(&buf).unwrap();
                kcp.async_update((time_util::unix_timestamp_ms() - kcp.start_ts()) as u32)
                    .await
                    .unwrap();
                kcp.async_flush().await.unwrap();

                while let Ok(len) = kcp.recv(&mut recv_buf) {
                    handler.enqueue(kcp.conv(), recv_buf[..len].into());
                }
            }
            NetEvent::Send(buf) => {
                kcp.send(&buf).unwrap();
                kcp.async_flush().await.unwrap();
            }
        }
    }
}

impl AsyncWrite for UdpOutput {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, std::io::Error>> {
        self.socket.poll_send_to(cx, buf, self.peer_addr)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), std::io::Error>> {
        task::Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), std::io::Error>> {
        task::Poll::Ready(Ok(()))
    }
}
