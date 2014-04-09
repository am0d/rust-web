use http::server::{Request, ResponseWriter};
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
use http::status;

use views::Action;

// Extension methods
pub fn get_url(request: &Request) -> ~str {
    match &request.request_uri {
        &Star => ~"*",
        &AbsoluteUri(ref url) => url.to_str(),
        &AbsolutePath(ref url) => url.to_owned(),
        &Authority(ref url) => url.to_owned()
    }
}

struct NotFound;

#[allow(unused_must_use)]
impl Action for NotFound {
    fn render(&self, out: &mut Writer) {
        out.write_str("This page could not be found");
    }
}

pub fn not_found(_: &Request, response: &mut ResponseWriter) -> ~Action {
    response.status = status::NotFound;
    ~NotFound as ~Action
}
