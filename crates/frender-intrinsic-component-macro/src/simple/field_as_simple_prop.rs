use std::borrow::Cow;

use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::{
    BoundsPath, Field, FieldDeclaration, FieldDeclarationEventListener,
    FieldDeclarationMaybeDetailsImpl, FieldDeclarationWithBounds,
    FieldDeclarationWithBoundsDetails, FieldDeclarationWithBoundsDetailsSimple,
};

pub struct FieldToSimpleProp {
    pub imp: Option<TokenStream>,
    pub impl_dom: Option<TokenStream>,
    pub impl_ssr: Option<TokenStream>,
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

        let mut builder_fn_bounds = None;
        let mut dom_bounds = None;
        let dom_state_ty;
        let dom_init;
        let dom_update;

        let mut csr_initialize_pat_this = quote!(this);
        let mut csr_initialize_pat_children_ctx = quote!(children_ctx);
        let mut csr_initialize_pat_dom_element = quote!(dom_element);

        let mut csr_update_pat_this = quote!(this);
        let mut csr_update_pat_children_ctx = quote!(children_ctx);
        let mut csr_update_pat_state = quote!(state);
        let mut csr_update_pat_dom_element = quote!(dom_element);

        let mut ssr_bounds = None;
        let mut ssr_attrs_args = quote!(this: Self);
        let ssr_attrs_ty;
        let ssr_attrs_block;

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
                            update = quote!(|el: &#dom_element_ty, _, #deref v: &_| el.#html_element_method(v));
                        }

                        if let Some(impl_update) = &details.impl_update {
                            let FieldDeclarationMaybeDetailsImpl {
                                keyword: _,
                                element_pat_ident,
                                rest,
                            } = &impl_update.content;
                            update =
                                quote!(|#element_pat_ident: &#dom_element_ty, _, v: &_| (#rest)(v));
                        }

                        if let Some(impl_remove) = &details.impl_remove {
                            let FieldDeclarationMaybeDetailsImpl {
                                keyword: _,
                                element_pat_ident,
                                rest,
                            } = &impl_remove.content;
                            remove = quote!(|#element_pat_ident: &#dom_element_ty, _| (#rest)());
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
                            bounds as #crate_path::impl_bounds::MaybeHandleEvent<#ty>,
                            attr_name = #event_type,
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
                    impl_dom: None,
                    impl_ssr: None,
                    field_info: quote!(children,),
                };
            }
            FieldDeclaration::Inherit(f) => {
                let from_path = &f.from_path;
                return FieldToSimpleProp {
                    imp: None,
                    impl_dom: None,
                    impl_ssr: None,
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
                let wb = match &wb.details {
                    FieldDeclarationWithBoundsDetails::Full(wb) => &wb.content,
                    FieldDeclarationWithBoundsDetails::Simple(
                        FieldDeclarationWithBoundsDetailsSimple {
                            bounds:
                                (
                                    bounds,
                                    bounds_as,
                                    BoundsPath {
                                        mod_path,
                                        generic_args,
                                    },
                                ),
                            attr_name,
                            imps,
                            trailing_comma,
                        },
                    ) => {
                        // TODO: remove
                        #[cfg(todo_remove)]
                        let generic_args = generic_args.as_ref().map(|generic_args| {
                            if generic_args.colon2_token.is_none() {
                                let mut generic_args = generic_args.clone();
                                generic_args.colon2_token = Some(Default::default());
                                Cow::Owned(generic_args)
                            } else {
                                Cow::Borrowed(generic_args)
                            }
                        });
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
                        return FieldToSimpleProp {
                            imp: Some(quote! {
                                #crate_path::impl_bounds!(super::props::#name(
                                    #bounds #bounds_as #mod_path #generic_args,
                                    element as #dom_element_ty
                                    #attr_name
                                    #(#imps)*
                                    #trailing_comma
                                ));
                            }),
                            impl_dom: None,
                            impl_ssr: None,
                            field_info: {
                                let alias = if field.options.alias.0.is_empty() {
                                    None
                                } else {
                                    let alias = field.options.alias.0.iter().enumerate().map(
                                        |(i, ident)| {
                                            let comma = if i == 0 { None } else { Some(quote!(,)) };
                                            quote!(#comma #ident)
                                        },
                                    );
                                    Some(quote!(alias![#(#alias)*] +))
                                };

                                let bounds = quote! {
                                    bounds![#mod_path::Bounds #generic_args]
                                };

                                quote! {
                                    #name : #alias #bounds,
                                }
                            },
                        };
                    }
                };

                builder_fn_bounds = wb
                    .common_bounds
                    .as_ref()
                    .map(|common_bounds| common_bounds.bounds.to_token_stream());

                dom_bounds = Some(quote!(: #builder_fn_bounds));
                ssr_bounds = Some(Cow::Borrowed(dom_bounds.as_ref().unwrap()));

                // csr
                {
                    let crate::FieldDeclarationWithCommonBoundsCsrContent {
                        fn_initialize,
                        fn_update,
                    } = wb.csr_content();
                    {
                        dom_state_ty = replace_self_type_with(
                            fn_initialize.ty_state.clone(),
                            "V",
                            syn::visit_mut::visit_type_mut,
                        )
                        .into_token_stream();

                        {
                            let args = &fn_initialize.args.content.args;
                            csr_initialize_pat_this = {
                                let pat = &args.this.pat;
                                quote!(Self(#pat))
                            };
                            csr_initialize_pat_children_ctx =
                                args.children_ctx.pat.to_token_stream();
                            csr_initialize_pat_dom_element = args.element.pat.to_token_stream();
                        }

                        dom_init = replace_self_type_with(
                            fn_initialize.block.clone(),
                            "V",
                            syn::visit_mut::visit_block_mut,
                        )
                        .to_token_stream();
                    }
                    {
                        {
                            let args = &fn_update.args.content;

                            csr_update_pat_this = {
                                let pat = &args.pre_args.this.pat;
                                quote!(Self(#pat))
                            };
                            csr_update_pat_children_ctx =
                                args.pre_args.children_ctx.pat.to_token_stream();
                            csr_update_pat_state = args.state.pat.to_token_stream();
                            csr_update_pat_dom_element =
                                args.pre_args.element.pat.to_token_stream();
                        }
                        dom_update = replace_self_type_with(
                            fn_update.block.clone(),
                            "V",
                            syn::visit_mut::visit_block_mut,
                        )
                        .to_token_stream();
                    }
                }

                // ssr
                {
                    let crate::FnIntoAttrs {
                        inputs,
                        ty_into_iter_attrs,
                        block,
                        ..
                    } = &wb.ssr_content().fn_into_iter_attrs;

                    ssr_attrs_args = {
                        let pat = &inputs.pat;
                        quote!(Self(#pat): Self)
                    };
                    ssr_attrs_ty = replace_self_type_with(
                        ty_into_iter_attrs.clone(),
                        "V",
                        syn::visit_mut::visit_type_mut,
                    )
                    .into_token_stream();
                    ssr_attrs_block =
                        replace_self_type_with(block.clone(), "V", syn::visit_mut::visit_block_mut)
                            .into_token_stream();
                }
            }
        }

        FieldToSimpleProp {
            imp: None,
            impl_dom: Some(quote! {
                impl<V #dom_bounds, E: ::core::convert::AsRef<#dom_element_ty>>
                    #crate_path::frender_csr::props::UpdateElementNonReactive<E>
                for super::props::#name<V> {
                    type State = super::props::#name<#dom_state_ty>;

                    fn initialize_state_non_reactive(
                        #csr_initialize_pat_this: Self,
                        element: &E,
                        #csr_initialize_pat_children_ctx: &mut #crate_path::frender_csr::CsrContext,
                    ) -> Self::State {
                        let #csr_initialize_pat_dom_element = element.as_ref();
                        super::props::#name(
                            #dom_init
                        )
                    }

                    fn update_element_non_reactive(
                        #csr_update_pat_this: Self,
                        element: &E,
                        #csr_update_pat_children_ctx: &mut #crate_path::frender_csr::CsrContext,
                        #csr_update_pat_state: ::core::pin::Pin<&mut Self::State>,
                    ) {
                        let #csr_update_pat_dom_element = element.as_ref();
                        #dom_update
                    }
                }
            }),
            impl_ssr: Some(quote! {
                impl<V #ssr_bounds>
                    #crate_path::frender_html::IntoAsyncWritableAttrs
                for super::props::#name<V> {
                    type AsyncWritableAttrs = #ssr_attrs_ty;
                    fn into_async_writable_attrs(#ssr_attrs_args) -> Self::AsyncWritableAttrs
                        #ssr_attrs_block
                }
            }),
            field_info: {
                let data = syn::punctuated::Punctuated::<TokenStream, syn::Token![+]>::from_iter(
                    [
                        if field.options.alias.0.is_empty() {
                            None
                        } else {
                            let alias =
                                field.options.alias.0.iter().enumerate().map(|(i, ident)| {
                                    let comma = if i == 0 { None } else { Some(quote!(,)) };
                                    quote!(#comma #ident)
                                });
                            Some(quote!(alias![#(#alias)*]))
                        },
                        builder_fn_bounds.map(|b| quote!(bounds![#b])),
                    ]
                    .into_iter()
                    .filter_map(std::convert::identity),
                );

                let colon = if data.is_empty() {
                    None
                } else {
                    Some(quote!(:))
                };

                quote! {
                    #name #colon #data,
                }
            },
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
