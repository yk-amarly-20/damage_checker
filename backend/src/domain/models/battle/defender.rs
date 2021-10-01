use super::super::efforts::effort_defense_point::EffortDefensePoint;
use super::super::efforts::effort_special_defense_point::EffortSpecialDefensePoint;
use super::super::individuals::individual_defense_point::IndividualDefensePoint;
use super::super::individuals::individual_special_defense_point::IndividualSpecialDefensePoint;
use super::super::other_status::level::Level;
use super::super::other_status::types::Types;

pub struct Defender {
    level: Level,
    type1: Types,
    type2: Types,
    defense_effort: EffortDefensePoint,
    special_defense_effort: EffortSpecialDefensePoint,
    defense_individual: IndividualDefensePoint,
    special_defense_individual: IndividualSpecialDefensePoint,
}

impl Defender {
    pub fn new(
        level: usize,
        type1: Types,
        type2: Types,
        defense_effort: usize,
        special_defense_effort: usize,
        defense_individual: usize,
        special_defense_individual: usize,
    ) -> Result<Defender, &'static str> {
        let level_result = Level::new(level)?;
        let defense_effort_result = EffortDefensePoint::new(defense_effort)?;
        let special_defense_effort_result = EffortSpecialDefensePoint::new(special_defense_effort)?;
        let defense_individual_result = IndividualDefensePoint::new(defense_individual)?;
        let special_defense_individual_result =
            IndividualSpecialDefensePoint::new(special_defense_individual)?;

        Ok(Self {
            level: level_result,
            type1: type1,
            type2: type2,
            defense_effort: defense_effort_result,
            special_defense_effort: special_defense_effort_result,
            defense_individual: defense_individual_result,
            special_defense_individual: special_defense_individual_result,
        })
    }
}
