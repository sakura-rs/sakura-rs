use sakura_data_derive::FromBinary;

#[derive(Debug, FromBinary)]
pub struct AvatarFlycloakExcelConfig {
    pub flycloak_id: u32,
    pub name: u32,
    pub desc: u32,
    pub prefab_path: String,
    pub json_name: String,
    pub icon: String,
    pub material_id: u32,
    pub hide: bool,
}
