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
