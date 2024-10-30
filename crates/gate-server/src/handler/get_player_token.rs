use sakura_encryption::xor::{MhyXorpad, XorpadGenerationMethod};
use sakura_proto::{GetPlayerTokenReq, GetPlayerTokenRsp, Retcode};
use rand::RngCore;
use tracing::{debug, error};

use crate::AppState;

use super::Session;

pub fn process_message(
    state: &AppState,
    session: &Session,
    req: GetPlayerTokenReq,
    uid: u32,
) -> GetPlayerTokenRsp {
    let mut rsp = GetPlayerTokenRsp {
        retcode: Retcode::RetFail.into(),
        ..Default::default()
    };

    if let Some(account_uid) = session.account_uid.get() {
        debug!("repeated GetPlayerTokenReq (account_uid: {account_uid})");
        return rsp;
    }

    let Some(xorpad) = gen_session_key(state, &req, &mut rsp) else {
        return rsp;
    };

    let _ = session.xorpad.set(xorpad);
    let _ = session.account_uid.set(req.account_uid);
    let _ = session.player_uid.set(uid);

    GetPlayerTokenRsp {
        retcode: Retcode::RetSucc.into(),
        uid,
        ..rsp
    }
}

fn gen_session_key(
    state: &AppState,
    req: &GetPlayerTokenReq,
    rsp: &mut GetPlayerTokenRsp,
) -> Option<MhyXorpad> {
    if !state
        .region_config
        .allowed_key_id_list
        .contains(&req.key_id)
    {
        debug!(
            "client key id ({}) is not allowed by region config",
            req.key_id
        );
        return None;
    }

    let Some(keys) = state.key_pair_map.get(&req.key_id) else {
        error!(
            "key id {} is allowed by region but doesn't exist in encryption config",
            req.key_id
        );
        return None;
    };

    let Ok(client_rand_key) = rbase64::decode(&req.client_rand_key) else {
        debug!(
            "failed to decode client_rand_key as base64, content: {}",
            req.client_rand_key
        );
        return None;
    };

    let Some(client_rand_key) = keys.decrypt(&client_rand_key) else {
        debug!(
            "failed to decrypt client_rand_key using key_id: {}",
            req.key_id
        );
        return None;
    };

    let client_rand_key = u64::from_be_bytes(
        client_rand_key
            .try_into()
            .inspect_err(|_| debug!("client_rand_key is not uint64"))
            .ok()?,
    );

    let server_rand_key = rand::thread_rng().next_u64();

    rsp.server_rand_key = rbase64::encode(&keys.encrypt(&server_rand_key.to_be_bytes()));
    rsp.sign = rbase64::encode(&keys.sign(&server_rand_key.to_be_bytes()));

    Some(MhyXorpad::new::<byteorder::BE>(
        client_rand_key ^ server_rand_key,
        XorpadGenerationMethod::ReseedWithSkip,
    ))
}
