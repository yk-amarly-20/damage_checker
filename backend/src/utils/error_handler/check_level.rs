pub fn check_level(level: u32) -> Result<u32, &'static str> {

    if (level <= 1) || (level > 100) {
        Err("level must to be between 1 and 100")
    } else {
        Ok(level)
    }
}

#[test]
fn test_check_level() {
    let level_case_1 = 50;
    let level_case_2 = 103;
    let level_case_3 = 0;

    assert_eq!(
        check_level(level_case_1).ok(),
        Some(50)
    );

    assert_eq!(
        check_level(level_case_2).err(),
        Some("level must to be between 1 and 100")
    );

    assert_eq!(
        check_level(level_case_3).err(),
        Some("level must to be between 1 and 100")
    );
}
