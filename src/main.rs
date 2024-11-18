mod utils;
mod model;

#[tokio::main]
async fn main() {
    // Connect to the SQLite database
    let pool = utils::connect()
        .await
        .expect("Failed to connect to database");

    let ability_as_json = utils::ability_to_json(
        &utils::search_ability_by_id(
            &pool, 12
        ).await
    ).await;

    println!("{}", &ability_as_json);
}
