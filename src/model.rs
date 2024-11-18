use warp::reject::Rejection;

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
    pub generation: i32
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
pub struct Representative {
    pub id: i32,
    pub national_number: i32,
    pub types: Vec<String>, // Link other table
    pub height: f32,
    pub weight: f32,
    pub abilities: i32, // Link other table,
    pub local_numbers: Vec<i32>, // Link other table
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spatk: i32,
    pub spdef: i32,
    pub speed: i32,
    pub base_friendship: i32,
    pub catch_rate: i32,
    // ev_yield
    pub growth_rate: i32,
    // egg groups
    // gender ratios
    // egg cycles
    // type defenses
    // evolution chart,
    pub entry: Vec<String>, // Link to other table
    // locations
}

pub type WebResult<T> = std::result::Result<T, Rejection>;
