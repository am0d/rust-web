extern mod extra;
extern mod http;

extern mod utils;
extern mod models;

use models::Todo;

trait View {
    fn render(&self, &fn(&AsSafeString)) -> ~str;
}

pub struct SafeHtmlString {
    priv val: ~str
}

impl SafeHtmlString {
    pub fn into_bytes(&mut self) -> ~[u8] {
        return (self.val).into_bytes()
    }
}

pub trait AsSafeString {
    fn as_safe_string(&self) -> SafeHtmlString;
}

pub struct RawHtmlString {
    priv val: ~str
}

impl AsSafeString for RawHtmlString {
    fn as_safe_string(&self) -> SafeHtmlString {
        SafeHtmlString {
            val: self.val.clone()
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

pub struct TodoIndexView {
    model: ~[Todo]
}

pub fn IndexView(m: ~[Todo]) -> TodoIndexView {
    TodoIndexView {
        model: m.clone()
    }
}

impl View for TodoIndexView {
    fn render(&self, print: &fn(&AsSafeString)) -> ~str {
        let mut buffer = ~"";
        buffer.push_str("<!DOCTYPE html>
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

    <div class=\"container\">");

        if self.model.len() > 0 {
            buffer.push_str("<ul>\n");
            for todo in self.model.iter() {
                buffer.push_str("<li>");
                buffer.push_str(todo.description);
                buffer.push_str("</li>\n");
            }
            buffer.push_str("</ul>\n");
        }
        else {
            buffer.push_str("There are no todos in the system yet");
        }

        buffer.push_str("</div><!-- /.container -->


    <!-- Bootstrap core JavaScript
    ================================================== -->
    <!-- Placed at the end of the document so the pages load faster -->
    <script src=\"../../assets/js/jquery.js\"></script>
    <script src=\"../../dist/js/bootstrap.min.js\"></script>
  </body>
</html>");

        return buffer
    }
}
