use crate::model::{self, WebResult};

use sqlx::Row;
use warp::{http::StatusCode, reply::json};

pub async fn search_ability_by_id(
    ability_id: i32,
    pool: sqlx::SqlitePool,
) -> WebResult<impl warp::Reply> {
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

pub async fn list_all_abilities(pool: sqlx::SqlitePool) -> WebResult<impl warp::Reply> {
    return match sqlx::query("SELECT name FROM abilities")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => {
            // Collect all ability names into a Vec<String>
            let ability_names: Vec<String> = rows.iter().map(|row| row.get("name")).collect();
            Ok(warp::reply::with_status(
                json(&ability_names),
                StatusCode::OK,
            ))
        }
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: "Failed to retrieve the list of abilities.".to_string(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    };
}
