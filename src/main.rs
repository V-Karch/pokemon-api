mod handlers;
mod model;
mod utils;

use handlers::pokemon::search_pokemon_by_name;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Connect to the SQLite database
    let pool = utils::connect()
        .await
        .expect("Failed to connect to database");

    // API setup
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    // Cloneable pool for route injection
    let pool_filter = warp::any().map(move || pool.clone());

    // Define individual routes
    let version_info_route = warp::path!("api")
        .and(warp::get())
        .and_then(handlers::version::version_info);

    let search_move_by_id_route = warp::path!("api" / "move" / i32)
        .and(warp::get())
        .and(pool_filter.clone()) // Inject the pool into the route
        .and_then(handlers::moves::search_move_by_id);

    let search_move_by_name_route = warp::path!("api" / "move" / String)
        .and(warp::get())
        .and(pool_filter.clone()) // Inject the pool into the route
        .and_then(handlers::moves::search_move_by_name);

    let list_all_moves_route = warp::path!("api" / "move")
        .and(warp::get())
        .and(pool_filter.clone()) // Inject the pool into the route
        .and_then(handlers::moves::list_all_moves);

    let search_ability_by_id_route = warp::path!("api" / "ability" / i32)
        .and(warp::get())
        .and(pool_filter.clone()) // Inject the pool into the route
        .and_then(handlers::abilities::search_ability_by_id);

    let search_ability_by_name_route = warp::path!("api" / "ability" / String)
        .and(warp::get())
        .and(pool_filter.clone()) // Inject the pool into the route
        .and_then(handlers::abilities::search_ability_by_name);

    let list_all_abilities_route = warp::path!("api" / "ability")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handlers::abilities::list_all_abilities);

    let list_all_types_route = warp::path!("api" / "type")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handlers::types::list_all_types);

    let search_type_by_id_route = warp::path!("api" / "type" / i32)
        .and(warp::get())
        .and(pool_filter.clone()) // Inject the pool into the route
        .and_then(handlers::types::search_type_by_id);

    let search_type_by_name_route = warp::path!("api" / "type" / String)
        .and(warp::get())
        .and(pool_filter.clone()) // Inject the pool into the route
        .and_then(handlers::types::search_type_by_name);

    let search_item_by_id_route = warp::path!("api" / "item" / i32)
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handlers::items::search_item_by_id);

    let search_item_by_name_route = warp::path!("api" / "item" / String)
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handlers::items::search_item_by_name);

    let list_all_items_route = warp::path!("api" / "item")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handlers::items::list_all_items);

    let search_items_by_category_route = warp::path!("api" / "item" / "category" / String)
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handlers::items::list_items_by_category);

    let list_all_pokemon_names_route = warp::path!("api" / "pokemon")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handlers::pokemon::list_all_pokemon_names);

    let list_unsupported_route = warp::path!("api" / "unsupported")
        .and(warp::get())
        .and_then(handlers::version::list_unsupported);

    let search_pokemon_by_id_route = warp::path!("api" / "pokemon" / i32)
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handlers::pokemon::search_pokemon_by_id);

    let search_pokemon_by_name_route = warp::path!("api" / "pokemon" / String)
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handlers::pokemon::search_pokemon_by_name);

    // Combine all the routes into one
    let routes = version_info_route
        .or(search_move_by_id_route) // Route for searching for moves by id
        .or(search_move_by_name_route) // Route for searching for moves by name
        .or(search_ability_by_id_route) // Route for searching for ability by id
        .or(search_ability_by_name_route) // Route for searching for ability by name
        .or(list_all_moves_route) // Route to list all move names
        .or(list_all_abilities_route) // Route to list all ability names
        .or(list_all_types_route) // Route to list all type names
        .or(search_type_by_id_route) // Route to search for a type by id
        .or(search_type_by_name_route) // Route to search for a type by name
        .or(search_item_by_id_route) // Route to search for an item by id
        .or(search_item_by_name_route) // Route to search for an item by name
        .or(list_all_items_route) // Route to list all items by name
        .or(search_items_by_category_route) // Route to list all items in a given category
        .or(list_all_pokemon_names_route) // Route to list all pokemon names
        .or(list_unsupported_route) // List to show currently unsupported pokemon
        .or(search_pokemon_by_id_route) // Search for a pokemon by it's id (uninmplemented)
        .or(search_pokemon_by_name_route) // Search for a pokemon by it's name (uninplemented)
        .with(warp::log("api"));

    println!("🚀 pokemon-api v0.0.1 started successfully at http://0.0.0.0:8000");

    // Run the server
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
