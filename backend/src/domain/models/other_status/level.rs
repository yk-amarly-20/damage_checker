pub struct Level {
    value: usize,
}

impl Level {
    pub fn new(value: usize) -> Result<Level, &'static str> {
        if 1 <= value && value <= 100 {
            Ok(Self { value: value })
        } else {
            Err("level must be more than 1 and less than 100")
        }
    }
}
