use darling::{FromMeta, ToTokens};
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::spanned::Spanned;

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct BgOptionsInput {
    /// Defaults to `::frender::bg`
    pub path: Option<syn::Path>,
}

#[derive(Debug)]
pub struct BgOptions {
    options: BgOptionsInput,
    pub span: Span,
}

impl Default for BgOptions {
    fn default() -> Self {
        Self {
            options: Default::default(),
            span: Span::call_site(),
        }
    }
}

impl FromMeta for BgOptions {
    fn from_word() -> darling::Result<Self> {
        Ok(Default::default())
    }

    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        BgOptionsInput::from_list(items).map(|options| Self {
            options,
            span: Span::call_site(),
        })
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        Self::from_string_and_span(value, Span::call_site())
    }

    fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
        (match *item {
            // bg
            syn::Meta::Path(ref word_bg) => Ok(Self {
                options: Default::default(),
                span: word_bg.span(),
            }),
            // bg
            syn::Meta::List(ref list) => {
                let span = list.path.span();
                BgOptionsInput::from_meta(item).map(|options| Self { span, options })
            }
            syn::Meta::NameValue(ref value) => match &value.lit {
                lit @ syn::Lit::Str(_) => syn::Path::from_value(lit).map(|path| Self {
                    options: BgOptionsInput { path: Some(path) },
                    span: value.path.span(),
                }),
                lit => Self::from_value(lit),
            },
        })
        .map_err(|e| e.with_span(item))
    }
}

impl BgOptions {
    pub fn path_to_ts(&self) -> TokenStream {
        self.options.path.as_ref().map_or_else(
            || quote_spanned! (self.span=> bg ),
            ToTokens::to_token_stream,
        )
    }

    fn from_string_and_span(value: &str, span: Span) -> Result<BgOptions, darling::Error> {
        syn::Path::from_string(value).map(|path| Self {
            options: BgOptionsInput { path: Some(path) },
            span,
        })
    }
}
