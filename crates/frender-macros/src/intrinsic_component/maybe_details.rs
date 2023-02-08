use proc_macro2::TokenStream;
use syn::parse::Parse;

use crate::utils::grouped::Bracketed;

use super::kw;

#[derive(Clone)]
pub struct FieldDeclarationMaybeDetails {
    pub html_prop_name: Option<syn::LitStr>,
    pub impl_update: Option<Bracketed<FieldDeclarationMaybeDetailsImpl<kw::update>>>,
    pub impl_remove: Option<Bracketed<FieldDeclarationMaybeDetailsImpl<kw::remove>>>,
    pub html_element_method: Option<FieldDeclarationMaybeDetailsMethod>,
}

#[derive(Clone)]
pub struct FieldDeclarationMaybeDetailsImpl<K> {
    pub keyword: K,
    pub element_pat_ident: syn::Ident,
    pub rest: TokenStream,
}

#[derive(Clone)]
pub struct FieldDeclarationMaybeDetailsMethod {
    pub name: syn::Ident,
    pub deref_star_token: Option<syn::Token![*]>,
}

impl<K: Parse> Parse for FieldDeclarationMaybeDetailsImpl<K> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            keyword: input.parse()?,
            element_pat_ident: input.parse()?,
            rest: input.parse()?,
        })
    }
}

impl Parse for FieldDeclarationMaybeDetails {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut custom_impl: Option<_> = None;
        Ok(Self {
            html_prop_name: input.parse()?,
            impl_update: {
                if input.peek(syn::token::Bracket) {
                    let content;
                    let group_token = syn::bracketed!(content in input);

                    if content.peek(kw::update) {
                        Some(Bracketed {
                            group_token,
                            content: content.parse()?,
                        })
                    } else {
                        custom_impl = Some((group_token, content));
                        None
                    }
                } else {
                    None
                }
            },
            impl_remove: {
                if custom_impl.is_none() {
                    if input.peek(syn::token::Bracket) {
                        let content;
                        let group_token = syn::bracketed!(content in input);
                        custom_impl = Some((group_token, content))
                    }
                }
                if let Some((group_token, content)) = custom_impl {
                    Some(Bracketed {
                        group_token,
                        content: content.parse()?,
                    })
                } else {
                    None
                }
            },
            html_element_method: {
                if let Some(name) = input.parse()? {
                    Some(FieldDeclarationMaybeDetailsMethod {
                        name,
                        deref_star_token: input.parse()?,
                    })
                } else {
                    None
                }
            },
        })
    }
}
