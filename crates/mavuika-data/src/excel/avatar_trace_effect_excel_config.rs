use mavuika_data_derive::FromBinary;

#[derive(Debug, FromBinary)]
pub struct AvatarTraceEffectExcelConfig {
    pub trace_effect_id: u32,
    pub avatar_id: u32,
    pub item_id: u32,
    pub unk_1: String,
    pub unk_2: Vec<String>,
    pub name_text_map_hash: u32,
    pub desc_text_map_hash: u32,
}
