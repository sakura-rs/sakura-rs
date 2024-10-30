use sakura_data_derive::FromBinary;

#[derive(Debug, FromBinary)]
pub struct MapAreaConfig {
    pub id: u32,
    pub r#type: i32,
    pub scene_id: u32,
    pub name: String,
    pub area_id_1: Vec<u32>,
    pub scene_point_id: u32,
    pub map_area_state: i32,
}
