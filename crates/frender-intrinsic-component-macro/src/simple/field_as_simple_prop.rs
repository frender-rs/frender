use std::borrow::Cow;

use darling::ToTokens;
use frender_macro_utils::grouped::Grouped;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::{
    BoundsPath, Field, FieldDeclaration, FieldDeclarationBounds, FieldDeclarationEventListener,
    FieldDeclarationMaybeDetailsImpl, FieldDeclarationWithBounds,
    FieldDeclarationWithBoundsDetails, FieldDeclarationWithBoundsDetailsSimple,
    FieldDeclarationWithCommonBoundsCsrContent, FieldDeclarationWithCommonBoundsCsrFnCommonArgs,
    FieldDeclarationWithCommonBoundsCsrFnInitialize,
    FieldDeclarationWithCommonBoundsCsrFnInitializeArgs,
    FieldDeclarationWithCommonBoundsCsrFnUpdate, FieldDeclarationWithCommonBoundsCsrFnUpdateArgs,
    FieldDeclarationWithCommonBoundsSsrContent, FnIntoAttrs, ImplClosure, TypedPatType,
};

pub struct FieldToSimpleProp {
    pub imp: Option<TokenStream>,
    pub field_info: TokenStream,
}

pub struct FieldAsSimpleProp<'a> {
    pub crate_path: &'a TokenStream,
    pub field: &'a Field,
    pub dom_element_ty: &'a syn::Type,
}

impl FieldAsSimpleProp<'_> {
    pub fn into_simple_prop(self) -> FieldToSimpleProp {
        let Self {
            crate_path,
            field,
            dom_element_ty,
        } = self;
        let name = &field.name;

        match &field.declaration {
            FieldDeclaration::Maybe(m) => {
                assert!(m.no_cache.is_none(), "simply typed prop must have state");

                let value_ty = &m.ty;

                let field = {
                    let mut field = field.clone();

                    let mut attr_name = None;
                    let mut update =
                        quote!(#crate_path::impl_bounds::MaybeValue::csr::default_update);
                    let mut remove =
                        quote!(#crate_path::impl_bounds::MaybeValue::csr::default_remove);

                    if let Some(details) = &m.details {
                        let details = &details.content;
                        attr_name = details
                            .html_prop_name
                            .as_ref()
                            .map(|prop| quote!(attr_name = #prop,));

                        if let Some(html_element_method) = &details.html_element_method {
                            assert!(
                                details.impl_update.is_none(),
                                "html_element_method and custom update cannot be both specified"
                            );
                            let deref = if m.by_ref.is_some() {
                                None
                            } else {
                                Some(quote!(&))
                            };
                            update = quote!(
                                |el: &mut ET::#dom_element_ty<Renderer>, renderer: &mut _, _, #deref v: &_|
                                    el.#html_element_method(renderer, v)
                            );
                        }

                        if let Some(impl_update) = &details.impl_update {
                            let FieldDeclarationMaybeDetailsImpl {
                                keyword: _,
                                impl_closure:
                                    ImplClosure {
                                        or1_token,
                                        inputs,
                                        comma: _,
                                        or2_token,
                                        body,
                                    },
                            } = &impl_update.content;

                            let mut inputs = inputs.fields_to_map();

                            let element_pat = inputs.take_or_wild("element").pat;
                            let renderer_pat = inputs.take_or_wild("renderer").pat;
                            let attr_name_pat = inputs.take_or_wild("attr_name").pat;
                            let value_pat = inputs.take_or_wild("value").pat;
                            inputs.assert_empty().unwrap();

                            update = quote!(#or1_token
                                #element_pat: &mut ET::#dom_element_ty<Renderer>,
                                #renderer_pat: &mut Renderer,
                                #attr_name_pat: &_,
                                #value_pat: &_
                            #or2_token #body);
                        }

                        if let Some(impl_remove) = &details.impl_remove {
                            let FieldDeclarationMaybeDetailsImpl {
                                keyword: _,
                                impl_closure:
                                    ImplClosure {
                                        or1_token,
                                        inputs,
                                        comma: _,
                                        or2_token,
                                        body,
                                    },
                            } = &impl_remove.content;

                            let mut inputs = inputs.fields_to_map();

                            let element_pat = inputs.take_or_wild("element").pat;
                            let renderer_pat = inputs.take_or_wild("renderer").pat;
                            let attr_name_pat = inputs.take_or_wild("attr_name").pat;
                            inputs.assert_empty().unwrap();

                            remove = quote!(#or1_token
                                #element_pat: &mut ET::#dom_element_ty<Renderer>,
                                #renderer_pat: &mut Renderer,
                                #attr_name_pat: &_,
                            #or2_token #body);
                        }
                    }

                    field.declaration = FieldDeclaration::WithBounds(parse_quote!(
                        : bounds![
                            bounds as #crate_path::impl_bounds::MaybeValue<#value_ty>,
                            #attr_name
                            csr {
                                update: #update,
                                remove: #remove,
                            },
                        ]
                    ));

                    field
                };
                return FieldAsSimpleProp {
                    crate_path,
                    field: &field,
                    dom_element_ty,
                }
                .into_simple_prop();
            }
            FieldDeclaration::EventListener(FieldDeclarationEventListener {
                at_token: _,
                event_type,
                ty,
            }) => {
                let field = {
                    let mut field = field.clone();

                    field.declaration = FieldDeclaration::WithBounds(parse_quote!(
                        : bounds![
                            #[event(#crate_path::frender_html::html::event_type_helpers::#name)]
                            bounds as #crate_path::impl_bounds::MaybeHandleEvent,
                        ]
                    ));

                    field
                };
                return FieldAsSimpleProp {
                    crate_path,
                    field: &field,
                    dom_element_ty,
                }
                .into_simple_prop();
            }
            FieldDeclaration::Full(_) => {
                assert_eq!(
                    name, "children",
                    "In simply typed props, only children can have with full declaration"
                );
                return FieldToSimpleProp {
                    imp: None,
                    field_info: quote!(children,),
                };
            }
            FieldDeclaration::Inherit(f) => {
                let from_path = &f.from_path;
                return FieldToSimpleProp {
                    imp: None,
                    field_info: {
                        let fields = f
                            .fields
                            .iter()
                            .map(|field| FieldAsSimpleProp {
                                crate_path,
                                field,
                                dom_element_ty: &f.dom_element_ty,
                            })
                            .map(FieldAsSimpleProp::into_simple_prop)
                            .map(|p| p.field_info);
                        quote! {
                            ..#from_path (
                                #(#fields)*
                            ),
                        }
                    },
                };
            }
            FieldDeclaration::WithBounds(wb) => {
                let imp;
                let builder_bounds;

                match &wb.details {
                    FieldDeclarationWithBoundsDetails::Full(wb) => {
                        let wb = &wb.content;
                        let (bounds, where_clause) = wb
                            .common_bounds
                            .as_ref()
                            .map(|common_bounds| {
                                (
                                    Some(replace_self_type_with(
                                        common_bounds.bounds.clone(),
                                        "V",
                                        |visitor, bounds| {
                                            for bound in bounds.iter_mut() {
                                                syn::visit_mut::visit_type_param_bound_mut(
                                                    visitor, bound,
                                                )
                                            }
                                        },
                                    )),
                                    common_bounds
                                        .where_clause
                                        .clone()
                                        .map(replace_self_type_in_where_clause),
                                )
                            })
                            .unwrap_or_default();

                        fn replace_self_type_in_where_clause(
                            where_clause: syn::WhereClause,
                        ) -> syn::WhereClause {
                            replace_self_type_with(
                                where_clause,
                                "V",
                                syn::visit_mut::visit_where_clause_mut,
                            )
                        }

                        builder_bounds = bounds.to_token_stream();

                        let bounds = bounds.map(|bounds| quote!(: #bounds));

                        fn join_where_clause(
                            a: Option<syn::WhereClause>,
                            b: Option<syn::WhereClause>,
                        ) -> Option<syn::WhereClause> {
                            let predicates: syn::punctuated::Punctuated<
                                syn::WherePredicate,
                                syn:: Token![,],
                            > = [a, b]
                                .into_iter()
                                .filter_map(std::convert::identity)
                                .flat_map(|wc| wc.predicates)
                                .collect();

                            if predicates.is_empty() {
                                None
                            } else {
                                Some(syn::WhereClause {
                                    where_token: syn::Token![where](proc_macro2::Span::call_site()),
                                    predicates,
                                })
                            }
                        }

                        let csr_where_clause = join_where_clause(
                            where_clause.clone(),
                            wb.csr
                                .where_clause
                                .clone()
                                .map(replace_self_type_in_where_clause),
                        );

                        let ssr_where_clause = join_where_clause(
                            where_clause,
                            wb.ssr
                                .where_clause
                                .clone()
                                .map(replace_self_type_in_where_clause),
                        );

                        let FieldDeclarationWithCommonBoundsCsrContent {
                            fn_initialize:
                                FieldDeclarationWithCommonBoundsCsrFnInitialize {
                                    fn_token: _,
                                    ident: _,
                                    args: csr_initialize_args,
                                    arrow_token: _,
                                    ty_state: csr_state_ty,
                                    block: csr_block,
                                },
                            fn_update:
                                FieldDeclarationWithCommonBoundsCsrFnUpdate {
                                    fn_token: _,
                                    ident: _,
                                    args: csr_update_args,
                                    block: csr_update_block,
                                },
                        } = wb.csr_content();

                        let FieldDeclarationWithCommonBoundsCsrFnCommonArgs {
                            this:
                                TypedPatType {
                                    pat: csr_pat_this,
                                    colon_token: _,
                                    ty: _,
                                },
                            comma_1: _,
                            element: csr_pat_element,
                            comma_2: _,
                            children_ctx:
                                TypedPatType {
                                    pat: csr_pat_children_ctx,
                                    colon_token: _,
                                    ty: _,
                                },
                        } = &csr_initialize_args.content.args;

                        let FieldDeclarationWithCommonBoundsCsrFnUpdateArgs {
                            pre_args:
                                FieldDeclarationWithCommonBoundsCsrFnCommonArgs {
                                    this:
                                        TypedPatType {
                                            pat: csr_update_pat_this,
                                            colon_token: _,
                                            ty: _,
                                        },
                                    comma_1: _,
                                    element: csr_update_pat_element,
                                    comma_2: _,
                                    children_ctx: csr_update_pat_children_ctx,
                                },
                            comma_3: _,
                            state:
                                TypedPatType {
                                    pat: csr_update_pat_state,
                                    colon_token: _,
                                    ty: _,
                                },
                            comma_trailing: _,
                        } = &csr_update_args.content;

                        let FieldDeclarationWithCommonBoundsSsrContent {
                            fn_into_iter_attrs:
                                FnIntoAttrs {
                                    fn_token: _,
                                    ident: _,
                                    paren_token: _,
                                    inputs:
                                        TypedPatType {
                                            pat: ssr_pat_this,
                                            colon_token: _,
                                            ty: _,
                                        },
                                    arrow_token: _,
                                    ty_into_iter_attrs: ssr_attrs_ty,
                                    block: ssr_block,
                                },
                        } = wb.ssr_content();

                        imp = Some(quote! {
                            #[cfg(feature = "csr")]
                            impl<V #bounds, E: ::core::convert::AsRef<#dom_element_ty>>
                                #crate_path::frender_csr::props::UpdateElementNonReactive<E>
                            for super::props::#name<V>
                            #csr_where_clause {
                                type State = super::props::#name<#csr_state_ty>;

                                fn initialize_state_non_reactive(
                                    Self(#csr_pat_this): Self,
                                    element: &E,
                                    #csr_pat_children_ctx: &mut #crate_path::frender_csr::CsrContext,
                                ) -> Self::State {
                                    let #csr_pat_element = element.as_ref();
                                    super::props::#name(#csr_block)
                                }

                                fn update_element_non_reactive(
                                    #csr_update_pat_this: Self,
                                    element: &E,
                                    #csr_update_pat_children_ctx: &mut #crate_path::frender_csr::CsrContext,
                                    #csr_update_pat_state: ::core::pin::Pin<&mut Self::State>,
                                ) {
                                    let #csr_update_pat_element = element.as_ref();
                                    #csr_update_block
                                }
                            }

                            #[cfg(feature = "ssr")]
                            impl<V #bounds>
                                #crate_path::frender_html::IntoAsyncWritableAttrs
                            for super::props::#name<V>
                            #ssr_where_clause {
                                type AsyncWritableAttrs = #ssr_attrs_ty;
                                fn into_async_writable_attrs(Self(#ssr_pat_this): Self) -> Self::AsyncWritableAttrs
                                    #ssr_block
                            }
                        });
                    }
                    FieldDeclarationWithBoundsDetails::Simple(
                        FieldDeclarationWithBoundsDetailsSimple {
                            bounds:
                                bounds @ FieldDeclarationBounds {
                                    bounds_attrs,
                                    bounds_keyword,
                                    as_token: bounds_as,
                                    path:
                                        BoundsPath {
                                            mod_path,
                                            generic_args,
                                        },
                                },
                            attr_name,
                            imps,
                            trailing_comma,
                        },
                    ) => {
                        let attr_name = attr_name.as_ref().map_or_else(
                            || {
                                let attr_name = syn::LitStr::new(&name.to_string(), name.span());
                                quote!(, attr_name = #attr_name)
                            },
                            |(comma, attr_name, eq, value)| quote!(#comma #attr_name #eq #value),
                        );
                        let imps = imps.iter().map(
                            |(comma, imp_name, imp_fields)| quote!(#comma #imp_name #imp_fields),
                        );

                        builder_bounds = bounds.to_builder_bounds();
                        let bounds_as = bounds.to_bounds_as();

                        imp = Some(quote! {
                            #crate_path::impl_bounds!(super::props::#name(
                                #bounds_as,
                                element as #dom_element_ty
                                #attr_name
                                #(#imps)*
                                #trailing_comma
                            ));
                        });
                    }
                }

                return FieldToSimpleProp {
                    imp,
                    field_info: {
                        let alias = field.options.alias.to_alias_macro();

                        let bounds = quote! {
                            bounds![#builder_bounds]
                        };

                        let macros = syn::punctuated::Punctuated::<_, syn::Token![+]>::from_iter(
                            [alias, Some(bounds)]
                                .into_iter()
                                .filter_map(std::convert::identity),
                        );

                        quote! {
                            #name : #macros,
                        }
                    },
                };
            }
        }
    }
}

pub trait ReplaceIdent {
    fn replace_ident(&mut self, ident: &syn::Ident) -> syn::Ident;
}

impl<F: FnMut(&syn::Ident) -> syn::Ident> ReplaceIdent for F {
    fn replace_ident(&mut self, ident: &syn::Ident) -> syn::Ident {
        self(ident)
    }
}

impl ReplaceIdent for &str {
    fn replace_ident(&mut self, ident: &syn::Ident) -> syn::Ident {
        syn::Ident::new(self, ident.span())
    }
}

struct ReplaceSelfTypeWith<F: ReplaceIdent>(F);
impl<F: ReplaceIdent> syn::visit_mut::VisitMut for ReplaceSelfTypeWith<F> {
    fn visit_ident_mut(&mut self, ident: &mut proc_macro2::Ident) {
        if ident == "Self" {
            *ident = self.0.replace_ident(ident);
        } else {
            syn::visit_mut::visit_ident_mut(self, ident)
        }
    }
}

fn replace_self_type_with<T, R: ReplaceIdent>(
    mut ty: T,
    replace_with: R,
    visit_with: impl FnOnce(&mut ReplaceSelfTypeWith<R>, &mut T),
) -> T {
    visit_with(&mut ReplaceSelfTypeWith(replace_with), &mut ty);
    ty
}
