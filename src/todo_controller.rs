use http::server::{Request, ResponseWriter};
use http::headers::content_type::MediaType;

use views::Action;

use super::models::Todo;
use super::views::todo;


pub struct TodoController;

impl TodoController {
    pub fn new() -> TodoController {
        TodoController
    }

    pub fn index(_request: &Request, response: &mut ResponseWriter) -> Box<Action + 'static> {
        let todo_list = vec!(
            Todo::new("Finish this wonderful framework!"),
            Todo::new("Make it more generic"),
            Todo::new("Learn rust"),
            Todo::new("Make <b> this & publish it"));

        response.headers.content_type = Some(MediaType{
            type_: String::from_str("text"),
            subtype: String::from_str("html"),
            parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
        });

        box todo::TodoIndexView::new(todo_list) as Box<Action>
    }

    pub fn details(_request: &Request, response: &mut ResponseWriter) -> Box<Action + 'static> {
        response.headers.content_type = Some(MediaType {
            type_: String::from_str("text"),
            subtype: String::from_str("html"),
            parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
        });

        let model = box Todo::new("Test");

        box todo::TodoDetailView::new(model) as Box<Action>
    }
    
    pub fn fail(_request: &Request, _response: &mut ResponseWriter) -> Box<Action + 'static> {
        panic!("Failing on purpose here!");
    }
}


