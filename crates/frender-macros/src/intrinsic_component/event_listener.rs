use darling::ToTokens;
use quote::quote;
use syn::parse::Parse;

#[derive(Clone)]
pub struct FieldDeclarationEventListener {
    pub at_token: syn::Token![@],
    pub ty: syn::Type,
}
impl FieldDeclarationEventListener {
    pub fn to_ts_dom_bounds(&self, crate_path: impl ToTokens) -> proc_macro2::TokenStream {
        let Self { ty, .. } = self;
        quote! {
            #crate_path::frender_html::props::UpdateDomEventListener<#ty>
        }
    }
}

impl Parse for FieldDeclarationEventListener {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            at_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}
