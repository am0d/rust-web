extern mod extra;
extern mod http;

extern mod utils;

use std::vec;

use std::rt::io::Writer;

use http::server::{Request, ResponseWriter};
//use http::method::{Get};

use utils::{get_url, not_found};



pub struct TodoController<'self> {
    request: &'self Request,
    response: &'self mut ResponseWriter<'self>
}

impl<'self> TodoController<'self> {
    pub fn new<'a>(r: &'a Request, rw: &'a mut ResponseWriter<'a>) -> TodoController<'a> {
        TodoController {
            request: r,
            response: rw
        }
    }

    pub fn dispatch_request(&mut self) {
        match get_url(self.request) {
            ~"/todos" | ~"/todos/" => {
                self.Index();
            },
            _ => {
                not_found(self.request, self.response);
            }
        }
    }

    pub fn Index(&mut self) {
        let mut todo_list: ~[~Todo] = vec::build(|push| {
            push(Todo::new(~"Finish this wonderful framework!"));
            push(Todo::new(~"Make it more generic"));
            push(Todo::new(~"Learn rust"));
        });
        self.response.write(bytes!("<!DOCTYPE html>
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

        if todo_list.len() > 0 {
            self.response.write(bytes!("<ul>
"));
            for todo in todo_list.iter() {
                self.response.write(fmt!("<li>%s</li>", todo.description).into_bytes());
            }
            self.response.write(bytes!("</ul>
"));
        }
        else {
            self.response.write(bytes!("There are no todos in the system yet"));
        }

        self.response.write(bytes!("</div><!-- /.container -->


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
