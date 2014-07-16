use http::server::{Request, ResponseWriter};
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
use http::status;

use views::Action;

// Extension methods
pub fn get_url(request: &Request) -> String {
    match &request.request_uri {
        &Star => String::from_str("*"),
        &AbsoluteUri(ref url) => url.to_string(),
        &AbsolutePath(ref url) => url.clone(),
        &Authority(ref url) => url.clone()
    }
}

struct NotFound;

#[allow(unused_must_use)]
impl Action for NotFound {
    fn render(&self, out: &mut Writer) {
        out.write_str("This page could not be found");
    }
}

pub fn not_found(_: &Request, response: &mut ResponseWriter) -> Box<Action> {
    response.status = status::NotFound;

    box NotFound as Box<Action>
}
