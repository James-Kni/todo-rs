use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use maud::{html, Markup};

use crate::AppState;

pub fn create_api_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/todo", post(add_todo).delete(delete_all_todos))
        .route("/todo/:id/edit", get(get_todo_edit))
        .route(
            "/todo/:id",
            get(get_todo).delete(delete_todo).put(update_todo),
        )
}

async fn add_todo() -> Markup {
    html! {}
}

async fn delete_all_todos() -> Markup {
    html! {}
}

async fn get_todo_edit() -> Markup {
    html! {}
}

async fn get_todo() -> Markup {
    html! {}
}

async fn delete_todo() -> Markup {
    html! {}
}

async fn update_todo() -> Markup {
    html! {}
}
