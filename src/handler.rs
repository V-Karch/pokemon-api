use crate::model::{self, WebResult};

use sqlx::Row;
use warp::{http::StatusCode, reply::json};

pub async fn version_info() -> model::WebResult<impl warp::Reply> {
    let response_json = &model::VersionInfoResponse {
        status: "Success".to_string(),
        version: "pokemon-api v0.0.1".to_string(),
    };
    Ok(json(response_json))
}

pub async fn search_move_by_name(
    move_name: String,
    pool: sqlx::SqlitePool,
) -> model::WebResult<impl warp::Reply> {
    match sqlx::query("SELECT * FROM moves WHERE name = ?")
        .bind(move_name.replace(" ", "-"))
        .fetch_one(&pool)
        .await
    {
        Ok(row) => Ok(warp::reply::with_status(
            json(&model::Move {
                id: row.get("id"),
                name: row.get("name"),
                move_type: row.get("type"),
                category: row.get("category"),
                power: row.try_get("power").ok(),
                accuracy: row.try_get("accuracy").ok(),
                pp: row.try_get("PP").ok(),
                effect: row.try_get("effect").ok(),
                probability: row.try_get("probability").ok(),
            }),
            StatusCode::OK,
        )),
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: format!("Move `{}` not found", move_name),
            }),
            StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn search_move_by_id(
    move_id: i32,
    pool: sqlx::SqlitePool,
) -> model::WebResult<impl warp::Reply> {
    match sqlx::query("SELECT * FROM moves WHERE id = ?")
        .bind(move_id)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => Ok(warp::reply::with_status(
            json(&model::Move {
                id: row.get("id"),
                name: row.get("name"),
                move_type: row.get("type"),
                category: row.get("category"),
                power: row.try_get("power").ok(),
                accuracy: row.try_get("accuracy").ok(),
                pp: row.try_get("PP").ok(),
                effect: row.try_get("effect").ok(),
                probability: row.try_get("probability").ok(),
            }),
            StatusCode::OK,
        )),
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: format!("Move ID `{}` not found", move_id),
            }),
            StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn search_ability_by_id(
    ability_id: i32,
    pool: sqlx::SqlitePool,
) -> model::WebResult<impl warp::Reply> {
    match sqlx::query("SELECT * FROM abilities WHERE id = ?")
        .bind(ability_id)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => Ok(warp::reply::with_status(
            json(&model::Ability {
                id: row.get("id"),
                name: row.get("name"),
                pokemon_count: row.get("pokemon"),
                effect: row.get("effect"),
                generation: row.get("generation"),
            }),
            StatusCode::OK,
        )),
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: format!("Ability ID `{}` not found", ability_id),
            }),
            StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn search_ability_by_name(
    ability_name: String,
    pool: sqlx::SqlitePool,
) -> WebResult<impl warp::Reply> {
    match sqlx::query("SELECT * FROM abilities WHERE name = ?")
        .bind(&ability_name)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => Ok(warp::reply::with_status(
            json(&model::Ability {
                id: row.get("id"),
                name: row.get("name"),
                pokemon_count: row.get("pokemon"),
                effect: row.get("effect"),
                generation: row.get("generation"),
            }),
            StatusCode::OK,
        )),
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: format!("Ability `{}` not found", ability_name),
            }),
            StatusCode::NOT_FOUND,
        )),
    }
}
