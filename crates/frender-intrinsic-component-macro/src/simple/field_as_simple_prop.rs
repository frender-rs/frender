use std::borrow::Cow;

use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Field, FieldDeclaration};

pub struct FieldToSimpleProp {
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
        let mut csr_initialize_pat_element = quote!(element);
        let mut csr_initialize_pat_children_ctx = quote!(children_ctx);
        let mut csr_initialize_pat_dom_element = quote!(dom_element);

        let mut csr_update_pat_this = quote!(this);
        let csr_update_pat_element = quote!(element);
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
                let ty = &m.ty;
                builder_fn_bounds = Some(quote! {
                    #crate_path::frender_html::props::MaybeUpdateValueWithState<#ty>
                });
                dom_bounds = Some(quote!(: #builder_fn_bounds));
                ssr_bounds = Some(Cow::Borrowed(dom_bounds.as_ref().unwrap()));
                dom_state_ty = {
                    // let
                    quote! {
                        V::State
                    }
                };
                csr_initialize_pat_dom_element = quote!(element);
                csr_update_pat_dom_element = quote!(element);

                dom_init = m
                    .to_ts_dom_initialize_state_custom(crate_path, name, quote!(V), quote!(this.0))
                    .unwrap();

                dom_update = m.to_ts_update_element_custom(
                    crate_path,
                    name,
                    quote!(V),
                    quote!(this.0),
                    quote!(&mut state.get_mut().0),
                );

                ssr_attrs_ty = quote! {
                    ::core::option::IntoIter<
                        #crate_path::frender_ssr::element::html::HtmlAttrPair<'a>
                    >
                };

                let attr_name = m.to_html_prop_name(name);

                ssr_attrs_block = quote! {{
                    V::maybe_into_html_attribute_value(
                        this.0
                    ).map(|attr_value| (::std::borrow::Cow::Borrowed(#attr_name), attr_value.map_or(
                            #crate_path::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            #crate_path::frender_ssr::element::html::HtmlAttributeValue::String,
                    ))).into_iter()
                }};
            }
            FieldDeclaration::EventListener(e) => {
                //
                let ty = &e.ty;
                let event_type = &e.event_type;
                builder_fn_bounds = Some(quote! {
                    #crate_path::frender_events::MaybeHandleEvent<#ty>
                });
                dom_bounds = Some(quote!(: #builder_fn_bounds));
                ssr_bounds = Some(Cow::Borrowed(dom_bounds.as_ref().unwrap()));

                dom_state_ty = {
                    quote! {
                        V::State
                    }
                };
                dom_init = quote! {
                    V::initialize_handle_event_state(
                        this.0,
                        |callable| #crate_path::frender_events::EventListener::new(
                            dom_element,
                            #event_type,
                            callable.clone(),
                        ),
                    )
                };
                dom_update = quote! {
                    V::update_handle_event_state(
                        this.0,
                        &mut state.get_mut().0,
                        |callable| #crate_path::frender_events::EventListener::new(
                            dom_element,
                            #event_type,
                            callable.clone(),
                        ),
                    )
                };

                ssr_attrs_ty = quote! {
                    ::core::iter::Empty<
                        #crate_path::frender_ssr::element::html::HtmlAttrPair<'a>
                    >
                };

                ssr_attrs_block = quote! {{
                    ::core::iter::empty()
                }};
            }
            FieldDeclaration::Full(_) => {
                assert_eq!(
                    name, "children",
                    "In simply typed props, only children can have with full declaration"
                );
                return FieldToSimpleProp {
                    impl_dom: None,
                    impl_ssr: None,
                    field_info: quote!(children,),
                };
            }
            FieldDeclaration::Inherit(f) => {
                let from_path = &f.from_path;
                return FieldToSimpleProp {
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
                let wb = &wb.details.content;
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
                        dom_state_ty = fn_initialize.ty_state.to_token_stream();

                        {
                            let args = &fn_initialize.args.content.args;
                            csr_initialize_pat_this = args.this.pat.to_token_stream();
                            csr_initialize_pat_children_ctx =
                                args.children_ctx.pat.to_token_stream();
                            csr_initialize_pat_dom_element = args.element.pat.to_token_stream();
                        }

                        dom_init = fn_initialize.block.to_token_stream();
                    }
                    {
                        {
                            let args = &fn_update.args.content;

                            csr_update_pat_this = args.pre_args.this.pat.to_token_stream();
                            csr_update_pat_children_ctx =
                                args.pre_args.children_ctx.pat.to_token_stream();
                            csr_update_pat_state = args.state.pat.to_token_stream();
                            csr_update_pat_dom_element =
                                args.pre_args.element.pat.to_token_stream();
                        }
                        dom_update = fn_update.block.to_token_stream();
                    }
                }

                // ssr
                {
                    let crate::FnIntoIterAttrs {
                        inputs,
                        ty_into_iter_attrs,
                        block,
                        ..
                    } = &wb.ssr_content().fn_into_iter_attrs;

                    ssr_attrs_args = inputs.to_token_stream();
                    ssr_attrs_ty = ty_into_iter_attrs.to_token_stream();
                    ssr_attrs_block = block.to_token_stream();
                }
            }
        }

        FieldToSimpleProp {
            impl_dom: Some(quote! {
                impl<V #dom_bounds, E: ::core::convert::AsRef<#dom_element_ty>>
                    #crate_path::frender_csr::props::UpdateElementNonReactive<E>
                for super::props::#name<V> {
                    type State = super::props::#name<#dom_state_ty>;

                    fn initialize_state_non_reactive(
                        #csr_initialize_pat_this: Self,
                        #csr_initialize_pat_element: &E,
                        #csr_initialize_pat_children_ctx: &mut #crate_path::frender_csr::CsrContext,
                    ) -> Self::State {
                        let #csr_initialize_pat_dom_element = element.as_ref();
                        super::props::#name(
                            #dom_init
                        )
                    }

                    fn update_element_non_reactive(
                        #csr_update_pat_this: Self,
                        #csr_update_pat_element: &E,
                        #csr_update_pat_children_ctx: &mut #crate_path::frender_csr::CsrContext,
                        #csr_update_pat_state: ::core::pin::Pin<&mut Self::State>,
                    ) {
                        let #csr_update_pat_dom_element = element.as_ref();
                        #dom_update
                    }
                }
            }),
            impl_ssr: Some(quote! {
                impl<'a, V #ssr_bounds>
                    #crate_path::frender_ssr::attrs::IntoIteratorAttrs<'a>
                for super::props::#name<V> {
                    type IntoIterAttrs = #ssr_attrs_ty;
                    fn into_iter_attrs(#ssr_attrs_args) -> Self::IntoIterAttrs {
                        #ssr_attrs_block
                    }
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
