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

    println!("{}", &ability_as_json);

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    let version_info_route = warp::path!("api")
        .and(warp::get())
        .and_then(handler::version_info);

    let routes = version_info_route.with(warp::log("api"));

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}