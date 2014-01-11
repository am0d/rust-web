#[feature(globs)];

extern mod extra;
extern mod http;
extern mod pcre;

use std::task;
use extra::time;

use std::io::net::ip::{SocketAddr, Ipv4Addr};

use http::server::{Config, Server, Request, ResponseWriter};
use http::method::{Get};

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
    router: ~router::Router
}

impl HelloWorldServer {
    fn new() -> HelloWorldServer {
        HelloWorldServer {
            router: ~router::Router::new()
        }
    }

    fn log_request(&self, _r: &Request, response: &mut ResponseWriter) {
        print!("{} \"{}\" {} {} {}\n", _r.method.to_str(), get_url(_r), time::now().rfc822(), _r.remote_addr.unwrap().to_str(), response.status.code() as uint);
    }

    fn dispatch_request(&self, request: &Request, response: &mut ResponseWriter) {
        let handler = self.router.find_route(get_url(request));

        let action = match handler {
            Some(h) => {
                h(request, response)
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
        //do task::try {
            self.dispatch_request(_r,w);
        //};

        self.log_request(_r, w);
    }
}

fn main() {
    let mut server = HelloWorldServer::new();

    server.router.add_route("^/todos/?$", TodoController::Index);
    server.router.add_route("^/todos/(\\d+)$", TodoController::Details);

    server.router.add_route("^/assets/.*", StaticController::Get);
    server.router.add_route("^/$", StaticController::Get);

    println("Rust server up and running");
    server.serve_forever();
}
