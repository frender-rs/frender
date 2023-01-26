mod auto_intrinsic;
mod case;
mod component_data;
mod component_to_tokens;
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

    let comp = ComponentDefinition::from_attrs_and_fn(attr_args, input);

    comp.into_ts()
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

/// intrinsic components are not auto prepend with `self::intrinsic_components::`
#[proc_macro]
pub fn rsx_xml_with_full_path(input: TokenStream) -> TokenStream {
    let value = match syn::parse::<rsx_data::OptionalCratePathAndRsxChild>(input) {
        Ok(v) => v,
        Err(err) => {
            return proc_macro::TokenTree::Group(proc_macro::Group::new(
                proc_macro::Delimiter::Brace,
                err.to_compile_error().into(),
            ))
            .into()
        }
    };

    value.into_ts().into()
}

#[proc_macro]
pub fn def_props(input: TokenStream) -> TokenStream {
    use props_data::PropsDefinitionWithOptions;
    let value = parse_macro_input!(input with PropsDefinitionWithOptions::parse_proc_macro_input);
    let mut ts = proc_macro2::TokenStream::new();
    value.into_tokens(&mut ts);
    ts.into()
}

#[proc_macro]
pub fn __impl_auto_prepend_intrinsic_components(input: TokenStream) -> TokenStream {
    auto_intrinsic::auto_intrinsic(input)
}
