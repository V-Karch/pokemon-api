use crate::model::{self, WebResult};
use warp::{http::StatusCode, reply::json};
use sqlx::Row;

pub async fn search_location_by_id(
    location_id: i32,
    pool: sqlx::SqlitePool,
) -> WebResult<impl warp::Reply> {
    match sqlx::query("SELECT * FROM locations WHERE id = ?")
        .bind(location_id)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => Ok(warp::reply::with_status(
            json(&model::Location {
                id: row.get("id"),
                name: row.get("name"),
                generation: row.get("generation"),
            }),
            StatusCode::OK,
        )),
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: format!("Location ID `{}` not found", location_id),
            }),
            StatusCode::NOT_FOUND,
        )),
    }
}