use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::Todo;

pub struct PageBaseConfig<'a> {
    pub title: &'a str,
    pub navigation: Option<Markup>,
    pub content: Markup,
}
/// Base page layout
pub fn page_base(config: PageBaseConfig) -> Markup {
    html! {
        (DOCTYPE)
        html data-theme="cupcake" {
            head {
                title { (config.title) }
                meta charset="UTF-8";
                link href="/assets/tailwind.css" rel="stylesheet";
                (PreEscaped("<script src=\"/assets/htmx.min.js\"></script>"))
            }
            body {
                @if let Some(navigation) = config.navigation {
                    (navigation)
                }
                #content {
                    (config.content)
                }
            }
        }
    }
}

pub fn navbar() -> Markup {
    html! {
        div hx-boost="true" ."navbar bg-base-300" {
            a href="/" ."btn btn-ghost text-xl" {
                "Todos"
            }
        }
    }
}

pub fn todo_form() -> Markup {
    html! {
        form
        ."w-full flex flex-row gap-3"
        hx-post="/api/todo"
        hx-target="#todo-list"
        hx-swap="beforeend" {
            input
            #todo-input
            .input.input-bordered.w-full
            type="text"
            name="title"
            placeholder="Type here";

            button type="submit" .btn.btn-primary {
                "Add"
            }
        }
    }
}

pub fn todo_list(todos: Vec<Todo>) -> Markup {
    html! {
        div ."flex flex-col gap-f w-full" {
            div ."flex flex-row justify-between" {
                h2 ."text-2xl font-bold" {
                    "Your todos"
                }

                button
                type="button"
                ."btn btn-neutral"
                hx-delete="/api/todo"
                hx-target="#todo-list"
                hx-confirm="Are you sure you want to delete all todos" {
                    "Clear todos"
                }
            }
            div #todo-list ."py-6 flex flex-col gap-6" {
                @for todo in todos {
                    (todo_item(todo))
                }
            }
        }
    }
}

pub fn todo_item(todo: Todo) -> Markup {
    html! {
        p { "Todo title: " (todo.title)}
    }
}
