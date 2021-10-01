use actix_web::{get, web, Responder};
use serde::Deserialize;
use crate::domain::models;
use models::other_status::types::Types;
use models::battle::attacker::Attacker;
use models

#[derive(Deserialize)]
pub struct DamageCaluculationParams {
    attacker_name: String,
    attacker_level: usize,
    skill_name: String,
    attack_effort: Option<usize>,
    special_attack_effort: Option<usize>,
    defender_name: String,
    defender_level: usize,
    defense_effort: Option<usize>,
    special_defense_effort: Option<usize>,
    terrain: Option<String>,
    climate: Option<String>,
}

#[get("/caluculate/damage")]
async fn caluculate_damage(query: web::Query<DamageCaluculationParams>) -> impl Responder {
  // とりあえず適当なやつ置いておく

}
