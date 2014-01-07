use super::models;

//pub use self::todo;

pub mod todo;

pub trait Action {
    fn render(&self, |&SafeHtmlString| -> ());
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
