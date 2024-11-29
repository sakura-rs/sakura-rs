use tokio::task::JoinHandle;
use tracing::error;
use zeromq::prelude::*;
use zeromq::{PullSocket, ZmqError};

use futures::future::BoxFuture;
use std::future::Future;

pub trait RecvCallback<S>: Send + Sync {
    fn call(&self, state: S, data: Box<[u8]>) -> BoxFuture<'static, ()>;
}

impl<T, F, S> RecvCallback<S> for T
where
    T: Fn(S, Box<[u8]>) -> F + Send + Sync,
    F: Future<Output = ()> + 'static + Send + Sync,
    S: Send + Sync,
{
    fn call(&self, state: S, data: Box<[u8]>) -> BoxFuture<'static, ()> {
        Box::pin(self(state, data))
    }
}

pub async fn listen<S: Send + Sync + Clone + 'static>(
    addr: &str,
    state: S,
    callback: impl RecvCallback<S> + 'static,
) -> Result<JoinHandle<()>, ZmqError> {
    let mut socket = PullSocket::new();
    socket.bind(&format!("tcp://{addr}")).await?;
    Ok(tokio::spawn(socket_recv_loop(socket, state, callback)))
}

async fn socket_recv_loop<S: Send + Sync + Clone>(
    mut socket: PullSocket,
    state: S,
    callback: impl RecvCallback<S>,
) {
    loop {
        let Ok(mut message) = socket
            .recv()
            .await
            .map(|m| m.into_vecdeque())
            .inspect_err(|err| error!("socket_recv_loop: recv failed: {err}"))
        else {
            continue;
        };

        while let Some(data) = message.pop_front() {
            tokio::spawn(callback.call(state.clone(), data.to_vec().into_boxed_slice()));
        }
    }
}
