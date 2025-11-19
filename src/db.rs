use sqlx::{sqlite::SqlitePoolOptions, sqlite::SqliteConnectOptions, Pool, Sqlite};
use std::env;
use std::str::FromStr;

pub type DbPool = Pool<Sqlite>;

pub async fn init_db() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection_options = SqliteConnectOptions::from_str(&database_url)
        .expect("Invalid DATABASE_URL")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
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