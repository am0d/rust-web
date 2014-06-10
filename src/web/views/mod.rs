pub use super::models;

//pub use self::todo;

pub mod todo;

pub trait Action {
    fn render(&self, &mut Writer);
}

pub struct SafeHtmlString {
    val: String
}

impl SafeHtmlString {
    pub fn new<'a>(v: &'a str) -> SafeHtmlString {
        SafeHtmlString {
            val: String::from_str(v)
        }
    }

    #[inline]
    pub fn to_str(&self) -> String {
        return self.val.clone()
    }
}

pub trait AsSafeString {
    fn as_safe_string(&self) -> SafeHtmlString;
}

pub struct RawHtmlString {
    val: String
}

impl RawHtmlString {
    pub fn new(v: &str) -> RawHtmlString {
        RawHtmlString {
            val: String::from_str(v)
        }
    }
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
        let mut buffer = String::with_capacity(self.char_len());

        for c in self.chars() {
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

impl AsSafeString for String {
    fn as_safe_string(&self) -> SafeHtmlString {
        let mut buffer = String::with_capacity(self.len());

        for c in self.as_slice().chars() {
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
