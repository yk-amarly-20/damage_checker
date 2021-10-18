use super::effort_point::EffortPoint;

#[derive(Debug, Clone, Copy)]
pub struct EffortSpecialDefensePoint {
    value: EffortPoint,
}

impl EffortSpecialDefensePoint {
    pub fn new(value: usize) -> Result<EffortSpecialDefensePoint, &'static str> {
        let effort_result = EffortPoint::new(value);
        match effort_result {
            Ok(effort_result) => Ok(Self {
                value: effort_result,
            }),
            Err(err) => Err(err),
        }
    }
}
