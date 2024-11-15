use sqlx::{sqlite::{SqlitePool, SqliteQueryResult}, Pool};

pub async fn connect() -> Result<Pool<sqlx::Sqlite>, sqlx::Error> {
    let pool: Pool<sqlx::Sqlite> = SqlitePool::connect("sqlite:pokemon.db").await?;
    return Ok(pool);
}

pub async fn create_moves_table(pool: &Pool<sqlx::Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
    let query: SqliteQueryResult = sqlx::query(
        "CREATE TABLE moves (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(255) NOT NULL,
            type VARCHAR(255) NOT NULL,
            category VARCHAR(255) NOT NULL,
            power INTEGER,
            accuracy INTEGER,
            PP INTEGER,
            effect TEXT,
            probability INTEGER
        );
        "
    )
    .execute(pool)
    .await?;

    return Ok(query);
}

pub async fn fill_moves_table(pool: &Pool<sqlx::Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
    let query = sqlx::query("")
        .execute(pool)
        .await?;

    return Ok(query);
}