use syn::parse::Parse;

use super::grouped::Braced;

pub struct ItemFn {
    pub outer_attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub sig: syn::Signature,
    pub block: Braced<proc_macro2::TokenStream>,
}

impl Parse for ItemFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let outer_attrs = input.call(syn::Attribute::parse_outer)?;
        let vis = input.parse()?;
        let sig = input.parse()?;
        let block = input.parse()?;

        Ok(Self {
            outer_attrs,
            vis,
            sig,
            block,
        })
    }
}
