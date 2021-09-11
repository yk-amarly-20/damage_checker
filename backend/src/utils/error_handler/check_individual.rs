pub fn check_individual(individual: u32) -> Result<u32, &'static str> {

    if individual >= 32 {
        Err("individual value is too big!")
    } else {
        Ok(individual)
    }
}

#[test]
fn test_check_individual() {
    let ind_case_1 = 20;
    let ind_case_2 = 56;

    assert_eq!(
        check_individual(ind_case_1).ok(),
        Some(20)
    );

    assert_eq!(
        check_individual(ind_case_2).err(),
        Some("individual value is too big!")
    );
}
