use sakura_database::{sql_op, DbConnection};
use sakura_persistence::player_information::PlayerInformation;
use tokio::{
    select,
    sync::{mpsc, oneshot},
};
use tracing::error;

use crate::player_info_util;

enum DbOperation {
    Fetch(u32, oneshot::Sender<Option<PlayerInformation>>),
}

pub struct DbWorkerHandle(mpsc::Sender<DbOperation>);

impl DbWorkerHandle {
    pub async fn fetch(&self, uid: u32) -> Option<PlayerInformation> {
        let (tx, rx) = oneshot::channel();
        let _ = self.0.send(DbOperation::Fetch(uid, tx)).await;

        rx.await.ok().flatten()
    }
}

pub fn start(connection: DbConnection) -> (DbWorkerHandle, mpsc::Sender<(u32, serde_json::Value)>) {
    let (op_tx, op_rx) = mpsc::channel(32);
    let (save_data_tx, save_data_rx) = mpsc::channel(32);

    tokio::spawn(async move {
        db_work_loop(connection, op_rx, save_data_rx).await;
    });

    (DbWorkerHandle(op_tx), save_data_tx)
}

async fn db_work_loop(
    connection: DbConnection,
    mut op_rx: mpsc::Receiver<DbOperation>,
    mut save_data_rx: mpsc::Receiver<(u32, serde_json::Value)>,
) {
    loop {
        select! {
            op = op_rx.recv() => {
                if let Some(DbOperation::Fetch(uid, tx)) = op {
                    let result = match sql_op::select_player_data_by_uid(&connection, uid as i32).await
                {
                    Ok(Some(row)) => Some(serde_json::from_value(row.data.0).unwrap()),
                    Ok(None) => Some(player_info_util::create_default_player_information(
                        uid,
                        String::from("sakura-rs"),
                    )),
                    Err(_) => None,
                };

                let _ = tx.send(result);
                }
            },
            save_data = save_data_rx.recv() => {
                if let Some((uid, data)) = save_data {
                    if let Err(err) =
                        sql_op::insert_or_update_player_data(&connection, uid as i32, data).await
                    {
                        error!("failed to save player data: {err}");
                    }
                }
            }
        }
    }
}
