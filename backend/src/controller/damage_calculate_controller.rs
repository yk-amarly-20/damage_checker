use crate::domain::models;
use actix_web::{get, web, Responder};
use models::battle::attacker::Attacker;
use models::battle::defender::Defender;
use models::other_status::types::Types;
use models::skill::skill::Skill;
use serde::Deserialize;

/*
// TODO: いろいろ整ったら引数を一部nullableに
#[derive(Deserialize)]
pub struct DamageCaluculationParams {
    attacker_name: String,
    attacker_level: usize,
    skill_name: String,
    // attack_effort: Option<usize>,
    attack_effort: usize,
    // special_attack_effort: Option<usize>,
    special_attack_effort: usize,
    defender_name: String,
    defender_level: usize,
    // defense_effort: Option<usize>,
    defense_effort: usize,
    // special_defense_effort: Option<usize>,
    special_defense_effort: usize,
    terrain: Option<String>,
    climate: Option<String>,
}

#[get("/caluculate/damage")]
async fn caluculate_damage(query: web::Query<DamageCaluculationParams>) -> impl Responder {
    // とりあえず適当なやつ置いておく

    let attacker = Attacker::new(
        query.attacker_level,
        Types::Normal,
        Types::Flying,
        Skill::new("test", 50, Types::Flying),
        query.attack_effort,
        query.special_attack_effort,
        32,
        30,
    );
}
*/
