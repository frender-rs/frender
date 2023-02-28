use darling::{FromMeta, ToTokens};

#[derive(Debug, Clone, Copy)]
pub enum ValueOrPath<V, P: From<syn::Path> = syn::Path> {
    /// from_value
    Value(V),
    /// from_list
    Path(P),
}

impl<V: ToTokens, P: From<syn::Path> + ToTokens> ToTokens for ValueOrPath<V, P> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            ValueOrPath::Value(v) => v.to_tokens(tokens),
            ValueOrPath::Path(v) => v.to_tokens(tokens),
        }
    }

    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            ValueOrPath::Value(v) => v.to_token_stream(),
            ValueOrPath::Path(v) => v.to_token_stream(),
        }
    }

    fn into_token_stream(self) -> proc_macro2::TokenStream
    where
        Self: Sized,
    {
        match self {
            ValueOrPath::Value(v) => v.into_token_stream(),
            ValueOrPath::Path(v) => v.into_token_stream(),
        }
    }
}

impl<V: FromMeta, P: From<syn::Path>> FromMeta for ValueOrPath<V, P> {
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        let len = items.len();
        if len == 1 {
            let item = &items[0];
            match item {
                syn::NestedMeta::Meta(m) => match m {
                    syn::Meta::Path(p) => Ok(Self::Path(P::from(p.clone()))),
                    syn::Meta::List(_) => Err(darling::Error::unexpected_type("list")),
                    syn::Meta::NameValue(_) => Err(darling::Error::unexpected_type("value")),
                },
                syn::NestedMeta::Lit(value) => Err(darling::Error::unexpected_lit_type(value)),
            }
        } else if len > 1 {
            Err(darling::Error::too_many_items(1))
        } else {
            Err(darling::Error::too_few_items(1))
        }
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        V::from_value(value).map(Self::Value)
    }
}
