use mavuika_data_derive::FromBinary;

#[derive(Debug, FromBinary)]
pub struct ProudSkillOpenConfig {
    pub proud_skill_group_id: u32,
    pub need_avatar_promote_level: u32,
}

#[derive(Debug, FromBinary)]
pub struct AvatarSkillDepotExcelConfig {
    pub id: u32,
    pub energy_skill: u32,
    pub talent_skill: u32,
    pub skills: Vec<u32>,
    pub sub_skills: Vec<u32>,
    pub attack_mode_skill: u32,
    pub leader_talent: u32,
    pub extra_abilities: Vec<String>,
    pub talents: Vec<u32>,
    pub talent_star_name: String,
    pub core_proud_skill_group_id: u32,
    pub core_proud_avatar_protomote_level: u32,
    pub inherent_proud_skill_opens: Vec<ProudSkillOpenConfig>,
    pub skill_depot_ability_group: String,
    pub unk_1: u32,
    pub unk_2: String,
}
