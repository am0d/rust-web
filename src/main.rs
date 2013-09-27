extern mod extra;
extern mod http;

extern mod todo_controller;
extern mod utils;

use std::os;
use std::path::{Path, GenericPath};

use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::rt::io::{Writer, Open};
use std::rt::io::file::FileInfo;
use std::rt::io::extensions::ReaderUtil;
use extra::time;

use http::server::{Config, Server, ServerUtil, Request, ResponseWriter};
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
use http::method::{Get};

use utils::{not_found, get_url};
use todo_controller::TodoController;

use http::headers::content_type::MediaType;

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
        let mut file_path: PosixPath = self.working_dir.push(url);

        if file_path.exists() {
            if !file_path.is_file() {
                // try index.html, default.html in a directory
                file_path = file_path.push("index.html");
                if !file_path.exists() || !file_path.is_file() {
                    file_path = file_path.with_filename("default.html");
                    if !file_path.exists() || !file_path.is_file() {
                        not_found(request, response);
                        return;
                    }
                }
            }

            let mut f = file_path.open_reader(Open);
            match f {
                Some(_) => {
                    match file_path.filetype() {
                        Some(".css") => {
                            response.headers.content_type = Some(MediaType {
                                type_: ~"text",
                                subtype: ~"css",
                                parameters: ~[]
                            });
                        }
                        _ => ()
                    }

                    let reader = f.get_mut_ref();
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
        printf!("%s \"", _r.method.to_str());

        match &_r.request_uri {
            &Star => print("*"),
            &AbsoluteUri(ref url) => printf!("%s", url.to_str()),
            &AbsolutePath(ref url) => printf!("%s", url.to_owned()),
            &Authority(ref url) => printf!("%s", url.to_owned())
        };

        printf!("\" %s %s %u", time::now().rfc822(), _r.remote_addr.unwrap().to_str(), response.status.code() as uint);

        println("");
    }

    fn dispatch_request(&self, request: &Request, response: &mut ResponseWriter) {
        match (&request.method, &request.request_uri) {
            (&Get, &AbsolutePath(ref url)) => {
                // All files are static for now!
                if url.starts_with("/todos") {
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
