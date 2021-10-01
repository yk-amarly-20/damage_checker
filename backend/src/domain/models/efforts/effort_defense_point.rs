use super::effort_point::EffortPoint;

#[derive(Debug, Clone, Copy)]
pub struct EffortDefensePoint {
    value: EffortPoint,
}

impl EffortDefensePoint {
    pub fn new(value: usize) -> Result<EffortDefensePoint, &'static str> {
        let effort_result = EffortPoint::new(value);
        match effort_result {
            Ok(effort_result) => Ok(Self {
                value: effort_result,
            }),
            Err(err) => Err(err),
        }
    }
}
