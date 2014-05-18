pub use super::models;

//pub use self::todo;

pub mod todo;

pub trait Action {
    fn render(&self, &mut Writer);
}

pub struct SafeHtmlString {
    val: StrBuf
}

impl SafeHtmlString {
    pub fn new<'a>(v: &'a str) -> SafeHtmlString {
        SafeHtmlString {
            val: v.into_strbuf()
        }
    }

    #[inline]
    pub fn to_str(&self) -> StrBuf {
        return self.val.clone()
    }
}

pub trait AsSafeString {
    fn as_safe_string(&self) -> SafeHtmlString;
}

pub struct RawHtmlString {
    val: StrBuf
}

impl RawHtmlString {
    pub fn new(v: &str) -> RawHtmlString {
        RawHtmlString {
            val: v.into_strbuf()
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
        let mut buffer = StrBuf::with_capacity(self.char_len());

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

impl AsSafeString for StrBuf {
    fn as_safe_string(&self) -> SafeHtmlString {
        let mut buffer = StrBuf::with_capacity(self.len());

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
