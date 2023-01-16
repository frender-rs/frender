macro_rules! css_ident_to_str {
    ( $($start:ident)? $(- $id:ident)* ) => {
        ::core::concat!(
            $(::core::stringify!($start),)?
            $(
                ::core::concat!("-", ::core::stringify!($id))
            )*
        )
    };
}

/// Produces an style attribute.
/// Please see <https://drafts.csswg.org/css-style-attr/#syntax>
macro_rules! style {
    () => {};
}
