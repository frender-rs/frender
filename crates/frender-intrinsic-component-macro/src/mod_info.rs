use quote::quote;
use syn::parse::ParseStream;

use syn::parse::Parse;

use darling::ToTokens;

use syn;

#[derive(Clone)]
pub struct ModInfo {
    pub vis: syn::Visibility,
    pub mod_token: syn::Token![mod],
    pub mod_name: syn::Ident,
    pub mod_feature: syn::LitStr,
}

impl ModInfo {
    pub fn to_ts(&self, content: impl ToTokens) -> proc_macro2::TokenStream {
        let Self {
            vis,
            mod_token,
            mod_name,
            mod_feature,
        } = self;
        quote! {
            #[cfg(feature = #mod_feature)]
            #vis #mod_token #mod_name {
                #[allow(unused_imports)]
                use super::*;

                #content
            }
        }
    }
}

impl Parse for ModInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            vis: input.parse()?,
            mod_token: input.parse()?,
            mod_name: input.parse()?,
            mod_feature: input.parse()?,
        })
    }
}
