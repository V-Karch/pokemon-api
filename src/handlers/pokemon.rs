use crate::model::{self, WebResult};

use sqlx::Row;
use warp::{http::StatusCode, reply::json};

pub async fn list_all_pokemon_names(pool: sqlx::SqlitePool) -> WebResult<impl warp::Reply> {
    return match sqlx::query("SELECT name FROM pokemon").fetch_all(&pool).await {
        Ok(rows) => {
            let pokemon_names: Vec<String> = rows.iter().map(|row| row.get("name")).collect();
            Ok(warp::reply::with_status(json(&pokemon_names), StatusCode::OK))
        }
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: "Failed to retrieve the list of pokemon names.".to_string(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    };
}
