extern mod http;

use std::rt::io::Writer;

use http::server::{Request, ResponseWriter};
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};

// Extension methods
pub fn get_url(request: &Request) -> ~str {
    match &request.request_uri {
        &Star => ~"*",
        &AbsoluteUri(ref url) => url.to_str(),
        &AbsolutePath(ref url) => url.to_owned(),
        &Authority(ref url) => url.to_owned()
    }
}

pub fn not_found(_request: &Request, response: &mut ResponseWriter) {
    response.status = http::status::NotFound;
    response.write(bytes!("This page could not be found"));
}

