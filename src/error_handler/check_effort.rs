pub fn check_effort(effort: u32) -> Result<u32, &'static str> {

    if effort >= 253 {
        Err("effort value is too big!")
    } else if effort % 4 != 0 {
        Err("effort value must be a multiple of 4")
    } else {
        Ok(effort)
    }
}

#[test]
fn test_check_effort() {
    let eff_case_1 = 4;
    let eff_case_2 = 400;
    let eff_case_3 = 54;

    assert_eq!(
        check_effort(eff_case_1).ok(),
        Some(4)
    );

    assert_eq!(
        check_effort(eff_case_2).err(),
        Some("effort value is too big!")
    );

    assert_eq!(
        check_effort(eff_case_3).err(),
        Some("effort value must be a multiple of 4")
    );

}
