use std::sync::Arc;

use api::create_api_router;
use axum::{extract::State, routing::get, Router};
use db::setup_db_pool;
use maud::Markup;
use pages::home_page;
use sqlx::{Pool, Sqlite};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod components;
mod db;
mod icons;
mod pages;

#[derive(Debug)]
struct Todo {
    id: i64,
    title: String,
    complete: bool,
    order: i64,
}

struct AppState {
    pool: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "todo_rs=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = setup_db_pool().await;
    let app_state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/", get(home_handler))
        .layer(LiveReloadLayer::new())
        .nest("/api", create_api_router())
        .with_state(app_state)
        .nest_service("/assets", ServeDir::new("assets"));

    info!("Route have been built");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn home_handler(State(state): State<Arc<AppState>>) -> Markup {
    match sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&state.pool)
        .await
    {
        Ok(todos) => home_page(todos),
        Err(_) => home_page(vec![]),
    }
}
