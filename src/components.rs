use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::{
    icons::{cancel_icon, confirm_icon, delete_icon, edit_icon},
    Todo,
};

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
                meta name="viewport" content="width=device-width, initial-scale=1";
                link href="/assets/tailwind.css" rel="stylesheet";
                link rel="icon" type="image/ico" href="/assets/favicon.ico";
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
        "hx-on::after-request"="if(event.detail.successful) this.reset()"
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
        div #{"todo-"(todo.id)} ."flex justify-between" {
            div ."flex gap-6" {
                input
                #{"todo-"(todo.id)"-checkbox"}
                ."checkbox"
                type="checkbox"
                autocomplete="off"
                checked[todo.complete]
                hx-put={"/api/todo/"(todo.id)}
                hx-vals={"js:{complete: document.getElementById('todo-"(todo.id)"-checkbox').checked}"}
                hx-target={"#todo-"(todo.id)}
                hx-swap="outerHTML";

                @if todo.complete {
                    s ."text-gray-400" {( todo.title )}
                } @else {
                    p {( todo.title )}
                }
            }

            button
            hx-target={"#todo-"(todo.id)}
            hx-get={"/api/todo/"(todo.id)"/edit"}
            hx-swap="outerHTML" {
                (edit_icon())
            }
        }
    }
}

pub fn todo_item_edit(todo: Todo) -> Markup {
    html! {
        form
        #{"todo-"(todo.id)"-edit"}
        ."flex justify-between"
        hx-put={"/api/todo/"(todo.id)}
        hx-swap="outerHTML" {
            div ."flex gap-6" {
                button
                hx-delete={"/api/todo/"(todo.id)}
                hx-target={"#todo-"(todo.id)}
                hx-swap="outerHtml" {
                    (delete_icon())
                }

                input ."input input-sm w-auto" type="text" name="title" value={(todo.title)};
            }

            div ."flex gap-4" {
                button type="submit" {( confirm_icon() )}

                button
                hx-target={"#todo-"(todo.id)"-edit"}
                hx-get={"/api/todo/"(todo.id)}
                hx-swap="outerHTML" {
                    (cancel_icon())
                }
            }
        }
    }
}
