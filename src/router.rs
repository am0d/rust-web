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

trait ToResponse {
    fn to_response(&self) -> () {}
}

impl ToResponse for ~str {}
impl ToResponse for () {}

struct Route <T> {
    method: Method,
    regex: pcre::Pcre,
    handler: extern fn () -> T
}

struct Router<T> {
    routes: ~[Route<T>]
}

impl<T:ToResponse> Router<T> {
    fn new () -> Router<T> {
        Router {
            routes: ~[]
        }
    }

    pub fn add_route (&mut self, pattern: &str, handler: fn() -> T) {
        match Pcre::compile_with_options(pattern, PCRE_CASELESS) {
            Ok(r) => {
                self.routes.push(Route {method: Get, regex: r, handler: handler});
            }
            Err(s) => {
                fail!("Error compiling route regex: {}", s.message());
            }
        }
    }

    fn find_route (&self, url: &str) -> Option<extern fn() -> T> {
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

#[cfg(test)]
mod test {
    use router::Router;

    fn route1() -> ~str {
        return ~"success"
    }

    fn route2() {}

    #[test]
    fn add_route() {
        let mut router = Router::new();

        router.add_route("^/", route1);

        let r = router.find_route("/");
        assert!(r.is_some());
        let ret = (r.unwrap())();
        assert!(ret.as_slice() == "success");
    }

    #[test]
    fn add_multi_routes() {
        let mut router = Router::new();

        router.add_route("^/", route1);
        router.add_route("^/hello", route2);

        let r = router.find_route("/hello");
        assert!(r.is_some());
        (r.unwrap())(); // no side effect

        let r = router.find_route("/404");
        assert!(r.is_none());
    }
}

