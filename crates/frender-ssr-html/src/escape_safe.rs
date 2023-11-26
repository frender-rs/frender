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

pub struct UnquotedAttribute;

impl EscapeSafe for UnquotedAttribute {
    fn escape_safe<'a>(&mut self, input: &'a str) -> Cow<'a, str> {
        html_escape::encode_unquoted_attribute(input)
    }
}

#[test]
fn test() {
    assert_eq!(html_escape::encode_unquoted_attribute("id"), "id");
    assert_eq!(html_escape::encode_unquoted_attribute("id="), "id&#x3D;");
}

pub struct DoubleQuotedAttribute;

impl EscapeSafe for DoubleQuotedAttribute {
    fn escape_safe<'a>(&mut self, input: &'a str) -> Cow<'a, str> {
        html_escape::encode_double_quoted_attribute(input)
    }
}

pub struct Safe;

impl EscapeSafe for Safe {
    fn escape_safe<'a>(&mut self, input: &'a str) -> Cow<'a, str> {
        html_escape::encode_safe(input)
    }
}
