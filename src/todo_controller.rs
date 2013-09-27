extern mod extra;
extern mod http;

extern mod utils;
extern mod views;

use std::vec;

use std::rt::io::Writer;
//use extra::time;

use http::server::{Request, ResponseWriter};
//use http::method::{Get};

use utils::{get_url, not_found};

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
        let mut todo_list: ~[~Todo] = vec::build(None, |push| {
            push(Todo::new(~"Finish this wonderful framework!"));
            push(Todo::new(~"Make it more generic"));
            push(Todo::new(~"Learn rust"));
        });

        response.write(views::IndexView().render().into_bytes());
    }
}

// Models
struct Todo {
    description: ~str,
    completed: bool
}

impl Todo {
    pub fn new(name: ~str) -> ~Todo {
        ~Todo {
            description: name,
            completed: false
        }
    }
}
