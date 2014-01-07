extern mod http;

use http::server::{Request, ResponseWriter};
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};

use views::{Action, SafeHtmlString};

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

impl Action for NotFound {
    fn render(&self, print: |&SafeHtmlString| -> ()) {
        print(&SafeHtmlString::new("This page could not be found"));
    }
}

pub fn not_found(_: &Request, response: &mut ResponseWriter) -> ~Action {
    response.status = http::status::NotFound;
    ~NotFound as ~Action
}
