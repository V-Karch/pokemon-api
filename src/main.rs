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

    let ability_as_json = utils::ability_to_json(
        &handler::search_ability_by_id(
            &pool, 12
        ).await
    ).await;

    let move_as_json = utils::move_to_json(
        &handler::search_move_by_id(
            &pool, 700
        ).await
    ).await;

    println!("{}", &ability_as_json);
    println!("{}", &move_as_json);

    // API setup
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    // Define individual routes
    let version_info_route = warp::path!("api")
        .and(warp::get())
        .and_then(handler::version_info);

    let search_move_by_id = warp::path!("api" / "move" / i32)
        .and(warp::get())
        .and_then(handler::search_move_by_id_temp);

    // Combine all the routes into one
    let routes = version_info_route
        .or(search_move_by_id)
        .with(warp::log("api"));

    println!("🚀 Server started successfully");

    // Run the server
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
