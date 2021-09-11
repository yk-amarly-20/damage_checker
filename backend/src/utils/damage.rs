use crate::error_handler::check_level::check_level;


/// calculate damage
///
/// # Auguments
/// * `attack_status`           - A status of attack pokemon (A or C)
/// * `diffence_status`         - A status of diffence pokemon (B or D)
/// * `power`                   - A power of moves.
/// * `correction`              - correction (match of type correction etc...)
/// * `_level`                  - attacker level.
pub fn calculate_damage(attack_status: u32, diffence_status: u32,
                        power: u32, correction: f32, _level: u32) -> Result<u32, &'static str> {

    let level = check_level(_level)?;
    let damage: u32 = (((level * 2 / 5 + 2) * power * attack_status / diffence_status / 50 + 2) as f32
                        * correction) as u32;

    Ok(damage)
}
#[test]
fn test_calculate_damage() {
    let attack_case = 182;
    let diffence_case = 180;
    let power_case = 120;
    let correction_case = 1.0;
    let level_case = 50;

    assert_eq!(
        calculate_damage(attack_case, diffence_case, power_case, correction_case, level_case).ok(),
        Some(55)
    );

}
