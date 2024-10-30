use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct RegionConfig {
    pub name: String,
    pub title: String,
    pub r#type: String,
    pub bind_version_list: Box<[String]>,
    pub channel_id: u16,
    pub sub_channel_id: u16,
    pub allowed_key_id_list: Box<[u32]>,
    pub dispatch_url: String,
    pub dispatch_seed: String,
    pub gateserver_ip: String,
    pub gateserver_port: u16,
    pub secret_key_path: Option<String>,
}

#[derive(Deserialize)]
pub struct EncryptionConfig {
    #[serde(deserialize_with = "from_hex")]
    pub server_private_key: Box<[u8]>,
    #[serde(deserialize_with = "from_hex")]
    pub client_public_key: Box<[u8]>,
}

pub fn from_hex<'de, D>(deserializer: D) -> Result<Box<[u8]>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer).and_then(|string| {
        hex::decode(&string)
            .map(|vec| vec.into_boxed_slice())
            .map_err(|err| Error::custom(err.to_string()))
    })
}
