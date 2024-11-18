use crate::model;
use serde::Serialize;
use sqlx::Row;
use warp::{reply::json, Rejection, Reply};

pub type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Serialize)]
pub struct VersionInfoResponse {
    pub status: String,
    pub version: String,
}

pub async fn version_info() -> WebResult<impl Reply> {
    let response_json = &VersionInfoResponse {
        status: "Success".to_string(),
        version: "pokemon-api v0.0.1".to_string(),
    };
    return Ok(json(response_json));
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

pub async fn search_move_by_id_temp(move_id: i32) -> WebResult<impl Reply> {
    let response_json = &VersionInfoResponse {
        status: "Success".to_string(),
        version: format!("Input: {}", move_id),
    };
    return Ok(json(response_json));
}

pub async fn search_ability_by_id(pool: &sqlx::SqlitePool, move_id: i32) -> Option<model::Ability> {
    if let Ok(row) = sqlx::query("SELECT * FROM abilities WHERE id = ?")
        .bind(move_id)
        .fetch_one(pool)
        .await
    {
        return Some(model::Ability {
            id: row.get("id"),
            name: row.get("name"),
            pokemon_count: row.get("pokemon"),
            effect: row.get("effect"),
            generation: row.get("generation"),
        });
    } else {
        return None;
    }
}

pub async fn search_ability_by_name(pool: &sqlx::SqlitePool, move_name: &str) -> Option<model::Ability> {
    if let Ok(row) = sqlx::query("SELECT * FROM abilities WHERE name = ?")
        .bind(move_name)
        .fetch_one(pool)
        .await
    {
        return Some(model::Ability {
            id: row.get("id"),
            name: row.get("name"),
            pokemon_count: row.get("pokemon"),
            effect: row.get("effect"),
            generation: row.get("generation"),
        });
    } else {
        return None;
    }
}