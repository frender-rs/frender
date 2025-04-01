pub struct DoubleQuotedAttributeValue;
pub struct SingleQuotedAttributeValue;
pub struct UnquotedAttributeValue;

macro_rules! define_special_char {
    (
        $vis:vis const fn $test_should_encode:ident(&self, $ch:ident: $ch_ty:ty) -> $ret:ty;
        $(,)?

        $($Ty:ty = $special_char:pat),+
        $(,)?
    ) => {$(
        impl $Ty {
            $vis const fn $test_should_encode(&self, $ch: $ch_ty) -> $ret {
                matches!($ch, $special_char)
            }
        }
    )+};
}

define_special_char!(
    pub(crate) const fn test_should_encode(&self, ch: char) -> bool;,
    // https://html.spec.whatwg.org/#attribute-value-(double-quoted)-state
    DoubleQuotedAttributeValue = '"' | '&' | '\0',
    // https://html.spec.whatwg.org/#attribute-value-(single-quoted)-state
    SingleQuotedAttributeValue = '\'' | '&' | '\0',
    // https://html.spec.whatwg.org/#attribute-value-(unquoted)-state
    UnquotedAttributeValue =
        '\t' | '\n' | '\x0C' | '\r' | ' ' | '&' | '>' | '\0' | '"' | '\'' | '<' | '=' | '`',
);
