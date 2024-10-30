use sakura_data_derive::FromBinary;

#[derive(Debug, FromBinary)]
pub struct OpenStateConfig {
    pub id: u32,
    pub default_state: bool,
    pub allow_client_open: bool,
    pub unk: bool,
    pub cond: Vec<OpenStateCond>,
    pub system_open_ui_id: u32,
}

#[derive(Debug, FromBinary)]
pub struct OpenStateCond {
    pub cond_type: u32,
    pub param_1: u32,
    pub param_2: u32,
}
