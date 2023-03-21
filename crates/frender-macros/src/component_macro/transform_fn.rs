use darling::ToTokens;
use hooks_macro_core::DetectedHooksTokens;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

use crate::{component_data::RenderCtx, err::RecordError};

pub fn extract_ctx_arg_and_state_impl_trait(
    item_fn: &mut syn::ItemFn,
    errors: &mut impl RecordError<darling::Error>,
) -> Option<(Box<syn::Pat>, syn::TypeImplTrait)> {
    let mut arg_ctx = None;
    let inputs = std::mem::take(&mut item_fn.sig.inputs);

    item_fn.sig.inputs = inputs
        .into_pairs()
        .filter_map(|i| {
            if let syn::FnArg::Typed(pt) = i.value() {
                match &*pt.ty {
                    syn::Type::Infer(_) => {
                        let pat;
                        let span;
                        if let syn::FnArg::Typed(pt) = i.into_value() {
                            pat = pt.pat;

                            if let syn::Type::Infer(ty) = &*pt.ty {
                                span = ty.underscore_token.span;
                            } else {
                                unreachable!()
                            }

                            report_context_arg_attrs(&pt.attrs, errors);
                        } else {
                            unreachable!()
                        };
                        arg_ctx = Some((
                            pat,
                            syn::TypeImplTrait {
                                impl_token: syn::Token![impl](span),
                                bounds: Default::default(),
                            },
                        ));

                        return None;
                    }
                    syn::Type::ImplTrait(_) => {
                        if let syn::FnArg::Typed(syn::PatType {
                            ref attrs,
                            pat,
                            colon_token,
                            ty,
                        }) = i.into_value()
                        {
                            if let syn::Type::ImplTrait(it) = *ty {
                                arg_ctx = Some((pat, it));
                                return None;
                            } else {
                                unreachable!()
                            }
                        } else {
                            unreachable!()
                        }
                    }
                    _ => {}
                }
            }
            Some(i)
        })
        .collect();
    arg_ctx
}

fn report_context_arg_attrs(
    attrs: &Vec<syn::Attribute>,
    errors: &mut impl RecordError<darling::Error>,
) {
    for attr in attrs {
        errors.record_error(
            darling::Error::custom("context argument cannot have attributes").with_span(attr),
        );
    }
}

#[inline]
pub fn transform_item_fn(
    item_fn: &mut syn::ItemFn,
    errors: &mut Vec<darling::Error>,
    hook_element_path: &syn::Path,
    render_ctx: RenderCtx,
    use_fn_once: darling::util::Flag,
) {
    transform_item_fn_with(
        item_fn,
        errors,
        hook_element_path,
        render_ctx,
        use_fn_once,
        |_, _| None,
    )
}

pub fn transform_item_fn_with(
    item_fn: &mut syn::ItemFn,
    errors: &mut Vec<darling::Error>,
    hook_element_path: &syn::Path,
    render_ctx: RenderCtx,
    use_fn_once: darling::util::Flag,
    before_stmts: impl FnOnce(
        &mut syn::ItemFn,
        &mut Vec<darling::Error>,
    ) -> Option<proc_macro2::TokenStream>,
) {
    let span = item_fn.sig.fn_token.span;

    let mut ctx_arg_pat = None;
    let mut state_it = None;
    if let Some((c, state_impl_trait)) = extract_ctx_arg_and_state_impl_trait(item_fn, errors) {
        ctx_arg_pat = Some(c);
        state_it = Some(state_impl_trait);
    };

    match &mut item_fn.sig.output {
        output @ syn::ReturnType::Default => {
            *output = syn::ReturnType::Type(
                syn::Token![->](span),
                Box::new(syn::Type::Verbatim(render_ctx.to_render_bounds(
                    span,
                    hook_element_path,
                    None,
                    state_it.as_ref(),
                ))),
            );
        }
        syn::ReturnType::Type(_, ty) => {
            let span;
            let mut bounds = None;

            match &**ty {
                syn::Type::Infer(ti) => {
                    span = ti.underscore_token.span;
                }
                syn::Type::ImplTrait(it) => {
                    span = it.impl_token.span;
                    if !it.bounds.is_empty() {
                        bounds = Some(it.bounds.to_token_stream());
                    }
                }
                ty => {
                    span = ty.span();
                    errors.record_error(
                        darling::Error::custom("component fn output must be _ or impl 'lifetime")
                            .with_span(&span),
                    );
                }
            }

            **ty = syn::Type::Verbatim(render_ctx.to_render_bounds(
                span,
                hook_element_path,
                bounds.as_ref(),
                state_it.as_ref(),
            ));
        }
    };

    // #hook_element_path::__private::hooks_core
    let hooks_core_path = {
        let span = hook_element_path.span();
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
        method_name = if ctx_arg_pat.is_some() {
            "FnOnce"
        } else {
            "FnOnceOutputElement"
        };
    } else {
        method_span = span;
        method_name = if ctx_arg_pat.is_some() {
            "FnMut"
        } else {
            "FnMutOutputElement"
        };
    };

    let mut method_generics = None;
    if ctx_arg_pat.is_none() {
        let ctx_ty = render_ctx.to_ctx_ty(span, hook_element_path);
        method_generics = Some(quote_spanned! {span=>
            ::<#ctx_ty, _>
        });
    };

    let method_name = proc_macro2::Ident::new(method_name, method_span);

    let prepend_stmt = before_stmts(item_fn, errors);

    item_fn
        .block
        .stmts
        .push(syn::Stmt::Expr(syn::Expr::Verbatim(quote_spanned! {span=>
            #hook_element_path::FnHookElement::#method_name #method_generics (
                move |#fn_arg_data_pat, #ctx_arg_pat| {

                    #fn_stmts_extract_data

                    #prepend_stmt

                    #(#stmts)*

                }
            )
        })));
}
