use maud::{html, Markup};

use crate::{
    components::{navbar, page_base, todo_form, todo_list, PageBaseConfig},
    Todo,
};

pub fn home_page() -> Markup {
    let test_todos: Vec<Todo> = vec![Todo {
        title: "Test todo".to_string(),
        order: 0,
        id: 0,
        complete: false,
    }];

    page_base(PageBaseConfig {
        title: "Home",
        navigation: Some(navbar()),
        content: html! {
            div ."container mx-auto py-8 px-4 flex flex-col gap-4 items-start" {
                div ."flex flex-col items-center gap-8 w-full max-w-screen-md mx-auto" {
                    (todo_form())
                    (todo_list(test_todos))
                }
            }
        },
    })
}
