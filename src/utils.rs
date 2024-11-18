use crate::model;
use sqlx::{sqlite::SqlitePool, Pool, Row};

pub async fn connect() -> Result<Pool<sqlx::Sqlite>, sqlx::Error> {
    let pool: Pool<sqlx::Sqlite> = SqlitePool::connect("sqlite:pokemon.db").await?;
    return Ok(pool);
}

pub async fn search_move_by_name(pool: &sqlx::SqlitePool, move_name: &str) -> Option<model::Move> {
    if let Ok(row) = sqlx::query("SELECT * FROM moves WHERE name = ?")
        .bind(move_name.replace(" ", "-"))
        .fetch_one(pool)
        .await
    {
        return Some(model::Move {
            id: row.get("id"),
            name: row.get("name"),
            move_type: row.get("type"),
            category: row.get("category"),
            power: row.try_get("power").ok(),
            accuracy: row.try_get("accuracy").ok(),
            pp: row.try_get("PP").ok(),
            effect: row.try_get("effect").ok(),
            probability: row.try_get("probability").ok(),
        })
    } else {
        return None;
    }
}

pub async fn search_move_by_id(pool: &sqlx::SqlitePool, move_id: i32) -> Option<model::Move> {
    if let Ok(row) = sqlx::query("SELECT * FROM moves WHERE id = ?")
        .bind(move_id)
        .fetch_one(pool)
        .await
    {
        return Some(model::Move {
            id: row.get("id"),
            name: row.get("name"),
            move_type: row.get("type"),
            category: row.get("category"),
            power: row.try_get("power").ok(),
            accuracy: row.try_get("accuracy").ok(),
            pp: row.try_get("PP").ok(),
            effect: row.try_get("effect").ok(),
            probability: row.try_get("probability").ok(),
        });
    } else {
        return None;
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