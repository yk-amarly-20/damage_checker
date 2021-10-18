use super::individual_point::IndividualPoint;

#[derive(Debug, Copy, Clone)]
pub struct IndividualSpecialDefensePoint {
    value: IndividualPoint,
}

impl IndividualSpecialDefensePoint {
    pub fn new(value: usize) -> Result<IndividualSpecialDefensePoint, &'static str> {
        let individual_result = IndividualPoint::new(value);
        match individual_result {
            Ok(individual_result) => Ok(Self {
                value: individual_result,
            }),
            Err(err) => Err(err),
        }
    }
}
