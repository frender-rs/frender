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

/// Currently <script> content is not perfectly encoded
/// and might cause safety issues.
///
/// See [Restrictions on the contents of raw text and escapable raw text elements](https://html.spec.whatwg.org/multipage/syntax.html#cdata-rcdata-restrictions);
///
/// See issue [Script escaping is incorrect](https://github.com/magiclen/html-escape/issues/1);
pub(crate) struct Script;

impl EscapeSafe for Script {
    fn escape_safe<'a>(&mut self, input: &'a str) -> Cow<'a, str> {
        html_escape::encode_script(input)
    }
}

pub(crate) struct Style;

impl EscapeSafe for Style {
    fn escape_safe<'a>(&mut self, input: &'a str) -> Cow<'a, str> {
        html_escape::encode_style(input)
    }
}

#[cfg(test)]
mod tests {
    #[ignore = "escaping script content is too complex"]
    #[test]
    fn script() {
        let hack = "</script";
        assert_eq!(html_escape::encode_script(hack), hack);

        let hack = "</script>";
        assert_eq!(html_escape::encode_script(hack), "<\\/script>");

        let hack = "</Script ";
        assert_eq!(html_escape::encode_script(hack), "<\\/Script ");

        // U+0009 CHARACTER TABULATION (tab), U+000A LINE FEED (LF), U+000C FORM FEED (FF), U+000D CARRIAGE RETURN (CR), U+0020 SPACE, U+003E GREATER-THAN SIGN (>), or U+002F SOLIDUS (/)
        let endings = [
            "\u{0009}", "\u{000A}", "\u{000C}", "\u{000D}", "\u{0020}", "\u{003E}", "\u{002F}",
        ];
        assert_eq!(
            endings
                .map(|v| "</script".to_string() + v)
                .map(|s| html_escape::encode_script(&s).into_owned()),
            endings.map(|v| "<\\/script".to_string() + v)
        );
    }
}
