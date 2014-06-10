use collections::enum_set::EnumSet;

use pcre::Pcre;
use pcre::{CompileOption,Caseless};

enum Method {
    Delete,
    Get,
    Patch,
    Post,
    Put
}

struct Route<T> {
    method: Method,
    regex: Pcre,
    pattern: String,
    handler: T
}

impl<T:Clone> Clone for Route<T> {
    fn clone(&self) -> Route<T> {
        let mut compile_options = EnumSet::<CompileOption>::empty();
        compile_options.add(Caseless);
        Route {
            method: self.method,
            regex: Pcre::compile_with_options(self.pattern.as_slice(), &compile_options).unwrap(),
            pattern: self.pattern.clone(),
            handler: self.handler.clone()
        }
    }
}

#[deriving(Clone)]
pub struct Router<T> {
    routes: Vec<Route<T>>
}

impl<T:Clone> Router<T> {
    pub fn new () -> Router<T> {
        Router {
            routes: vec![]
        }
    }

    pub fn add_route(&mut self, pattern: &str, handler: T) {
        let mut compile_options = EnumSet::<CompileOption>::empty();
        compile_options.add(Caseless);
        match Pcre::compile_with_options(pattern, &compile_options) {
            Ok(r) => {
                self.routes.push(Route {
                    method: Get, 
                    regex: r, 
                    pattern: String::from_str(pattern),
                    handler: handler
                });
            }
            Err(s) => {
                fail!("Error compiling route regex: {}", s.message());
            }
        }
    }

    pub fn find_route<'a> (&'a self, url: String) -> Option<&'a T> {
        for route in self.routes.iter() {
            let h = &route.handler;
            match route.regex.exec(url.as_slice()) {
                Some(_) => {
                    return Some(h)
                }
                None => {}
            }
        }
        None
    }
}

#[cfg(test)]
fn route1() {
}

#[test]
fn add_route() {
    let mut router = Router::<extern fn()>::new();

    router.add_route("^/", route1);

    let r = router.find_route("/");
    assert!(r.is_some());
}

#[test]
fn dont_match_routes() {
    let mut router = Router::<extern fn()>::new();

    router.add_route("^/$", route1);

    let r = router.find_route("/missing");
    assert!(r.is_none());
}
