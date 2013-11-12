extern mod http;
extern mod pcre;

//use http::server::{Request, ResponseWriter};

use pcre::Pcre;
use pcre::{PCRE_CASELESS};

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
    handler: extern fn ()
}

struct Router {
    routes: ~[Route]
}

impl Router {
    fn new () -> Router {
        Router {
            routes: ~[]
        }
    }

    pub fn add_route(&mut self, pattern: &str, handler: fn()) {
        match Pcre::compile_with_options(pattern, PCRE_CASELESS) {
            Ok(r) => {
                self.routes.push(Route {method: Get, regex: r, handler: handler});
            }
            Err(s) => {
                fail!("Error compiling route regex: {}", s.message());
            }
        }
    }

    fn find_route (&self, url: &str) -> Option<extern fn()> {
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
