use std::borrow::Cow;

pub trait EscapeSafe {
    fn escape_safe(self, input: &str) -> Cow<str>;
}

impl<F: FnOnce(&str) -> Cow<'_, str>> EscapeSafe for F {
    fn escape_safe(self, input: &str) -> Cow<str> {
        self(input)
    }
}
