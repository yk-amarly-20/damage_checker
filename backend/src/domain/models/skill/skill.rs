use super::super::other_status::types::Types;

pub struct Skill {
    skill_name: &'static str,
    skill_power: usize,
    skill_type: Types,
}

impl Skill {
    pub fn new(skill_name: &'static str, skill_power: usize, skill_type: Types) -> Skill {
        Self {
            skill_name: skill_name,
            skill_power: skill_power,
            skill_type: skill_type,
        }
    }
}
