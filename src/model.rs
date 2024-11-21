use warp::reject::Rejection;

pub type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(serde::Serialize)]
pub struct Move {
    pub id: i32,
    pub name: String,
    pub move_type: String,
    pub category: String,
    pub power: Option<i32>,
    pub accuracy: Option<i32>,
    pub pp: Option<i32>,
    pub effect: Option<String>,
    pub probability: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct Ability {
    pub id: i32,
    pub name: String,
    pub pokemon_count: i32,
    pub effect: String,
    pub generation: i32,
}

#[derive(serde::Serialize)]
pub struct VersionInfoResponse {
    pub status: String,
    pub version: String,
}

#[derive(serde::Serialize)]
pub struct GenericFailure {
    pub status: String,
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct PokemonType {
    pub id: i32,
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub category: Option<String>,
    pub effect: Option<String>,
}

#[derive(serde::Serialize)]
pub struct BasePokemonData {
    pub id: i32,
    pub name: String,
    pub species: String,
    pub height: f32,
    pub weight: f32,
    pub catch_rate: i32,
    pub base_friendship: i32,
    pub base_experience: i32,
    pub growth_rate: String,
}
