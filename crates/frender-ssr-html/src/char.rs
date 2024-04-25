/// [`html_escape::encode_safe`]
fn char_encode_safe(c: char) -> Option<&'static str> {
    match c {
        '&' => Some("&amp;"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&#x27;"),
        '/' => Some("&#x2F;"),
        _ => None,
    }
}

struct CharString {
    value: char,
    buf: [u8; 4],
}

impl CharString {
    fn new(value: char) -> Self {
        Self { value, buf: [0; 4] }
    }

    fn encode_safe(&mut self) -> &str {
        char_encode_safe(self.value).unwrap_or_else(|| self.value.encode_utf8(&mut self.buf))
    }
}

pub struct IterCharStringEncodeSafe {
    taken: bool,
    char_string: CharString,
}

impl IterCharStringEncodeSafe {
    pub fn new(c: char) -> Self {
        Self {
            taken: false,
            char_string: CharString::new(c),
        }
    }
}

impl async_str_iter::AsyncStrIterator for IterCharStringEncodeSafe {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        let this = self.get_mut();
        if this.taken {
            return std::task::Poll::Ready(None);
        }

        this.taken = true;
        let s = this.char_string.encode_safe();
        std::task::Poll::Ready(Some(s))
    }
}
