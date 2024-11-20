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

pub async fn list_all_abilities(pool: sqlx::SqlitePool) -> model::WebResult<impl warp::Reply> {
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

pub async fn list_all_types(pool: sqlx::SqlitePool) -> model::WebResult<impl warp::Reply> {
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
) -> model::WebResult<impl warp::Reply> {
    match sqlx::query("SELECT * FROM pokemon_types WHERE type_id = ?")
        .bind(type_id)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => Ok(warp::reply::with_status(
            json(&model::PokemonType {
                id: row.get("type_id"),
                name: row.get("type_name")
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
) -> model::WebResult<impl warp::Reply> {
    match sqlx::query("SELECT * FROM pokemon_types WHERE type_name = ?")
        .bind(&type_name)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => Ok(warp::reply::with_status(
            json(&model::PokemonType {
                id: row.get("type_id"),
                name: row.get("type_name")
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

pub async fn search_item_by_id(
    item_id: i32,
    pool: sqlx::SqlitePool,
) -> model::WebResult<impl warp::Reply> {
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
) -> model::WebResult<impl warp::Reply> {
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
    return match sqlx::query("SELECT name FROM items")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => {
            // Collect all item names into a Vec<String>
            let item_names: Vec<String> = rows.iter().map(|row| row.get("name")).collect();
            Ok(warp::reply::with_status(
                json(&item_names),
                StatusCode::OK,
            ))
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
) -> model::WebResult<impl warp::Reply> {
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
