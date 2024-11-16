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
    pub slot: i32,
    pub name: String,
    pub effect: String,
    pub hidden: bool
}

#[derive(serde::Serialize)]
pub struct EVYield {
    pub stat: String,
    pub amount: i32
}

#[derive(serde::Serialize)]
pub struct Pokemon {
    pub id: i32,
    pub national_number: i32,
    pub types: Vec<String>,
    pub height: f32,
    pub weight: f32,
    pub abilities: Vec<Ability>,
    pub local_numbers: Vec<i32>,
    pub ev_yield: EVYield
    // catch rate
    // base friendship
    // growth rate
    // egg groups
    // gender ratios
    // egg cycles
    // base stats
    // type defenses
    // evolution chart,
    // entries
    // moves
    // locations
}
