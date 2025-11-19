use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;

pub type DbPool = Pool<Sqlite>;

pub async fn init_db() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Automatically run migrations on startup
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    println!("->> DATABASE MIGRATIONS ran successfully");

    pool
}