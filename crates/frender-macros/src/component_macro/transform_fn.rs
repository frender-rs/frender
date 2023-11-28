use darling::ToTokens;
use hooks_macro_core::DetectedHooksTokens;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

use crate::err::RecordError;

fn element_return_ty(
    span: proc_macro2::Span,
    hook_element_path: &syn::Path,
    ssr_only: darling::util::Flag,
) -> proc_macro2::TokenStream {
    let ssr_only_path;
    let element_path;
    if ssr_only.is_present() {
        ssr_only_path = Some(quote_spanned!(ssr_only.span()=> ::ssr_only));
        element_path = quote_spanned!(span => ::SsrElement);
    } else {
        ssr_only_path = None;
        element_path = quote_spanned!(span => ::Element);
    }
    quote_spanned!(span =>
        impl
        #hook_element_path
        #ssr_only_path
        #element_path
    )
}

#[inline]
pub fn transform_item_fn(
    item_fn: &mut syn::ItemFn,
    errors: &mut Vec<darling::Error>,
    hook_element_path: &syn::Path,
    ssr_only: darling::util::Flag,
    use_fn_once: darling::util::Flag,
) {
    transform_item_fn_with(
        item_fn,
        errors,
        hook_element_path,
        ssr_only,
        use_fn_once,
        |_, _| None,
    )
}

pub fn transform_item_fn_with(
    item_fn: &mut syn::ItemFn,
    errors: &mut Vec<darling::Error>,
    hook_element_path: &syn::Path,
    ssr_only: darling::util::Flag,
    use_fn_once: darling::util::Flag,
    before_stmts: impl FnOnce(
        &mut syn::ItemFn,
        &mut Vec<darling::Error>,
    ) -> Option<proc_macro2::TokenStream>,
) {
    let span = item_fn.sig.fn_token.span;

    match &mut item_fn.sig.output {
        output @ syn::ReturnType::Default => {
            *output = syn::ReturnType::Type(
                syn::Token![->](span),
                Box::new(syn::Type::Verbatim(element_return_ty(
                    span,
                    hook_element_path,
                    ssr_only,
                ))),
            );
        }
        syn::ReturnType::Type(r_arrow, ty) => {
            let span;

            match &**ty {
                syn::Type::Infer(ti) => {
                    span = ti.underscore_token.span;
                }
                syn::Type::ImplTrait(it) => {
                    span = it.impl_token.span;
                }
                ty => {
                    span = r_arrow.span();
                }
            }

            **ty = syn::Type::Verbatim(element_return_ty(span, hook_element_path, ssr_only));
        }
    };

    // #hook_element_path::__private::hooks_core
    let hooks_core_path = {
        let span = proc_macro2::Span::mixed_site().located_at(hook_element_path.span());
        let mut p = hook_element_path.clone();
        p.segments.extend(
            ["__private", "hooks_core"]
                .map(|ident| syn::PathSegment::from(syn::Ident::new(ident, span))),
        );
        p
    };

    let mut stmts = std::mem::replace(&mut item_fn.block.stmts, Vec::with_capacity(1));

    let detected_hooks = hooks_macro_core::detect_hooks(stmts.iter_mut(), &hooks_core_path);
    let DetectedHooksTokens {
        fn_arg_data_pat,
        fn_stmts_extract_data,
    } = hooks_macro_core::detected_hooks_to_tokens(detected_hooks.hooks, hooks_core_path, span);
    // TODO: link #[not_hook]

    let method_name;
    let method_span;
    if use_fn_once.is_present() {
        method_span = use_fn_once.span();
        method_name = "FnOnceOutputElement";
    } else {
        method_span = span;
        method_name = "new_fn_hook_element";
    };

    let method_name = proc_macro2::Ident::new(method_name, method_span);

    let prepend_stmt = before_stmts(item_fn, errors);

    item_fn
        .block
        .stmts
        .push(syn::Stmt::Expr(syn::Expr::Verbatim(quote_spanned! {span=>
            #hook_element_path::#method_name (
                move |#fn_arg_data_pat| {

                    #fn_stmts_extract_data

                    #prepend_stmt

                    #(#stmts)*

                }
            )
        })));
}
