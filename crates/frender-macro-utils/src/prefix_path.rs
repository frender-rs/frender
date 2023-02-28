use syn::parse::Parse;

use super::grouped::Bracketed;

pub struct ExplicitPath {
    pub at_token: syn::Token![@],
    pub bracketed_path: Bracketed<proc_macro2::TokenStream>,
}

pub struct PrefixPath<S> {
    pub path: Option<ExplicitPath>,
    pub rest: S,
}

impl<S> PrefixPath<S> {
    pub fn map<R>(self, f: impl FnOnce(S, Option<proc_macro2::TokenStream>) -> R) -> R {
        f(self.rest, self.path.map(|p| p.bracketed_path.content))
    }
}

impl<S: Parse> Parse for PrefixPath<S> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = if input.peek(syn::Token![@]) && input.peek2(syn::token::Bracket) {
            Some(ExplicitPath {
                at_token: input.parse()?,
                bracketed_path: input.parse()?,
            })
        } else {
            None
        };

        let rest = input.parse()?;

        Ok(Self { path, rest })
    }
}
