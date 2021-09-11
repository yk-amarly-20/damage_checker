pub fn check_base(base: u32) -> Result<u32, &'static str> {
    if base >= 300 {
        Err("base value is too big!")
    } else {
        Ok(base)
    }
}

#[test]
fn test_check_base() {
    let base_case_1 = 200;
    let base_case_2 = 500;

    assert_eq!(
        check_base(base_case_1).ok(),
        Some(200)
    );

    assert_eq!(
        check_base(base_case_2).err(),
        Some("base value is too big!")
    );
}
