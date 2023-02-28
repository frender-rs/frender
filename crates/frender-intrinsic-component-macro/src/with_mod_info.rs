use std::borrow::Cow;

use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;

use crate::utils::{grouped::Parenthesized, kw::PrefixKeyword};

use super::{kw, IntrinsicComponentPropsData, ModInfo};

pub struct IntrinsicComponentPropsDataWithModInfo {
    pub fully_typed: Option<PrefixKeyword<kw::fully_typed, Parenthesized<ModInfo>>>,
    pub simply_typed: Option<PrefixKeyword<kw::simply_typed, Parenthesized<ModInfo>>>,
    pub props: IntrinsicComponentPropsData,
}

impl Parse for IntrinsicComponentPropsDataWithModInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fully_typed: input.parse()?,
            simply_typed: input.parse()?,
            props: input.parse()?,
        })
    }
}

impl IntrinsicComponentPropsDataWithModInfo {
    pub fn into_ts(self, explicit_path: Option<TokenStream>) -> TokenStream {
        let Self {
            fully_typed,
            simply_typed,
            props,
        } = self;
        let crate_path = explicit_path.unwrap_or_else(|| quote!(::frender_html));

        match (fully_typed, simply_typed) {
            (Some(f), Some(s)) => {
                let fully = f
                    .content
                    .content
                    .to_ts(props.clone().into_ts_fully(&crate_path));

                let simply = s.content.content.to_ts(props.into_ts_simply(&crate_path));

                quote! {
                    #fully
                    #simply
                }
            }
            (Some(PrefixKeyword { content, .. }), None) => {
                content.content.to_ts(props.into_ts_fully(&crate_path))
            }
            (None, Some(PrefixKeyword { content, .. })) => {
                content.content.to_ts(props.into_ts_simply(&crate_path))
            }
            (None, None) => TokenStream::new(),
        }
    }
}
