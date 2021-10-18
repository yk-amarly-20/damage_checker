use super::individual_point::IndividualPoint;

#[derive(Debug, Copy, Clone)]
pub struct IndividualDefensePoint {
    value: IndividualPoint,
}

impl IndividualDefensePoint {
    pub fn new(value: usize) -> Result<IndividualDefensePoint, &'static str> {
        let individual_result = IndividualPoint::new(value);
        match individual_result {
            Ok(individual_result) => Ok(Self {
                value: individual_result,
            }),
            Err(err) => Err(err),
        }
    }
}
