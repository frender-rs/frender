use syn::parse::Parse;

#[derive(Clone)]
pub struct PrefixKeyword<K, C> {
    pub keyword: K,
    pub content: C,
}

impl<K: Parse, C: Parse> Parse for PrefixKeyword<K, C> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            keyword: input.parse()?,
            content: input.parse()?,
        })
    }
}

impl<K: syn::token::Token, C> syn::token::CustomToken for PrefixKeyword<K, C> {
    fn peek(cursor: syn::buffer::Cursor) -> bool {
        K::peek(cursor)
    }

    fn display() -> &'static str {
        K::display()
    }
}
