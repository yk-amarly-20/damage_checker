use crate::error_handler::check_base::check_base;
use crate::error_handler::check_individual::check_individual;
use crate::error_handler::check_effort::check_effort;
use crate::error_handler::check_level::check_level;

/// calculate status of A, B, C, D, S.
///
/// # Augments
/// * `_base`           - A base value.
/// * `_individual`     - A individual value.
///                       This must be between 0 and 31.
/// * `_effort`         - A effort value.
///                       This must be between 0 and 252, and multiple of 4.
/// * `_level`          - A level.
///                       This must be between 1 and 100.
pub fn calc_main_status(_base: u32, _individual: u32, _effort: u32, _level: u32, ) -> Result<u32, &'static str> {

    // check each value.
    let base = check_base(_base)?;
    let individual = check_individual(_individual)?;
    let effort = check_effort(_effort)?;
    let level = check_level(_level)?;

    // calculate status.
    let status: u32 = (base * 2 + individual + effort / 4) * level / 100 + 5;

    Ok(status)
}

#[test]
fn test_calc_main_status() {
    let base_case_1 = 130;
    let ind_case_1 = 31;
    let effort_case_1 = 252;
    let level_case_1 = 50;

    let base_case_2 = 130;
    let ind_case_2 = 31;
    let effort_case_2 = 252;
    let level_case_2 = 500;

    assert_eq!(
        calc_main_status(base_case_1, ind_case_1, effort_case_1, level_case_1).ok(),
        Some(182)
    );

    assert_eq!(
        calc_main_status(base_case_2, ind_case_2, effort_case_2, level_case_2).err(),
        Some("level must to be between 1 and 100")
    );
}
