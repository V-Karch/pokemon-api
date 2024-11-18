mod utils;
mod model;
mod handler;

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
        .and_then(handler::version_info);

    let search_move_by_id_route = warp::path!("api" / "move" / i32)
        .and(warp::get())
        .and(pool_filter.clone()) // Inject the pool into the route
        .and_then(handler::search_move_by_id);

    let search_move_by_name_route = warp::path!("api" / "move" / String)
        .and(warp::get())
        .and(pool_filter.clone()) // Inject the pool into the route
        .and_then(handler::search_move_by_name);

    let search_ability_by_id_route = warp::path!("api" / "ability" / i32)
        .and(warp::get())
        .and(pool_filter.clone()) // Inject the pool into the route
        .and_then(handler::search_ability_by_id);

    let search_ability_by_name_route = warp::path!("api" / "ability" / String)
        .and(warp::get())
        .and(pool_filter.clone()) // Inject the pool into the route
        .and_then(handler::search_ability_by_name);

    // Combine all the routes into one
    let routes = version_info_route
        .or(search_move_by_id_route)
        .or(search_move_by_name_route)
        .or(search_ability_by_id_route)
        .or(search_ability_by_name_route)
        .with(warp::log("api"));

    println!("🚀 Server started successfully at http://0.0.0.0:8000");

    // Run the server
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
