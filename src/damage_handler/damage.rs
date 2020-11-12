use crate::error_handler::check_level::check_level;

pub fn calculate_damage(attack_status: u32, diffence_status: u32,
                        power: u32, correction: f32, _level: u32) -> Result<u32, &'static str> {

    let level = check_level(_level)?;
    let damage = (level * 2 / 5 + 2) * power * attack_status / diffence_status / 50 + 2;

    Ok(damage)
}
