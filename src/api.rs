use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Form, Router,
};
use maud::{html, Markup};
use serde::Deserialize;
use sqlx::QueryBuilder;

use crate::{
    components::{todo_item, todo_item_edit},
    db::get_todo_by_id,
    AppState, Todo,
};

pub fn create_api_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/todo", post(add_todo).delete(delete_all_todos))
        .route("/todo/:id/edit", get(get_todo_edit))
        .route(
            "/todo/:id",
            get(get_todo).delete(delete_todo).put(update_todo),
        )
}

#[derive(Deserialize)]
struct CreateTodoForm {
    title: String,
}

async fn add_todo(
    State(state): State<Arc<AppState>>,
    Form(todo): Form<CreateTodoForm>,
) -> Result<Markup, StatusCode> {
    // Don't allow empty todos
    if todo.title.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let result = sqlx::query_as!(Task, "INSERT INTO todos (title) VALUES (?)", todo.title)
        .execute(&state.pool)
        .await
        .unwrap();

    Ok(todo_item(Todo {
        id: result.last_insert_rowid(),
        title: todo.title.clone(),
        complete: false,
        order: 0,
    }))
}

async fn delete_all_todos(State(state): State<Arc<AppState>>) -> Result<Markup, StatusCode> {
    match sqlx::query!("DELETE FROM todos").execute(&state.pool).await {
        Ok(_) => Ok(html! {}),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_todo_edit(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Markup, StatusCode> {
    match get_todo_by_id(&state.pool, id).await {
        Ok(todo) => Ok(todo_item_edit(todo)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Markup, StatusCode> {
    match get_todo_by_id(&state.pool, id).await {
        Ok(todo) => Ok(todo_item(todo)),
        Err(err) => Err(err),
    }
}

async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Markup, StatusCode> {
    match sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&state.pool)
        .await
    {
        Ok(_) => Ok(html! {}),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Deserialize)]
struct UpdateTodoForm {
    title: Option<String>,
    complete: Option<bool>,
}

async fn update_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Form(todo): Form<UpdateTodoForm>,
) -> Result<Markup, StatusCode> {
    let current_data = get_todo_by_id(&state.pool, id).await?;

    let mut query_builder = QueryBuilder::new("UPDATE todos SET");

    if let Some(title) = &todo.title {
        query_builder.push(" title = ");
        query_builder.push_bind(title);
    }

    if let Some(complete) = &todo.complete {
        query_builder.push(" complete = ");
        query_builder.push_bind(complete);
    }

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(id);

    let query = query_builder.build();

    match query.execute(&state.pool).await {
        Ok(_) => Ok(todo_item({
            Todo {
                id,
                title: todo.title.unwrap_or(current_data.title),
                complete: todo.complete.unwrap_or(current_data.complete),
                order: current_data.order,
            }
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
