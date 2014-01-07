extern mod http;
extern mod pcre;

use extra::enum_set::EnumSet;

use http::server::{Request, ResponseWriter};

use pcre::Pcre;
use pcre::{CompileOption,Caseless};

use views::Action;

enum Method {
    Delete,
    Get,
    Patch,
    Post,
    Put
}

struct Route {
    method: Method,
    regex: pcre::Pcre,
    pattern: ~str,
    handler: extern fn (&Request, &mut ResponseWriter) -> ~Action
}

impl Clone for Route {
    fn clone(&self) -> Route {
        let mut compile_options = EnumSet::<CompileOption>::empty();
        compile_options.add(Caseless);
        Route {
            method: self.method,
            regex: Pcre::compile_with_options(self.pattern, &compile_options).unwrap(),
            pattern: self.pattern.clone(),
            handler: self.handler
        }
    }
}

#[deriving(Clone)]
pub struct Router {
    routes: ~[Route]
}

impl Router {
    pub fn new () -> Router {
        Router {
            routes: ~[]
        }
    }

    pub fn add_route(&mut self, pattern: &str, handler: fn(&Request, &mut ResponseWriter) -> ~Action) {
        let mut compile_options = EnumSet::<CompileOption>::empty();
        compile_options.add(Caseless);
        match Pcre::compile_with_options(pattern, &compile_options) {
            Ok(r) => {
                self.routes.push(Route {
                    method: Get, 
                    regex: r, 
                    pattern: pattern.to_owned(),
                    handler: handler});
            }
            Err(s) => {
                fail!("Error compiling route regex: {}", s.message());
            }
        }
    }

    pub fn find_route (&self, url: &str) -> Option<extern fn(&Request, &mut ResponseWriter) -> ~Action> {
        for route in self.routes.iter() {
            let h = route.handler;
            match route.regex.exec(url) {
                Some(_) => {
                    return Some(h)
                }
                None => {}
            }
        }
        None
    }
}

#[test]
fn route1() {
}

#[test]
fn add_route() {
    let mut router = Router::new();

    router.add_route("^/", route1);

    let r = router.find_route("/");
    assert!(r.is_some());
}
