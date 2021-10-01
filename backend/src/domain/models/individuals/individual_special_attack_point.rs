use super::individual_point::IndividualPoint;

#[derive(Debug, Copy, Clone)]
pub struct IndividualSpecialAttackPoint {
    value: IndividualPoint,
}

impl IndividualSpecialAttackPoint {
    pub fn new(value: usize) -> Result<IndividualSpecialAttackPoint, &'static str> {
        let individual_result = IndividualPoint::new(value);
        match individual_result {
            Ok(individual_result) => Ok(Self {
                value: individual_result,
            }),
            Err(err) => Err(err),
        }
    }
}
