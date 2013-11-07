extern mod http;
extern mod pcre;
extern mod todo_controller;

use http::server::{Request, ResponseWriter};

use pcre::Pcre;
use pcre::{PCRE_CASELESS};

use todo_controller::TodoController;

fn add_route(pattern: &str, handler: fn()) {
}

fn find_route (url: &str) -> Option<fn(&Request, &mut ResponseWriter)> {
    let r = Pcre::compile_with_options("^/todos/?", PCRE_CASELESS);
    match r.exec(url) {
        Some(_) => {
            return Some(todo_controller::TodoController::Index)
        }
        None => {}
    }
    None
}
