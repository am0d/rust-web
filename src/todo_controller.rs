extern mod extra;
extern mod http;

extern mod utils;

use std::vec;

//use std::os;
//use std::path::{Path, GenericPath};

use std::io;
//use std::io::ReaderUtil;

use std::rt::io::Writer;
//use extra::time;

use http::server::{Request, ResponseWriter};
use http::server::request::{AbsoluteUri, AbsolutePath};
//use http::method::{Get};

use utils::{get_url, not_found};

struct TodoController;

impl TodoController {
    pub fn new() -> TodoController {
        TodoController
    }

    pub fn dispatch_request(&self, request: &Request, response: &mut ResponseWriter) {
        match get_url(request) {
            ~"/todos" | ~"/todos/" => {
                self.Index(request, response);
            },
            _ => {
                not_found(request, response);
            }
        }
    }

    pub fn Index(&self, request: &Request, response: &mut ResponseWriter) {
        let mut todo_list: ~[~Todo] = vec::build(|push| {
            push(Todo::new(~"Finish this wonderful framework!"));
            push(Todo::new(~"Make it more generic"));
            push(Todo::new(~"Learn rust"));
        });
        response.write(bytes!("<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <meta name=\"description\" content=\"\">
    <meta name=\"author\" content=\"\">
    <link rel=\"shortcut icon\" href=\"../../assets/ico/favicon.png\">

    <title>Starter Template for Bootstrap</title>

    <!-- Bootstrap core CSS-->
    <link href=\"/static/css/bootstrap.css\" rel=\"stylesheet\">

    <!-- Custom styles for this template-->
    <link href=\"/static/css/starter-template.css\" rel=\"stylesheet\">

    <!-- HTML5 shim and Respond.js IE8 support of HTML5 elements and media queries -->
    <!--[if lt IE 9]>
      <script src=\"../../assets/js/html5shiv.js\"></script>
      <script src=\"../../assets/js/respond.min.js\"></script>
    <![endif]-->
  </head>

  <body>

    <div class=\"navbar navbar-inverse navbar-fixed-top\">
      <div class=\"container\">
        <div class=\"navbar-header\">
          <button type=\"button\" class=\"navbar-toggle\" data-toggle=\"collapse\" data-target=\".navbar-collapse\">
            <span class=\"icon-bar\"></span>
            <span class=\"icon-bar\"></span>
            <span class=\"icon-bar\"></span>
          </button>
          <a class=\"navbar-brand\" href=\"#\">Project name</a>
        </div>
        <div class=\"collapse navbar-collapse\">
          <ul class=\"nav navbar-nav\">
            <li><a href=\"/\">Home</a></li>
            <li class=\"active\"><a href=\"/todos\">Todos</a></li>
            <li><a href=\"#contact\">Contact</a></li>
          </ul>
        </div><!--/.nav-collapse -->
      </div>
    </div>

    <div class=\"container\">"));

        response.write(bytes!("</div><!-- /.container -->


    <!-- Bootstrap core JavaScript
    ================================================== -->
    <!-- Placed at the end of the document so the pages load faster -->
    <script src=\"../../assets/js/jquery.js\"></script>
    <script src=\"../../dist/js/bootstrap.min.js\"></script>
  </body>
</html>"));
    }
}

// Models
struct Todo {
    description: ~str,
    completed: bool
}

impl Todo {
    pub fn new(name: ~str) -> ~Todo {
        ~Todo {
            description: name,
            completed: false
        }
    }
}
