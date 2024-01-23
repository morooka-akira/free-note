use chrono::{DateTime, FixedOffset};
use sqlx::mysql::MySqlPool;
use std::env;

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: u64,
    created_at: DateTime<chrono::Utc>,
    name: String,
    icon_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    let row: User =
        sqlx::query_as("SELECT * from users JOIN user_details ON users.id = user_details.user_id WHERE users.id = ?")
            .bind(19)
            .fetch_one(&pool)
            .await?;

    print!("row: {:?}", &row);

    Ok(())
}
