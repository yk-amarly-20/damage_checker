use crate::error_handler::check_base::check_base;
use crate::error_handler::check_individual::check_individual;
use crate::error_handler::check_effort::check_effort;
use crate::error_handler::check_level::check_level;

pub fn calc_hit_point(_base: u32, _individual: u32, _effort: u32, _level: u32) -> Result<u32, &'static str> {

    let base = check_base(_base)?;
    let individual = check_individual(_individual)?;
    let effort = check_effort(_effort)?;
    let level = check_level(_level)?;

    let _hit_point: u32 = (base * 2 + individual + effort / 4) * level / 100;
    let hit_point: u32 = _hit_point + level + 10;

    Ok(hit_point)
}
