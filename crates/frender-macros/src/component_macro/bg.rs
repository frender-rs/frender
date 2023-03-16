use darling::ToTokens;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_quote_spanned, spanned::Spanned};

use crate::{component_data::RenderCtx, err::RecordError};

use super::transform_item_fn_with;

pub struct ItemFnToBg<'a> {
    pub span_default: Span,
    pub span_bg: Span,
    pub errors: &'a mut Vec<darling::Error>,
    pub hook_element_path: &'a syn::Path,
    pub bg_path: &'a TokenStream,
    pub item_fn: syn::ItemFn,
    pub render_ctx: RenderCtx,
    pub use_fn_once: darling::util::Flag,
}

fn before_stmts(
    item_fn: &mut syn::ItemFn,
    errors: &mut Vec<darling::Error>,
    span_bg: Span,
    hook_element_path: &impl ToTokens,
    bg_path: impl ToTokens,
    out_props_path: &mut TokenStream,
) -> Option<TokenStream> {
    let sig = &mut item_fn.sig;
    let name = &sig.ident;

    let mut props_stmt = None;

    if sig.inputs.len() > 1 {
        let ignored_inputs = sig.inputs.pairs().skip(1);
        let ignored_inputs = quote!(#(#ignored_inputs)*);
        errors.record_error(
            darling::Error::custom(
                "component with bg pattern can have at most one argument as props",
            )
            .with_span(&ignored_inputs),
        );
    }

    if sig.inputs.is_empty() {
        sig.inputs.push(
            syn::FnArg::Typed(default_props_pat_type(
                vec![],
                span_bg,
                span_bg,
                hook_element_path,
            ))
            .into(),
        )
    }

    let props_input = sig.inputs.first_mut().unwrap();
    let pt = match props_input {
        syn::FnArg::Receiver(r) => {
            let span = r.self_token.span;

            errors.record_error(
                darling::Error::custom("component with bg pattern cannot have self arguments")
                    .with_span(&r.self_token),
            );

            let attrs = std::mem::take(&mut r.attrs);

            *props_input = syn::FnArg::Typed(default_props_pat_type(
                attrs,
                span,
                span_bg,
                hook_element_path,
            ));

            match props_input {
                syn::FnArg::Typed(pt) => pt,
                syn::FnArg::Receiver(_) => unreachable!(),
            }
        }
        syn::FnArg::Typed(pt) => {
            match &mut *pt.pat {
                syn::Pat::Ident(syn::PatIdent {
                    by_ref: None,
                    mutability: None,
                    subpat: None,
                    ..
                })
                | syn::Pat::Wild(_) => {}
                pat => {
                    let name = syn::Ident::new("__frender_bg_props", span_bg);
                    let pat = std::mem::replace(pat, syn::Pat::Verbatim(name.to_token_stream()));

                    props_stmt = Some(quote_spanned! {span_bg=>
                        let #pat = #name;
                    });
                }
            }

            pt
        }
    };

    let props_path = resolve_props_path_from_ty(&pt.ty, errors)
        .unwrap_or_else(|| default_props_path(span_bg, bg_path));
    *pt.ty = syn::Type::Verbatim(quote_spanned!(props_path.span()=>
        #props_path ::Data<TypesDef>
    ));

    sig.generics.params.push(parse_quote_spanned! {span_bg=>
        TypesDef: ?::core::marker::Sized + #props_path::ValidTypes
    });

    *out_props_path = props_path;

    props_stmt
}

impl ItemFnToBg<'_> {
    pub fn into_ts(self) -> TokenStream {
        let Self {
            span_default,
            span_bg,
            errors,
            hook_element_path,
            bg_path,
            mut item_fn,
            render_ctx,
            use_fn_once,
        } = self;

        let vis = std::mem::replace(&mut item_fn.vis, syn::Visibility::Inherited);
        let mut outer_attrs = Vec::with_capacity(item_fn.attrs.len());
        let mut inner_attrs = vec![];

        for attr in std::mem::take(&mut item_fn.attrs).into_iter() {
            match attr.style {
                syn::AttrStyle::Outer => &mut outer_attrs,
                syn::AttrStyle::Inner(_) => &mut inner_attrs,
            }
            .push(attr)
        }

        item_fn.attrs = inner_attrs;

        let mut props_path = TokenStream::new();

        transform_item_fn_with(
            &mut item_fn,
            errors,
            hook_element_path,
            render_ctx,
            use_fn_once,
            |item_fn, errors| {
                before_stmts(
                    item_fn,
                    errors,
                    span_bg,
                    hook_element_path,
                    bg_path,
                    &mut props_path,
                )
            },
        );

        let output_ty = &item_fn.sig.output;
        let name = &item_fn.sig.ident;

        quote_spanned! {span_default =>
            impl self::#name::FrenderImpl {
                #[allow(non_snake_case)]
                #item_fn
            }

            #bg_path::builder! {
                #(#outer_attrs)*
                #vis struct #name(#props_path);

                pub(super) enum FrenderImpl {}

                mod frender_build_element {
                    #[allow(unused_imports)]
                    use super::super::*;

                    #[inline]
                    pub fn build_element<TypesDef: ?::core::marker::Sized + super::ValidTypes>(
                        super::Building(props): super::Building<TypesDef>,
                    ) #output_ty {
                        super::FrenderImpl::#name (props)
                    }
                }

                pub use frender_build_element::build_element;
            }
        }
    }
}

fn resolve_props_path_from_ty(
    ty: &syn::Type,
    errors: &mut impl RecordError<darling::Error>,
) -> Option<TokenStream> {
    match ty {
        syn::Type::Group(ty) => resolve_props_path_from_ty(&ty.elem, errors),
        syn::Type::Paren(ty) => resolve_props_path_from_ty(&ty.elem, errors),
        syn::Type::Path(path) => Some(path.to_token_stream()),
        syn::Type::Reference(tr) => {
            errors.record_error(
                darling::Error::custom(
                    "props type of component with bg pattern must be a simple path",
                )
                .with_span(&tr.and_token.span),
            );

            resolve_props_path_from_ty(&tr.elem, errors)
        }
        syn::Type::Verbatim(ty) => Some(ty.to_token_stream()),
        _ => None,
    }
}

fn default_props_pat_type(
    attrs: Vec<syn::Attribute>,
    span: Span,
    span_bg: Span,
    bg_path: impl ToTokens,
) -> syn::PatType {
    syn::PatType {
        attrs,
        pat: Box::new(syn::Pat::Wild(syn::PatWild {
            attrs: vec![],
            underscore_token: syn::Token![_](span),
        })),
        colon_token: syn::Token![:](span),
        ty: Box::new(syn::Type::Verbatim(default_props_path(span_bg, bg_path))),
    }
}

fn default_props_path(span_bg: Span, bg_path: impl ToTokens) -> TokenStream {
    quote_spanned!(span_bg=>
        #bg_path::Empty
    )
}
