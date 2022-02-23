mod case;
mod component_data;
mod component_to_tokens;
mod custom_element;
mod err;
mod props_data;
mod props_to_tokens;
mod rsx_data;
mod rsx_to_tokens;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn component(args: TokenStream, input: TokenStream) -> TokenStream {
    use component_data::*;
    let attr_args = parse_macro_input!(args as syn::AttributeArgs);
    let input_fn = parse_macro_input!(input as syn::ItemFn);

    let comp = ComponentDefinition::from_attrs_and_fn(attr_args, input_fn);

    comp.into_ts().into()
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
    use props_data::PropsDefinitionWithOptions;
    let value = parse_macro_input!(input with PropsDefinitionWithOptions::parse_proc_macro_input);
    let mut ts = proc_macro2::TokenStream::new();
    value.into_tokens(&mut ts);
    ts.into()
}

#[proc_macro_attribute]
pub fn custom_element(args: TokenStream, input: TokenStream) -> TokenStream {
    use custom_element::*;

    match CustomElementDefinition::parse_input(args, input) {
        Ok(v) => v.into_ts().into(),
        Err(error) => error.write_errors().into(),
    }
}
