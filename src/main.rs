mod utils;
mod model;

#[tokio::main]
async fn main() {
    // Connect to the SQLite database
    let pool = utils::connect()
        .await
        .expect("Failed to connect to database");

    let test_string = "waterfall";

    let test_move: model::Move = utils::search_move_by_name(&pool, test_string)
    .await
    .expect(&format!("[Move `{}`] could not be found in the database", test_string));

    println!("{}", test_move.to_string());
}
