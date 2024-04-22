use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;

use crate::{
    component_macro::{transform_item_fn, MainItem},
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
                    ssr_only,
                    frender_path,
                    use_fn_once,
                },
            mut item_fn,
        } = self;

        // let span = item_fn.sig.fn_token.span;

        let frender_path = frender_path.unwrap_or_else(default_frender_path);

        let main_block = main.map(|main| {
            let span = main.original.path().span();
            let expr_element = {
                let name = &item_fn.sig.ident;
                let span = Span::call_site().located_at(name.span());
                quote_spanned!(span => #name() )
            };

            MainItem {
                span_default: span,
                span_fn_ident: span,
                frender_path: &frender_path,
                options: &main.parsed,
                vis: &item_fn.vis,
                expr_element,
            }
            .into_ts()
        });

        let mut tokens = {
            transform_item_fn(
                &mut item_fn,
                &mut errors,
                &frender_path,
                ssr_only,
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

/// `::frender`
fn default_frender_path() -> syn::Path {
    let span = proc_macro2::Span::call_site();
    syn::Path {
        leading_colon: Some(Default::default()),
        segments: FromIterator::from_iter([
            //
            syn::PathSegment::from(syn::Ident::new("frender", span)),
        ]),
    }
}
