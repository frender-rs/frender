use std::borrow::Cow;

pub trait EscapeSafe {
    fn escape_safe<'a>(&mut self, input: &'a str) -> Cow<'a, str>;
}

impl<F: FnMut(&str) -> Cow<str>> EscapeSafe for F {
    fn escape_safe<'a>(&mut self, input: &'a str) -> Cow<'a, str> {
        self(input)
    }
}

pub struct EscapeSafeRefMut<'a, E: EscapeSafe>(pub &'a mut E);

impl<E: EscapeSafe> EscapeSafe for EscapeSafeRefMut<'_, E> {
    fn escape_safe<'a>(&mut self, input: &'a str) -> Cow<'a, str> {
        E::escape_safe(self.0, input)
    }
}
