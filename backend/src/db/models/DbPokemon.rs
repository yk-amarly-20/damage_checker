use chrono::NaiveDateTime;

#[derive(Queryable, Clone, Debug)]
pub struct DbPokemon {
    id: i16,
    book_number: i16,
    name: String,
    en_name: String,
    weight: f32,
    ketaguri: i16,
    type1: String,
    type2: Option<String>,
    status_h: i16,
    status_a: i16,
    status_b: i16,
    status_c: i16,
    status_d: i16,
    status_s: i16,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
