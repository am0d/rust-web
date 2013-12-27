extern mod extra;
extern mod http;

use super::super::{View, SafeHtmlString};
use super::super::models::Todo;

pub struct TodoDetailView<'a> {
    model: &'a Todo
}

impl<'a> TodoDetailView<'a> {
    pub fn new(m: &'a Todo) -> TodoDetailView<'a> {
        TodoDetailView {
            model: m//.clone()
        }
    }
}

impl<'a> View for TodoDetailView<'a> {
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

                print(&self.model.description.as_safe_string());

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

