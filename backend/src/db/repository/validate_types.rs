// use super::super::domain::models::other_status::types::Types;
use crate::domain::models::other_status::types::Types;

// typeのvalidation
// 文字列が来るので、enumに変換
pub struct ValidateTypesRepository {}

impl ValidateTypesRepository {
    pub fn validate(types: Option<&str>) -> Types {
        match types {
            None => Types::None,
            Some("ノーマル") => Types::Normal,
            Some("ほのお") => Types::Fire,
            Some("みず") => Types::Water,
            Some("でんき") => Types::Electric,
            Some("くさ") => Types::Grass,
            Some("こおり") => Types::Ice,
            Some("かくとう") => Types::Fighting,
            Some("どく") => Types::Poison,
            Some("じめん") => Types::Ground,
            Some("ひこう") => Types::Flying,
            Some("エスパー") => Types::Psychic,
            Some("むし") => Types::Bug,
            Some("いわ") => Types::Rock,
            Some("ゴースト") => Types::Ghost,
            Some("ドラゴン") => Types::Dragon,
            Some("あく") => Types::Dark,
            Some("はがね") => Types::Steel,
            Some("フェアリー") => Types::Faily,
            Some(_) => Types::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::validate_types::ValidateTypesRepository;
    use crate::domain::models::other_status::types::Types;

    #[test]
    // タイプがnormalの時
    fn case_1() {
        let type_case_1: Option<&str> = Some("ノーマル");
        assert_eq!(
            ValidateTypesRepository::validate(type_case_1),
            Types::Normal
        );
    }

    #[test]
    // タイプがnullの時
    fn case_2() {
        let type_case_2: Option<&str> = None;
        assert_eq!(ValidateTypesRepository::validate(type_case_2), Types::None);
    }
}
