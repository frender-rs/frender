use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn component(args: TokenStream, input: TokenStream) -> TokenStream {
    use darling::ast::NestedMeta;

    use frender_macro_core::component::*;

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };

    let item_fn = parse_macro_input!(input as syn::ItemFn);

    let comp = ComponentDefinition::from_attrs_and_fn(&attr_args, item_fn);

    comp.into_ts().into()
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    use frender_macro_core::rsx;

    let value = match syn::parse::<rsx::OptionalCratePathAndRsxChild>(input) {
        Ok(v) => v,
        Err(err) => {
            return proc_macro::TokenTree::Group(proc_macro::Group::new(
                proc_macro::Delimiter::Brace,
                err.to_compile_error().into(),
            ))
            .into()
        }
    };

    value.map(rsx::RsxChild::into_ts).into()
}
