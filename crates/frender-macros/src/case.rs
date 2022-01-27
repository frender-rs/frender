use lazy_static::lazy_static;
use proc_macro2::Span;
use regex::{Captures, Regex};

pub fn snake_to_camel(value: &str) -> std::borrow::Cow<str> {
    lazy_static! {
        static ref RE: Regex = Regex::new("_([a-z])").unwrap();
    }

    let value = RE.replace_all(&value, |cap: &Captures| {
        let c = &cap[1];
        c.to_uppercase()
    });

    value
}

pub fn snake_to_camel_lit_str(value: &str, span: Span) -> syn::LitStr {
    let value = snake_to_camel(value);
    let lit = syn::LitStr::new(value.as_ref(), span);
    lit
}
