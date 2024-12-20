use crate::model;

use sqlx::Row;
use warp::{http::StatusCode, reply::json};

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

pub async fn list_all_moves(pool: sqlx::SqlitePool) -> model::WebResult<impl warp::Reply> {
    return match sqlx::query("SELECT name FROM moves").fetch_all(&pool).await {
        Ok(rows) => {
            // Collect all move names into a Vec<String>
            let move_names: Vec<String> = rows.iter().map(|row| row.get("name")).collect();
            Ok(warp::reply::with_status(json(&move_names), StatusCode::OK))
        }
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: "Failed to retrieve the list of moves.".to_string(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    };
}
