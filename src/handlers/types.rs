use crate::model::{self, WebResult};

use sqlx::Row;
use warp::{http::StatusCode, reply::json};

pub async fn list_all_types(pool: sqlx::SqlitePool) -> WebResult<impl warp::Reply> {
    return match sqlx::query("SELECT type_name FROM pokemon_types")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => {
            // Collect all ability names into a Vec<String>
            let ability_names: Vec<String> = rows.iter().map(|row| row.get("type_name")).collect();
            Ok(warp::reply::with_status(
                json(&ability_names),
                StatusCode::OK,
            ))
        }
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: "Failed to retrieve the list of types.".to_string(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    };
}

pub async fn search_type_by_id(
    type_id: i32,
    pool: sqlx::SqlitePool,
) -> WebResult<impl warp::Reply> {
    match sqlx::query("SELECT * FROM pokemon_types WHERE type_id = ?")
        .bind(type_id)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => Ok(warp::reply::with_status(
            json(&model::PokemonType {
                id: row.get("type_id"),
                name: row.get("type_name"),
            }),
            StatusCode::OK,
        )),
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: format!("Type ID `{}` not found", type_id),
            }),
            StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn search_type_by_name(
    type_name: String,
    pool: sqlx::SqlitePool,
) -> WebResult<impl warp::Reply> {
    match sqlx::query("SELECT * FROM pokemon_types WHERE type_name = ?")
        .bind(&type_name)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => Ok(warp::reply::with_status(
            json(&model::PokemonType {
                id: row.get("type_id"),
                name: row.get("type_name"),
            }),
            StatusCode::OK,
        )),
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: format!("Type`{}` not found", &type_name),
            }),
            StatusCode::NOT_FOUND,
        )),
    }
}
