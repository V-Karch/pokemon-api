use crate::model;
use sqlx::{sqlite::SqlitePool, Pool};

pub async fn connect() -> Result<Pool<sqlx::Sqlite>, sqlx::Error> {
    let pool: Pool<sqlx::Sqlite> = SqlitePool::connect("sqlite:pokemon.db").await?;
    return Ok(pool);
}

pub async fn ability_to_json(ability_object: &Option<model::Ability>) -> String {
    return match ability_object {
        Some(value) => match serde_json::to_string(&value) {
            Ok(inner_value) => inner_value,
            Err(_) => "Failed to serailize Ability object".to_string()
        },
        None => "[Move] was empty".to_string(),
    }
}

pub async fn move_to_json(move_object: &Option<model::Move>) -> String {
    return match move_object {
        Some(value) => match serde_json::to_string(&value) {
            Ok(inner_value) => inner_value,
            Err(_) => "Failed to serialize Move object".to_string(),
        },
        None => "[Move] was empty".to_string(),
    };
}