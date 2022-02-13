mod case;
mod component_data;
mod component_to_tokens;
mod err;
mod props_data;
mod props_to_tokens;
mod rsx_data;
mod rsx_to_tokens;

use proc_macro::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn component(args: TokenStream, input: TokenStream) -> TokenStream {
    use component_data::*;
    let attr_args = parse_macro_input!(args as syn::AttributeArgs);
    let input_fn = parse_macro_input!(input as syn::ItemFn);

    let (comp, error) = ComponentDefinition::try_from_attrs_and_fn(attr_args, input_fn)
        .map_or_else(|(comp, err)| (comp, Some(err)), |comp| (comp, None));

    let mut t = comp.into_token_stream();
    if let Some(error) = error {
        t.append_all(error.write_errors());
    }

    t.into()
}

#[proc_macro]
pub fn snake_to_camel(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as syn::LitStr);
    let lit = case::snake_to_camel_lit_str(&lit.value(), lit.span());
    lit.to_token_stream().into()
}

#[proc_macro]
pub fn ident_snake_to_camel(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as syn::Ident);
    let lit = case::snake_to_camel_lit_str(&ident.to_string(), ident.span());
    lit.to_token_stream().into()
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let value = parse_macro_input!(input as rsx_data::RsxChild);

    value.to_token_stream().into()
}

#[proc_macro]
pub fn def_props(input: TokenStream) -> TokenStream {
    let value = parse_macro_input!(input as props_data::PropsDefinition);
    value.to_token_stream().into()
}
