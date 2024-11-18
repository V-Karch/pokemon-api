mod utils;
mod model;

#[tokio::main]
async fn main() {
    // Connect to the SQLite database
    let pool = utils::connect()
        .await
        .expect("Failed to connect to database");

    let move_as_json = utils::move_to_json(
        &utils::search_move_by_name(
            &pool, "waterfall"
        )
        .await
    ).await;

    let another_move_as_json = utils::move_to_json(
        &utils::search_move_by_id(
            &pool, 12
        )
        .await
    ).await;

    println!("{}", &move_as_json);

    println!("{}", &another_move_as_json);
}
