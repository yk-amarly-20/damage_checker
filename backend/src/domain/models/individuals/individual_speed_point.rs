use super::individual_point::IndividualPoint;

#[derive(Debug, Clone, Copy)]
pub struct IndividualSpeedPoint {
    value: IndividualPoint,
}

impl IndividualSpeedPoint {
    pub fn new(value: usize) -> Result<IndividualSpeedPoint, &'static str> {
        let individual_result = IndividualPoint::new(value);
        match individual_result {
            Ok(individual_result) => Ok(Self {
                value: individual_result,
            }),
            Err(err) => Err(err),
        }
    }
}
