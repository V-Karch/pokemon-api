use crate::model::{self, WebResult};

use sqlx::Row;
use warp::{http::StatusCode, reply::json};

pub async fn search_item_by_id(
    item_id: i32,
    pool: sqlx::SqlitePool,
) -> WebResult<impl warp::Reply> {
    match sqlx::query("SELECT * FROM items WHERE id = ?")
        .bind(item_id)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => Ok(warp::reply::with_status(
            json(&model::Item {
                id: row.get("id"),
                name: row.get("name"),
                category: row.try_get("category").ok(),
                effect: row.try_get("effect").ok(),
            }),
            StatusCode::OK,
        )),
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: format!("Item ID `{}` not found", item_id),
            }),
            StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn search_item_by_name(
    item_name: String,
    pool: sqlx::SqlitePool,
) -> WebResult<impl warp::Reply> {
    match sqlx::query("SELECT * FROM items WHERE name = ?")
        .bind(&item_name)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => Ok(warp::reply::with_status(
            json(&model::Item {
                id: row.get("id"),
                name: row.get("name"),
                category: row.try_get("category").ok(),
                effect: row.try_get("effect").ok(),
            }),
            StatusCode::OK,
        )),
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: format!("Item `{}` not found", &item_name),
            }),
            StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn list_all_items(pool: sqlx::SqlitePool) -> model::WebResult<impl warp::Reply> {
    return match sqlx::query("SELECT name FROM items").fetch_all(&pool).await {
        Ok(rows) => {
            // Collect all item names into a Vec<String>
            let item_names: Vec<String> = rows.iter().map(|row| row.get("name")).collect();
            Ok(warp::reply::with_status(json(&item_names), StatusCode::OK))
        }
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: "Failed to retrieve the list of items.".to_string(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    };
}

pub async fn list_items_by_category(
    category_name: String,
    pool: sqlx::SqlitePool,
) -> WebResult<impl warp::Reply> {
    match sqlx::query("SELECT * FROM items WHERE category = ?")
        .bind(&category_name)
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => {
            let items: Vec<model::Item> = rows
                .into_iter()
                .map(|row| model::Item {
                    id: row.get("id"),
                    name: row.get("name"),
                    category: row.try_get("category").ok(),
                    effect: row.try_get("effect").ok(),
                })
                .collect();

            Ok(warp::reply::with_status(json(&items), StatusCode::OK))
        }
        Err(_) => Ok(warp::reply::with_status(
            json(&model::GenericFailure {
                status: "Error".to_string(),
                message: format!("No items found in category `{}`", &category_name),
            }),
            StatusCode::NOT_FOUND,
        )),
    }
}
