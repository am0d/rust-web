#[feature(macro_rules)];
#[feature(globs)];

extern crate collections;
extern crate time;
extern crate http;
extern crate pcre;

use std::io::net::ip::{SocketAddr, Ipv4Addr};

use http::server::{Config, Server, Request, ResponseWriter};
use http::method::{Get};
use http::status::InternalServerError;

use self::utils::{not_found, get_url};
use self::todo_controller::TodoController;


use static_controller::StaticController;

pub mod todo_controller;
pub mod static_controller;
pub mod utils;
pub mod models;
pub mod views;
pub mod router;

// Web server part
#[deriving(Clone)]
struct HelloWorldServer {
    router: router::Router<fn(&Request, &mut ResponseWriter) -> ~views::Action>
}

impl HelloWorldServer {
    fn new() -> HelloWorldServer {
        HelloWorldServer {
            router: router::Router::new()
        }
    }

    fn log_request(&self, _r: &Request, response: &mut ResponseWriter) {
        print!("{} \"{}\" {} {} {}\n", _r.method.to_str(), get_url(_r), time::now().rfc822(), _r.remote_addr.unwrap().to_str(), response.status.code() as uint);
    }

    fn dispatch_request(&self, request: &Request, response: &mut ResponseWriter) {
        let handler = self.router.find_route(get_url(request));

        let action = match handler {
            Some(h) => {
                (*h)(request, response)
            },
            None => {
                not_found(request, response)
            }
        };

        action.render(|s| {
            response.write(s.to_str().into_bytes());
        });
    }
}

impl Server for HelloWorldServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8080 } }
    }

    fn handle_request(&self, _r: &Request, w: &mut ResponseWriter) {
        use std::unstable::finally::try_finally;
        use std::task::failing;

        struct State <'a, 'b> {
            r: &'a Request,
            w: &'a mut ResponseWriter<'b>
        };

        let mut s = State {r: _r, w: w};

        try_finally(&mut s, (),
            |state, ()| {
                let r = state.r;
                let w = &mut state.w;
                // try to execute the request
                self.dispatch_request(r,*w);
            },
            |state| {
                let r = state.r;
                let w = &mut state.w;
                if failing() {
                    // Set the status to 500 if the request failed
                    w.status = InternalServerError;
                    drop(w.write(bytes!("Internal server error")));
                    drop(w.flush());
                }

                self.log_request(r, *w);
            });
    }
}

macro_rules! route(
    ($router:expr -> $($url:expr => $handler:expr),+) => {
        for &(url, handler) in vec!($(($url, $handler)),+).iter() {
            $router.add_route(url, handler);
        }
    }
    )

fn main() {
    let mut server = HelloWorldServer::new();

    route!(server.router -> 
           "^/todos/?$" => TodoController::Index,
           "^/todos/(\\d+)$" => TodoController::Details,

           "^/fail" => TodoController::Fail,

           "^/assets/.*" => StaticController::Get,
           "^/$" => StaticController::Get);

    println!("{}", "Rust server up and running");
    server.serve_forever();
}
