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
        let mut dom_pre = None;
        let dom_init;
        let dom_update;
        let mut ssr_bounds = None;
        let ssr_attrs_ty;
        let ssr_attrs_impl;

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
                dom_pre = Some(quote!(let element = dom_element;));

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

                ssr_attrs_impl = quote! {
                    V::maybe_into_html_attribute_value(
                        this.0
                    ).map(|attr_value| (::std::borrow::Cow::Borrowed(#attr_name), attr_value.map_or(
                            #crate_path::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            #crate_path::frender_ssr::element::html::HtmlAttributeValue::String,
                    ))).into_iter()
                };
            }
            FieldDeclaration::EventListener(e) => {
                //
                let ty = &e.ty;
                dom_bounds = Some(quote! {
                    : #crate_path::frender_html::props::UpdateDomEventListener<#ty>
                });
                dom_state_ty = {
                    quote! {
                        V::State
                    }
                };
                dom_init = quote! {
                    V::initialize_dom_event_listener_state(this.0, dom_element)
                };
                dom_update = quote! {
                    V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
                };

                ssr_attrs_ty = quote! {
                    ::core::iter::Empty<
                        #crate_path::frender_ssr::element::html::HtmlAttrPair<'a>
                    >
                };

                ssr_attrs_impl = quote! {
                    ::core::iter::empty()
                };
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
        }

        FieldToSimpleProp {
            impl_dom: Some(quote! {
                impl<V #dom_bounds, E: ::core::convert::AsRef<#dom_element_ty>>
                    #crate_path::frender_dom::props::UpdateElementNonReactive<E>
                for super::props::#name<V> {
                    type State = super::props::#name<#dom_state_ty>;

                    fn initialize_state_non_reactive(
                        this: Self,
                        element: &E,
                        children_ctx: &mut #crate_path::frender_dom::Dom,
                    ) -> Self::State {
                        let dom_element = element.as_ref();
                        #dom_pre
                        super::props::#name(
                            #dom_init
                        )
                    }

                    fn update_element_non_reactive(
                        this: Self,
                        element: &E,
                        children_ctx: &mut #crate_path::frender_dom::Dom,
                        state: ::core::pin::Pin<&mut Self::State>,
                    ) {
                        let dom_element = element.as_ref();
                        #dom_pre
                        #dom_update
                    }
                }
            }),
            impl_ssr: Some(quote! {
                impl<'a, V #ssr_bounds>
                    #crate_path::frender_ssr::attrs::IntoIteratorAttrs<'a>
                for super::props::#name<V> {
                    type IntoIterAttrs = #ssr_attrs_ty;
                    fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
                        #ssr_attrs_impl
                    }
                }
            }),
            field_info: {
                let bounds = builder_fn_bounds.map(|b| quote!(: bounds![#b]));
                quote! {
                    #name #bounds,
                }
            },
        }
    }
}
