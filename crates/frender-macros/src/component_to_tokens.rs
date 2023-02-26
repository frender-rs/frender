use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

use crate::{
    component_macro::{transform_item_fn, ItemFnToBg, MainItem},
    err::OutputError,
};

use super::component_data::*;

impl ComponentDefinition {
    pub fn into_ts(self) -> TokenStream {
        let Self {
            mut errors,
            options:
                ComponentOptions {
                    //
                    main,
                    render_ctx,
                    hook_element_path,
                    use_fn_once,
                    bg,
                },
            mut item_fn,
        } = self;

        // let span = item_fn.sig.fn_token.span;

        let hook_element_path = hook_element_path
            .map_or_else(default_hook_element_path, |path| path.into_token_stream());

        let main_block = main.map(|main| {
            let span = main.original.path().span();
            MainItem {
                span_default: span,
                span_fn_ident: span,
                hook_element_path: &hook_element_path,
                options: &main.parsed,
                vis: &item_fn.vis,
                expr_get_element: if bg.is_present() {
                    todo!("bg")
                } else {
                    &item_fn.sig.ident
                },
            }
            .into_ts()
        });

        let mut tokens = if bg.is_present() {
            ItemFnToBg {
                span_default: item_fn.sig.fn_token.span,
                span_bg: bg.span(),
                errors: &mut errors,
                hook_element_path: &hook_element_path,
                item_fn,
                render_ctx,
                use_fn_once,
            }
            .into_ts()
        } else {
            transform_item_fn(
                &mut item_fn,
                &mut errors,
                &hook_element_path,
                render_ctx,
                use_fn_once,
            );

            quote!(
                #[allow(non_snake_case)]
                #item_fn
            )
        };

        let errors = errors.output_error().map(darling::Error::write_errors);

        main_block.to_tokens(&mut tokens);
        errors.to_tokens(&mut tokens);

        tokens
    }
}

/// `::frender::hook_element`
fn default_hook_element_path() -> TokenStream {
    quote!(::frender::hook_element)
}
