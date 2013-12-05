extern mod extra;
extern mod http;

use std::vec;

use std::io::Writer;

use http::server::{Request, ResponseWriter};
use http::headers::content_type::MediaType;

use views::View;

use super::models::Todo;
use super::views::todo;


pub struct TodoController;

impl TodoController {
    pub fn new() -> TodoController {
        TodoController
    }

    pub fn Index(_request: &Request, response: &mut ResponseWriter) {
        let todo_list: ~[Todo] = vec::build(None, |push| {
            push(Todo::new(~"Finish this wonderful framework!"));
            push(Todo::new(~"Make it more generic"));
            push(Todo::new(~"Learn rust"));
            push(Todo::new(~"Make <b> this & publish it"));
        });

        response.headers.content_type = Some(MediaType{
            type_: ~"text",
            subtype: ~"html",
            parameters: ~[(~"charset", ~"UTF-8")]
        });
        todo::TodoIndexView::new(todo_list).render(|s| {
            response.write(s.to_str().into_bytes());
        });
    }

    pub fn Details(_request: &Request, response: &mut ResponseWriter) {
        response.headers.content_type = Some(MediaType {
            type_: ~"text",
            subtype: ~"html",
            parameters: ~[(~"charset", ~"UTF-8")]
        });

        let model = Todo::new(~"Test");
        todo::TodoDetailView::new(&model).render(|s| {
            response.write(s.to_str().into_bytes());
        });
    }
}


