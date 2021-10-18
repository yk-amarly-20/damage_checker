use crate::db::connect::establish_connection;
use crate::db::models::DbPokemon::DbPokemon;
use crate::diesel::query_dsl::filter_dsl::FilterDsl;
use crate::diesel::query_dsl::limit_dsl::LimitDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::schema::pokemons::dsl::*;

pub struct PokemonFactory {}

impl PokemonFactory {
    pub fn load_pokemon(pokemon_name: String) -> Result<DbPokemon, &'static str> {
        let connection = establish_connection();

        let results = pokemons
            .filter(name.eq(pokemon_name))
            .limit(1)
            .load::<DbPokemon>(&connection)
            .expect("Error loading pokemon");

        let pokemon = results.get(0);

        match pokemon {
            Some(value) => Ok(value.as_ref()),

            None => {}
        }
    }
}
