extern mod extra;
extern mod http;

use std::os;
use std::path::{Path, GenericPath};

use std::io;
use std::io::ReaderUtil;

use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::rt::io::Writer;
use extra::time;

use http::server::{Config, Server, ServerUtil, Request, ResponseWriter};
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
use http::method::{Get};
//use http::headers::content_type::MediaType;

// Extension methods
fn get_url(request: &Request) -> ~str {
    match &request.request_uri {
        &Star => ~"*",
        &AbsoluteUri(ref url) => url.to_str(),
        &AbsolutePath(ref url) => url.to_owned(),
        &Authority(ref url) => url.to_owned()
    }
}


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
        let file_path: PosixPath = self.working_dir.push(url);
        
        match io::file_reader(&file_path) {
            Ok(reader) => {
                let file_contents = reader.read_whole_stream();

                response.headers.content_length = Some(file_contents.len());

                response.write(file_contents);
            },
            _ => {
                response.status = http::status::NotFound;
                response.write(bytes!("This page could not be found"));
            }
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
                StaticController::new().Get(request, response);
            },
            (_, _) => {
                self.not_found(request, response);
            }
        }
    }

    fn not_found(&self, request: &Request, response: &mut ResponseWriter) {
        response.status = http::status::NotFound;
        response.write(bytes!("This page could not be found"));
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
