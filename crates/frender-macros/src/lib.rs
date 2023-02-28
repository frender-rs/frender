mod auto_intrinsic;
mod component_data;
mod component_macro;
mod component_to_tokens;
mod err;
mod hook_component;
mod props_data;
mod props_to_tokens;
mod rsx_data;
mod rsx_to_tokens;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use frender_macro_utils as utils;

#[proc_macro_attribute]
pub fn component(args: TokenStream, input: TokenStream) -> TokenStream {
    use component_data::*;
    let attr_args = parse_macro_input!(args as syn::AttributeArgs);
    let item_fn = parse_macro_input!(input as syn::ItemFn);

    let comp = ComponentDefinition::from_attrs_and_fn(attr_args, item_fn);

    comp.into_ts().into()
}

#[proc_macro]
pub fn detect_hooks(input: TokenStream) -> TokenStream {
    let value: utils::prefix_path::PrefixPath<hook_component::DetectHooks> =
        parse_macro_input!(input);

    value
        .rest
        .into_ts(value.path.map_or_else(
            || quote::quote!(::hooks::core),
            |p| p.bracketed_path.content,
        ))
        .into()
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

    value.map(rsx_data::RsxChild::into_ts).into()
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

#[cfg(feature = "intrinsic-component")]
#[proc_macro]
pub fn def_intrinsic_component_props(input: TokenStream) -> TokenStream {
    frender_intrinsic_component_macro::into_ts(parse_macro_input!(input)).into()
}
