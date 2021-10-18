table! {
    pokemons (id) {
        id -> Int2,
        book_number -> Int2,
        name -> Varchar,
        en_name -> Varchar,
        weight -> Float4,
        ketaguri -> Int2,
        type1 -> Varchar,
        type2 -> Nullable<Varchar>,
        status_h -> Int2,
        status_a -> Int2,
        status_b -> Int2,
        status_c -> Int2,
        status_d -> Int2,
        status_s -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    skills (id) {
        id -> Int2,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        dist -> Varchar,
        power -> Nullable<Int2>,
        dai_max_power -> Nullable<Int2>,
        correction -> Nullable<Float4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    pokemons,
    skills,
);
