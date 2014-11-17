use regex::Regex;

#[deriving(Clone)]
enum Method {
    Delete,
    Get,
    Patch,
    Post,
    Put
}

//#[deriving(Clone)]
struct Route<T> {
    method: Method,
    regex: Regex,
    pattern: String,
    handler: T
}

impl<T:Clone> Clone for Route<T> {
    fn clone(&self) -> Route<T> {
        Route {
            method: self.method.clone(),
            regex: self.regex.clone(),
            pattern: self.pattern.clone(),
            handler: self.handler.clone()
        }
    }
}

//#[deriving(Clone)]
pub struct Router<T> {
    routes: Vec<Route<T>>
}


impl<T:Copy> Clone for Router<T> {
    fn clone(&self) -> Router<T> {
        //let mut routes = self.routes.iter().map(|r| *r);
        Router {
            routes: self.routes.iter().map(|r| {
                        Route {
                            method: r.method.clone(),
                            regex: r.regex.clone(),
                            pattern: r.pattern.clone(),
                            handler: r.handler
                        }
                    }).collect()
        }
    }
}

impl<T> Router<T> {
    pub fn new () -> Router<T> {
        Router {
            routes: vec![]
        }
    }

    pub fn add_route(&mut self, pattern: &str, handler: T) {
        match Regex::new(pattern) {
            Ok(r) => {
                self.routes.push(Route {
                    method: Get, 
                    regex: r, 
                    pattern: String::from_str(pattern),
                    handler: handler
                });
            }
            Err(e) => {
                panic!("Error compiling route regex: {}", e);
            }
        }
    }

    pub fn find_route<'a> (&'a self, url: String) -> Option<&'a T> {
        for route in self.routes.iter() {
            let h = &route.handler;
            if route.regex.is_match(url.as_slice()) {
                return Some(h)
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::Router;

    fn route1() {
    }

    #[test]
    fn add_route() {
        let mut router = Router::<fn()>::new();

        router.add_route("^/", route1);

        let r = router.find_route("/".to_string());
        assert!(r.is_some());
    }

    #[test]
    fn dont_match_routes() {
        let mut router = Router::<fn()>::new();

        router.add_route("^/$", route1);

        let r = router.find_route("/missing".to_string());
        assert!(r.is_none());
    }
}
