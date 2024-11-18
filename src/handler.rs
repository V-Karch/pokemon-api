use crate::model::{self, WebResult};

use sqlx::Row;
use warp::reply::json;

pub async fn version_info() -> model::WebResult<impl warp::Reply> {
    let response_json = &model::VersionInfoResponse {
        status: "Success".to_string(),
        version: "pokemon-api v0.0.1".to_string(),
    };
    return Ok(json(response_json));
}

pub async fn search_move_by_name(
    move_name: String,
    pool: sqlx::SqlitePool,
) -> model::WebResult<impl warp::Reply> {
    if let Ok(row) = &sqlx::query("SELECT * FROM moves WHERE name = ?")
        .bind(move_name.replace(" ", "-"))
        .fetch_one(&pool)
        .await
    {
        return Ok(json(&model::Move {
            id: row.get("id"),
            name: row.get("name"),
            move_type: row.get("type"),
            category: row.get("category"),
            power: row.try_get("power").ok(),
            accuracy: row.try_get("accuracy").ok(),
            pp: row.try_get("PP").ok(),
            effect: row.try_get("effect").ok(),
            probability: row.try_get("probability").ok(),
        }));
    } else {
        return Ok(json(&model::GenericFailure {
            status: "Error".to_string(),
            message: "Failed to retrieve mode".to_string(),
        }));
    }
}

pub async fn search_move_by_id(
    move_id: i32,
    pool: sqlx::SqlitePool,
) -> model::WebResult<impl warp::Reply> {
    if let Ok(row) = sqlx::query("SELECT * FROM moves WHERE id = ?")
        .bind(move_id)
        .fetch_one(&pool)
        .await
    {
        return Ok(json(&model::Move {
            id: row.get("id"),
            name: row.get("name"),
            move_type: row.get("type"),
            category: row.get("category"),
            power: row.try_get("power").ok(),
            accuracy: row.try_get("accuracy").ok(),
            pp: row.try_get("PP").ok(),
            effect: row.try_get("effect").ok(),
            probability: row.try_get("probability").ok(),
        }));
    } else {
        return Ok(json(&model::GenericFailure {
            status: "Error".to_string(),
            message: "Failed to retrieve move".to_string(),
        }));
    }
}

pub async fn search_ability_by_id(move_id: i32, pool: sqlx::SqlitePool) -> model::WebResult<impl warp::Reply> {
    if let Ok(row) = &sqlx::query("SELECT * FROM abilities WHERE id = ?")
        .bind(move_id)
        .fetch_one(&pool)
        .await
    {
        return Ok(json(&model::Ability {
            id: row.get("id"),
            name: row.get("name"),
            pokemon_count: row.get("pokemon"),
            effect: row.get("effect"),
            generation: row.get("generation"),
        }));
    } else {
        return Ok(json(&model::GenericFailure {
            status: "Error".to_string(),
            message: "Failed to retrieve ability".to_string(),
        }));
    }
}

pub async fn search_ability_by_name(
    move_name: String, pool: sqlx::SqlitePool
) -> WebResult<impl warp::Reply> {
    if let Ok(row) = sqlx::query("SELECT * FROM abilities WHERE name = ?")
        .bind(move_name)
        .fetch_one(&pool)
        .await
    {
        return Ok(json(&model::Ability {
            id: row.get("id"),
            name: row.get("name"),
            pokemon_count: row.get("pokemon"),
            effect: row.get("effect"),
            generation: row.get("generation"),
        }));
    } else {
        return Ok(json(&model::GenericFailure {
            status: "Error".to_string(),
            message: "Failed to retrieve ability".to_string(),
        }));
    }
}
