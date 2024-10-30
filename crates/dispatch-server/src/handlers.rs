use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use sakura_encryption::rsa::RsaKeyPair;
use sakura_proto::{
    Protobuf, QueryCurrRegionHttpRsp, QueryRegionListHttpRsp, RegionInfo, RegionSimpleInfo,
};
use serde::Deserialize;
use serde_json::json;

use crate::AppState;

pub fn routes() -> Router<&'static AppState> {
    Router::new()
        .route("/query_region_list", get(query_region_list))
        .route("/query_cur_region", get(query_cur_region))
}

#[derive(Deserialize, Debug)]
struct QueryRegionListParam {
    pub version: String,
    #[expect(dead_code)]
    pub lang: u32,
    #[expect(dead_code)]
    pub platform: u32,
    #[expect(dead_code)]
    pub binary: u8,
    #[expect(dead_code)]
    pub time: u64,
    pub channel_id: u16,
    pub sub_channel_id: u16,
}

#[derive(Deserialize, Debug)]
struct QueryCurrRegionParam {
    pub version: String,
    #[expect(dead_code)]
    pub lang: u32,
    #[expect(dead_code)]
    pub platform: u32,
    #[expect(dead_code)]
    pub binary: u8,
    #[expect(dead_code)]
    pub time: u64,
    pub channel_id: u16,
    pub sub_channel_id: u16,
    #[serde(rename = "dispatchSeed")]
    pub dispatch_seed: String,
    pub key_id: u32,
}

enum ProtobufData<T> {
    Plain(T),
    Encrypted(T, &'static RsaKeyPair),
}

impl<T> IntoResponse for ProtobufData<T>
where
    T: Protobuf,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Plain(proto) => {
                (StatusCode::OK, rbase64::encode(&proto.encode_to_vec())).into_response()
            }
            Self::Encrypted(proto, keys) => {
                let plain = proto.encode_to_vec();
                let content = keys.encrypt(&plain);
                let sign = keys.sign(&plain);

                (
                    StatusCode::OK,
                    json! ({
                        "content": rbase64::encode(&content),
                        "sign": rbase64::encode(&sign),
                    })
                    .to_string(),
                )
                    .into_response()
            }
        }
    }
}

async fn query_cur_region(
    Query(param): Query<QueryCurrRegionParam>,
    State(state): State<&'static AppState>,
) -> ProtobufData<QueryCurrRegionHttpRsp> {
    tracing::debug!("query_cur_region: {param:?}");

    let Some(cur_region_name) = state.config.region.cur_region_name.as_ref() else {
        tracing::debug!("query_cur_region requested, but this dispatch doesn't have bound region");
        return ProtobufData::Plain(QueryCurrRegionHttpRsp {
            retcode: -1,
            ..Default::default()
        });
    };

    let region_config = state
        .region_list
        .iter()
        .find(|r| &r.name == cur_region_name)
        .unwrap();

    if !region_config.allowed_key_id_list.contains(&param.key_id) {
        tracing::debug!(
            "query_cur_region: region {cur_region_name} doesn't allow key_id {}",
            param.key_id
        );
        return ProtobufData::Plain(QueryCurrRegionHttpRsp {
            retcode: -1,
            ..Default::default()
        });
    }

    let keys = state.key_pair_map.get(&param.key_id).unwrap();

    if region_config.channel_id != param.channel_id
        || region_config.sub_channel_id != param.sub_channel_id
        || !region_config.bind_version_list.contains(&param.version)
    {
        tracing::debug!(
            "Unsupported version (v={}, c={}, s={})",
            &param.version,
            param.channel_id,
            param.sub_channel_id
        );
        return ProtobufData::Encrypted(
            QueryCurrRegionHttpRsp {
                retcode: -1,
                ..Default::default()
            },
            keys,
        );
    }

    if region_config.dispatch_seed != param.dispatch_seed {
        tracing::debug!(
            "wrong dispatch seed, client: {}, server: {}",
            &param.dispatch_seed,
            &region_config.dispatch_seed
        );
        return ProtobufData::Encrypted(
            QueryCurrRegionHttpRsp {
                retcode: -1,
                ..Default::default()
            },
            keys,
        );
    }

    ProtobufData::Encrypted(
        QueryCurrRegionHttpRsp {
            retcode: 0,
            client_secret_key: state
                .cur_region_secret_key_ec2b
                .as_ref()
                .map(|k| k.to_vec())
                .unwrap_or_default(),
            region_info: Some(RegionInfo {
                gateserver_ip: region_config.gateserver_ip.clone(),
                gateserver_port: region_config.gateserver_port as u32,
                ..Default::default()
            }),
            ..Default::default()
        },
        keys,
    )
}

async fn query_region_list(
    Query(param): Query<QueryRegionListParam>,
    State(state): State<&'static AppState>,
) -> ProtobufData<QueryRegionListHttpRsp> {
    tracing::debug!("query_region_list: {param:?}");

    if state.config.forbid_first_dispatch {
        tracing::debug!("query_region_list is forbidden in this dispatch");

        ProtobufData::Plain(QueryRegionListHttpRsp {
            retcode: 8,
            ..Default::default()
        })
    } else {
        ProtobufData::Plain(QueryRegionListHttpRsp {
            enable_login_pc: state.config.region.enable_login_pc,
            client_secret_key: state.global_secret_key_ec2b.to_vec(),
            client_custom_config_encrypted: state.client_custom_config_encrypted.to_vec(),
            region_list: state
                .region_list
                .iter()
                .filter(|r| {
                    r.channel_id == param.channel_id
                        && r.sub_channel_id == param.sub_channel_id
                        && r.bind_version_list.contains(&param.version)
                })
                .map(|r| RegionSimpleInfo {
                    name: r.name.clone(),
                    title: r.title.clone(),
                    r#type: r.r#type.clone(),
                    dispatch_url: r.dispatch_url.clone(),
                })
                .collect(),
            ..Default::default()
        })
    }
}
