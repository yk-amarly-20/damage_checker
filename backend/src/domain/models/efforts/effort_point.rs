#[derive(Debug, Clone, Copy)]
pub struct EffortPoint {
    value: usize,
}

impl EffortPoint {
    pub fn new(value: usize) -> Result<EffortPoint, &'static str> {
        if value <= 252 && value % 4 == 0 {
            Ok(Self { value: value })
        } else {
            Err("effort point must be less than 252 and multiple of 4.")
        }
    }
}
