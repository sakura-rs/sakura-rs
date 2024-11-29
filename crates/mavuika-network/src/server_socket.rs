use std::time::Duration;

use tokio::sync::mpsc;
use zeromq::prelude::*;
use zeromq::PushSocket;

#[derive(Clone)]
pub struct ServerSocket(mpsc::Sender<Box<[u8]>>);

impl ServerSocket {
    const CONNECT_REPEAT_TIMEOUT: Duration = Duration::from_secs(2);

    pub fn new(addr: &str) -> Self {
        let (tx, rx) = mpsc::channel(64);
        tokio::spawn(Self::worker_loop(format!("tcp://{addr}"), rx));

        Self(tx)
    }

    pub async fn send(&self, data: Box<[u8]>) {
        let _ = self.0.send(data).await;
    }

    async fn worker_loop(endpoint: String, mut rx: mpsc::Receiver<Box<[u8]>>) {
        let mut socket = Self::connect_to(&endpoint).await;

        while let Some(buf) = rx.recv().await {
            while socket.send(buf.to_vec().into()).await.is_err() {
                socket = Self::connect_to(&endpoint).await;
            }
        }
    }

    async fn connect_to(endpoint: &str) -> PushSocket {
        let mut socket = PushSocket::new();

        while socket.connect(endpoint).await.is_err() {
            tokio::time::sleep(Self::CONNECT_REPEAT_TIMEOUT).await;
        }

        socket
    }
}
