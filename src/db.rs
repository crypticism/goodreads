use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::structs::AppConfig;

pub async fn init_db(config: &AppConfig) -> Pool<Postgres> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Unable to connect to database");

    sqlx::migrate!("db/migrations")
        .run(&pool)
        .await
        .expect("Unable to run migrations");

    pool
}
