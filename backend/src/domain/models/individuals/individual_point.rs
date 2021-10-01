// 能力値のステータスの値オブジェクト
#[derive(Debug, Copy, Clone)]
pub struct IndividualPoint {
    value: usize,
}

impl IndividualPoint {
    pub fn new(value: usize) -> Result<IndividualPoint, &'static str> {
        // validation
        // 値は999以下に設定
        if value <= 999 {
            Ok(Self { value: value })
        } else {
            Err("ability must be less than 999.")
        }
    }
}
