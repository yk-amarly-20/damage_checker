use super::super::efforts::effort_attack_point::EffortAttackPoint;
use super::super::efforts::effort_special_attack_point::EffortSpecialAttackPoint;
use super::super::individuals::individual_attack_point::IndividualAttackPoint;
use super::super::individuals::individual_special_attack_point::IndividualSpecialAttackPoint;
use super::super::other_status::level::Level;
use super::super::other_status::types::Types;
use super::super::skill::skill::Skill;

pub struct Attacker {
    level: Level,
    type1: Types,
    type2: Types,
    skill: Skill,
    attack_effort: EffortAttackPoint,
    special_attack_effort: EffortSpecialAttackPoint,
    attack_individual: IndividualAttackPoint,
    special_attack_individual: IndividualSpecialAttackPoint,
}

impl Attacker {
    pub fn new(
        level: usize,
        type1: Types,
        type2: Types,
        skill: Skill,
        attack_effort: usize,
        special_attack_effort: usize,
        attack_individual: usize,
        special_attack_individual: usize,
    ) -> Result<Attacker, &'static str> {
        let level_result = Level::new(level)?;
        let attack_effort_result = EffortAttackPoint::new(attack_effort)?;
        let special_attack_effort_result = EffortSpecialAttackPoint::new(special_attack_effort)?;
        let attack_individual_result = IndividualAttackPoint::new(attack_individual)?;
        let special_attack_individual_result =
            IndividualSpecialAttackPoint::new(special_attack_individual)?;

        Ok(Self {
            level: level_result,
            type1: type1,
            type2: type2,
            skill: skill,
            attack_effort: attack_effort_result,
            special_attack_effort: special_attack_effort_result,
            attack_individual: attack_individual_result,
            special_attack_individual: special_attack_individual_result,
        })
    }
}
