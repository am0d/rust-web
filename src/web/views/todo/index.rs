extern mod extra;
extern mod http;

use super::super::{View, SafeHtmlString};
use super::super::models::Todo;

pub struct TodoIndexView<'self> {
    model: &'self [Todo]
}

impl<'self> TodoIndexView<'self> {
    pub fn new(m: &'self [Todo]) -> TodoIndexView<'self> {
        TodoIndexView {
            model: m//.clone()
        }
    }
}

impl<'self> View for TodoIndexView<'self> {
    fn render(&self, print: |&SafeHtmlString| -> ()) {
        print(&SafeHtmlString::new("<!DOCTYPE html>
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

    <div class=\"container\">"));

        if self.model.len() > 0 {
            print(&SafeHtmlString::new("<ul class=\"list-group\">\n"));
            for todo in self.model.iter() {
                print(&SafeHtmlString::new("<li class=\"list-group-item\">"));
                print(&SafeHtmlString::new("<a href=\"/todos/"));
                print(&SafeHtmlString::new(todo.id.to_str()));
                print(&SafeHtmlString::new("\">"));
                print(&todo.description.as_safe_string());
                print(&SafeHtmlString::new("</a></li>\n"));
            }
            print(&SafeHtmlString::new("</ul>\n"));
        }
        else {
            print(&SafeHtmlString::new("There are no todos in the system yet"));
        }

        print(&SafeHtmlString::new("</div><!-- /.container -->


    <!-- Bootstrap core JavaScript
    ================================================== -->
    <!-- Placed at the end of the document so the pages load faster -->
    <script src=\"/assets/js/jquery.js\"></script>
    <script src=\"/assets/js/bootstrap.min.js\"></script>
  </body>
</html>"));
    }
}

