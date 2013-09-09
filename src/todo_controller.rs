extern mod extra;
extern mod http;

extern mod utils;

//use std::os;
//use std::path::{Path, GenericPath};

use std::io;
//use std::io::ReaderUtil;

use std::rt::io::Writer;
//use extra::time;

use http::server::{Request, ResponseWriter};
use http::server::request::{AbsoluteUri, AbsolutePath};
//use http::method::{Get};

use utils::{get_url, not_found};

struct TodoController;

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
        response.write(bytes!("This is the list of todos"));
    }
}
