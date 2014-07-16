#![feature(macro_rules)]

extern crate collections;
extern crate time;
extern crate regex;
extern crate http;

use std::io::net::ip::{SocketAddr, Ipv4Addr};

use http::server::{Config, Server, Request, ResponseWriter};
//use http::method::{Get};
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

type RequestHandler = fn(&Request, &mut ResponseWriter) -> Box<views::Action>;

// Web server part
#[deriving(Clone)]
struct HelloWorldServer {
    router: router::Router<RequestHandler>
}

impl HelloWorldServer {
    fn new() -> HelloWorldServer {
        HelloWorldServer {
            router: router::Router::new()
        }
    }

    fn log_request(&self, _r: &Request, response: &mut ResponseWriter) {
        print!("{} \"{}\" {} {} {}\n", _r.method.to_string(), get_url(_r), time::now().rfc822(), _r.remote_addr.unwrap().to_string(), response.status.code() as uint);
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

        action.render(response);
    }
}

impl Server for HelloWorldServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8080 } }
    }

    fn handle_request(&self, _r: Request, w: &mut ResponseWriter) {
        use std::finally::try_finally;
        use std::task::failing;

        struct State <'a, 'b> {
            r: &'a Request,
            w: &'a mut ResponseWriter<'b>
        };

        let mut s = State {r: &_r, w: w};

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
                    use std::io::MemWriter;
                    //use std::rt::task::Task;
                    //use std::rt::local::Local;

                    // get a backtrace for the failure
                    let mut trace = MemWriter::new();
                    drop(::std::rt::backtrace::write(&mut trace));

                    // The lines below don't yet work.
                    // There does not appear to be any way to get the failure
                    // reason in this context.
                    //let task = Local::borrow(None::<Task>);
                    //let cause = task.get().unwinder.cause;

                    // Set the status to 500 if the request failed
                    w.status = InternalServerError;
                    drop(w.write(b"Internal server error\n"));

                    // print the backtrace TODO make this more secure
                    drop(w.write(trace.unwrap().as_slice()));
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
           "^/todos/?$" => TodoController::index,
           "^/todos/(\\d+)$" => TodoController::details,

           "^/fail" => TodoController::fail,

           "^/assets/.*" => StaticController::get,
           "^/$" => StaticController::get);

    println!("{}", "Rust server up and running");
    server.serve_forever();
}
