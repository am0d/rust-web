extern mod extra;
extern mod http;

extern mod utils;
extern mod models;
extern mod views;

use std::vec;

use std::rt::io::Writer;

use http::server::{Request, ResponseWriter};

use utils::{get_url, not_found};

use models::Todo;
use views::View;

pub struct TodoController;

impl TodoController {
    pub fn new() -> TodoController {
        TodoController
    }

    pub fn dispatch_request(&self, request: &Request, response: &mut ResponseWriter) {
        match get_url(request) {
            ~"/todos" | ~"/todos/" => {
                self.Index(request, response);
            },
            _ => {
                not_found(request, response);
            }
        }
    }

    pub fn Index(&self, request: &Request, response: &mut ResponseWriter) {
        let todo_list: ~[Todo] = vec::build(None, |push| {
            push(Todo::new(~"Finish this wonderful framework!"));
            push(Todo::new(~"Make it more generic"));
            push(Todo::new(~"Learn rust"));
            push(Todo::new(~"Make <> this & publish it"));
        });

        views::IndexView(todo_list).render(|s| {
            response.write(s.as_safe_string().into_bytes());
        });
    }
}


