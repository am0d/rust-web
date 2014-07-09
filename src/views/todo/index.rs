use super::super::{Action, AsSafeString};
use super::super::models::Todo;

pub struct TodoIndexView<'a> {
    model: Vec<Todo>
}

impl<'a> TodoIndexView<'a> {
    pub fn new(m: Vec<Todo>) -> TodoIndexView<'a> {
        TodoIndexView {
            model: m//.clone()
        }
    }
}

#[allow(unused_must_use)]
impl<'a> Action for TodoIndexView<'a> {
    fn render(&self, out: &mut Writer) {
        out.write_str("<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <meta name=\"description\" content=\"\">
    <meta name=\"author\" content=\"\">
    <link rel=\"shortcut icon\" href=\"/assets/ico/favicon.png\">

    <title>Starter Template for Bootstrap</title>

    <!-- Bootstrap core CSS-->
    <link href=\"/assets/css/bootstrap.css\" rel=\"stylesheet\">

    <!-- Custom styles for this template-->
    <link href=\"/assets/css/starter-template.css\" rel=\"stylesheet\">

    <!-- HTML5 shim and Respond.js IE8 support of HTML5 elements and media queries -->
    <!--[if lt IE 9]>
      <script src=\"/assets/js/html5shiv.js\"></script>
      <script src=\"/assets/js/respond.min.js\"></script>
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

    <div class=\"container\">");

        if self.model.len() > 0 {
            out.write_str("<ul class=\"list-group\">\n");
            for todo in self.model.iter() {
                out.write_str("<li class=\"list-group-item\">");
                out.write_str("<a href=\"/todos/");
                out.write_str(todo.id.to_str().as_safe_string().to_str().as_slice());
                out.write_str("\">");
                out.write_str(todo.description.as_safe_string().to_str().as_slice());
                out.write_str("</a></li>\n");
            }
            out.write_str("</ul>\n");
        }
        else {
            out.write_str("There are no todos in the system yet");
        }

        out.write_str("</div><!-- /.container -->


    <!-- Bootstrap core JavaScript
    ================================================== -->
    <!-- Placed at the end of the document so the pages load faster -->
    <script src=\"/assets/js/jquery.js\"></script>
    <script src=\"/assets/js/bootstrap.min.js\"></script>
  </body>
</html>");
    }
}

