extern mod extra;
extern mod http;
extern mod pcre;

use std::os;
use std::path::{Path, GenericPath};

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::Writer;
use std::io::fs::File;
use std::io;
use extra::time;

use http::server::{Config, Server, Request, ResponseWriter};
use http::server::request::AbsolutePath;
use http::method::{Get};

use self::utils::{not_found, get_url};
use self::todo_controller::TodoController;

use http::headers::content_type::MediaType;

use pcre::Pcre;
use pcre::{PCRE_CASELESS};

pub mod todo_controller;
pub mod utils;
pub mod models;
pub mod views;

// Controllers
trait Controller {
    fn dispatch_request(request: &Request, response: &mut ResponseWriter);
}

struct StaticController {
    working_dir: Path
}

impl StaticController {
    fn new() -> StaticController {
        let cwd = os::getcwd();
        StaticController {
            working_dir: cwd
        }
    }

    fn Get (&self, request: &Request, response: &mut ResponseWriter) {
        let url = get_url(request);
        let mut file_path: PosixPath = self.working_dir.join(url.slice_from(1));

        if file_path.exists() {
            if !file_path.is_file() {
                // try index.html, default.html in a directory
                file_path = file_path.join("index.html");
                if !file_path.exists() || !file_path.is_file() {
                    file_path = file_path.with_filename("default.html");
                    if !file_path.exists() || !file_path.is_file() {
                        not_found(request, response);
                        return;
                    }
                }
            }

            let f = io::result(|| File::open(&file_path));
            match f {
                Ok(mut reader) => {
                    response.headers.content_type = match file_path.extension_str() {
                        Some(".css") => {
                            Some(MediaType {
                                type_: ~"text",
                                subtype: ~"css",
                                parameters: ~[]
                            })
                        }
                        Some(".js") => {
                            Some(MediaType {
                                type_: ~"text",
                                subtype: ~"javascript",
                                parameters: ~[]
                            })
                        }
                        _ => None
                    };

                    //let reader = f.get_mut_ref();
                    let file_contents = reader.read_to_end();

                    response.headers.content_length = Some(file_contents.len());

                    response.write(file_contents);
                },
                _ => {
                    not_found(request, response);
                }
            }
        }
        else {
            not_found(request, response);
        }
    }
}

// Web server part
#[deriving(Clone)]
struct HelloWorldServer;

impl HelloWorldServer {
    fn new() -> HelloWorldServer {
        HelloWorldServer
    }

    fn log_request(&self, _r: &Request, response: &mut ResponseWriter) {
        print!("{} \"{}\" {} {} {}\n", _r.method.to_str(), get_url(_r), time::now().rfc822(), _r.remote_addr.unwrap().to_str(), response.status.code() as uint);
    }

    fn dispatch_request(&self, request: &Request, response: &mut ResponseWriter) {
        let r = Pcre::compile_with_options("^/todos/?.*", PCRE_CASELESS).unwrap();
        match (&request.method, &request.request_uri) {
            (&Get, &AbsolutePath(ref url)) => {
                // All files are static for now!
                if r.exec(*url).is_some() {
                    TodoController::new().dispatch_request(request, response);
                }
                else {
                    StaticController::new().Get(request, response);
                }
            },
            (_, _) => {
                not_found(request, response);
            }
        }
    }
}

impl Server for HelloWorldServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8080 } }
    }

    fn handle_request(&self, _r: &Request, w: &mut ResponseWriter) {
        self.dispatch_request(_r, w);
        self.log_request(_r, w);
    }
}

fn main() {
    println("Rust server up and running");
    HelloWorldServer.serve_forever();
}
