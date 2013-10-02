#[link(name="views",
       vers="0.1")];

extern mod extra;
extern mod http;

extern mod utils;
extern mod models;

use models::Todo;

trait View {
    fn render(&self, &fn(&SafeHtmlString));
}

pub struct SafeHtmlString {
    priv val: ~str
}

impl SafeHtmlString {
    pub fn new<'a>(v: &'a str) -> SafeHtmlString {
        SafeHtmlString {
            val: v.to_owned()
        }
    }

    #[inline]
    pub fn to_str(&self) -> ~str {
        return self.val.to_owned()
    }
}

pub trait AsSafeString {
    fn as_safe_string(&self) -> SafeHtmlString;
}

pub struct RawHtmlString {
    priv val: ~str
}

impl RawHtmlString {
    pub fn new(v: &str) -> RawHtmlString {
        RawHtmlString {
            val: v.to_owned()
        }
    }
}

impl AsSafeString for RawHtmlString {
    fn as_safe_string(&self) -> SafeHtmlString {
        SafeHtmlString {
            val: self.val.to_owned()
        }
    }
}

impl AsSafeString for ~str {
    fn as_safe_string(&self) -> SafeHtmlString {
        use std::str;
        let mut buffer = str::with_capacity(self.char_len());

        for c in self.iter() {
            match c {
                '<' => buffer.push_str("&lt;"),
                '>' => buffer.push_str("&gt;"),
                '&' => buffer.push_str("&amp;"),
                _ => buffer.push_char(c)
            }
        }

        return SafeHtmlString {
            val: buffer
        }
    }
}

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
    fn render(&self, print: &fn(&SafeHtmlString)) {
        print(&SafeHtmlString::new("<!DOCTYPE html>
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

        if self.model.len() > 0 {
            print(&SafeHtmlString::new("<ul>\n"));
            for todo in self.model.iter() {
                print(&SafeHtmlString::new("<li>"));
                print(&todo.description.as_safe_string());
                print(&SafeHtmlString::new("</li>\n"));
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
    <script src=\"../../assets/js/jquery.js\"></script>
    <script src=\"../../dist/js/bootstrap.min.js\"></script>
  </body>
</html>"));
    }
}
