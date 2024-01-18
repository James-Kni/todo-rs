use std::env;

use axum::http::StatusCode;
use dotenvy::dotenv;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use tracing::info;

use crate::Todo;

pub async fn setup_db_pool() -> Pool<Sqlite> {
    dotenv().unwrap();

    let database_url = env::var("DATABASE_URL").unwrap_or("sqlite://sqlite.db".to_string());

    if !Sqlite::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        info!("Database does not exist. Attempting to create...");
        match Sqlite::create_database(&database_url).await {
            Ok(_) => info!("Database created at {}", database_url),
            Err(err) => panic!("Database could not be created. {}", err),
        }
    } else {
        info!("Database exists ✓")
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("An error occurred setting up database ✗");
    info!("Pool has been setup ✓");

    match sqlx::migrate!().run(&pool).await {
        Ok(_) => info!("Migrations ran successfully ✓"),
        Err(err) => info!("Migrations failed to run: {}", err),
    }

    pool
}

pub async fn get_todo_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<Todo, StatusCode> {
    match sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = ?", id)
        .fetch_one(pool)
        .await
    {
        Ok(todo) => Ok(todo),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
