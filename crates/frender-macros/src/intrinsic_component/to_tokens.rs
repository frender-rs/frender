use std::borrow::Cow;

use proc_macro2::{Group, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse_quote;

use crate::utils::grouped::{Braced, Bracketed};

use super::{
    kw, Field, FieldDeclaration, FieldDeclarationEventListener, FieldDeclarationInherit,
    FieldDeclarationMaybe, IntrinsicComponentProps, IntrinsicComponentPropsData,
};

impl FieldDeclaration {
    pub fn dom_initialize(
        &self,
        crate_path: impl ToTokens,
        field_name: &syn::Ident,
    ) -> Option<Cow<TokenStream>> {
        match self {
            FieldDeclaration::Maybe(m) => {
                if m.no_cache.is_some() {
                    Some(Cow::Owned(m.to_ts_update_element(crate_path, field_name)))
                } else {
                    None
                }
            }
            FieldDeclaration::Full(f) => f
                .definitions
                .content
                .dom_definitions
                .content
                .initialize
                .as_ref()
                .map(|init| Cow::Borrowed(&init.content.content)),
            FieldDeclaration::Inherit(_) => None,
            FieldDeclaration::EventListener(_) => None,
        }
    }

    pub fn dom_implementation(
        &self,
        field_name: &syn::Ident,
        crate_path: &impl ToTokens,
        only_one_inherit_field: bool,
    ) -> Cow<TokenStream> {
        match self {
            FieldDeclaration::Maybe(m) => {
                Cow::Owned(m.to_ts_update_element(crate_path, field_name))
            }
            FieldDeclaration::Full(v) => Cow::Borrowed(
                &v.definitions
                    .content
                    .dom_definitions
                    .content
                    .implementation
                    .impl_body
                    .content,
            ),
            FieldDeclaration::EventListener(FieldDeclarationEventListener { ty, .. }) => {
                Cow::Owned(quote! {
                    #crate_path ::props::UpdateDomEventListener::<#ty>::update_dom_event_listener(this.#field_name, element, state.#field_name);
                })
            }
            FieldDeclaration::Inherit(_) => {
                let state = if only_one_inherit_field {
                    quote!(state)
                } else {
                    quote!(state.#field_name)
                };
                Cow::Owned(quote!(#crate_path ::props::UpdateElement::update_element(
                    this.#field_name,
                    element.as_ref(),
                    children_ctx,
                    #state
                );))
            }
        }
    }

    pub fn dom_bounds(
        &self,
        crate_path: &impl ToTokens,
        field_name: &syn::Ident,
    ) -> Option<TokenStream> {
        match self {
            FieldDeclaration::Maybe(FieldDeclarationMaybe { .. }) => None,
            FieldDeclaration::Full(v) => v
                .definitions
                .content
                .dom_definitions
                .content
                .bounds
                .as_ref()
                .map(|bounds| &bounds.content.content)
                .map(|simple_bounds| quote! { TypeDefs::#field_name : #simple_bounds }),
            FieldDeclaration::EventListener(v) => {
                let simple_bounds = v.to_ts_dom_bounds(crate_path);

                Some(quote! { TypeDefs::#field_name : #simple_bounds })
            }
            FieldDeclaration::Inherit(v) => {
                let base = &v.from_path;
                let dom_element_ty = &v.dom_element_ty;
                Some(quote!(
                    #base ::Data<TypeDefs::#field_name>: #crate_path ::props::UpdateElement<#dom_element_ty>
                ))
            }
        }
    }

    pub fn ssr_bounds(
        &self,
        crate_path: &impl ToTokens,
        field_name: &syn::Ident,
    ) -> Option<TokenStream> {
        if field_name == "children" {
            return Some(quote! {
                TypeDefs::#field_name: ::frender_core::UpdateRenderState<::frender_ssr::SsrContext<W>>,
                <TypeDefs::#field_name as ::frender_core::UpdateRenderState<
                    ::frender_ssr::SsrContext<W>,
                >>::State: ::core::marker::Unpin
            });
        }

        match self {
            FieldDeclaration::Maybe(_) => None,
            FieldDeclaration::Full(_) => None,
            FieldDeclaration::EventListener(v) => {
                // let simple_bounds = v.to_ts_dom_bounds(crate_path);

                // Some(quote! { TypeDefs::#field_name : #simple_bounds }) // TODO:
                None
            }
            FieldDeclaration::Inherit(v) => {
                let base = &v.from_path;
                Some(quote!(
                    #base ::Data<TypeDefs::#field_name>: ::frender_ssr::IntoSsrData<W>
                ))
            }
        }
    }

    pub fn dom_state<'a>(
        &'a self,
        crate_path: &impl ToTokens,
        field_name: &'a syn::Ident,
    ) -> Option<DomState<'a>> {
        match self {
            FieldDeclaration::Maybe(m @ FieldDeclarationMaybe { no_cache, ty, .. }) => {
                if no_cache.is_some() {
                    return None;
                }
                Some(DomState {
                    inherit: None,
                    pin: None,
                    field_name,
                    ty: Cow::Owned(syn::Type::Verbatim(quote! {
                        <TypeDefs::#field_name as ::frender_dom::props::MaybeUpdateValueWithState<#ty>>::State
                    })),
                    bounds: None,
                    initialize_state: Cow::Owned(
                        m.to_ts_dom_initialize_state(crate_path, field_name)
                            .unwrap(),
                    ),
                })
            }
            FieldDeclaration::Full(v) => {
                return v.dom_state().map(|state| {
                    let ty = Cow::Borrowed(&state.ty);
                    let bounds = state
                        .implementation
                        .as_ref()
                        .map(|im| &im.bounds.content)
                        .map_or_else(
                            || Cow::Owned(quote!(::core::default::Default)),
                            Cow::Borrowed,
                        );
                    let initialize_state = state
                        .implementation
                        .as_ref()
                        .map(|imp| &imp.initialize_state.content)
                        .map_or_else(
                            || Cow::Owned(quote!(::core::default::Default::default())),
                            Cow::Borrowed,
                        );

                    DomState {
                        inherit: None,
                        pin: state.pin,
                        field_name,
                        ty,
                        bounds: Some(bounds),
                        initialize_state,
                    }
                });
            }
            FieldDeclaration::EventListener(FieldDeclarationEventListener { ty, .. }) => {
                Some(DomState {
                    inherit: None,
                    pin: None,
                    field_name,
                    ty: Cow::Owned(syn::Type::Verbatim(quote! {
                            <TypeDefs::#field_name as #crate_path::props::UpdateDomEventListener<#ty>>::State
                    })),
                    bounds: None,
                    initialize_state: Cow::Owned(quote!(
                        #crate_path::props::UpdateDomEventListener::<#ty>::initialize_dom_event_listener_state(
                            this.#field_name,
                            element,
                        )
                    )),
                })
            }
            FieldDeclaration::Inherit(v) => {
                let path = &v.from_path;
                let dom_element_ty = &v.dom_element_ty;

                Some(DomState {
                    inherit: Some(v),
                    pin: Some(Default::default()),
                    field_name,
                    ty: Cow::Owned(syn::Type::Verbatim(quote! {
                        <#path ::Data<TypeDefs::#field_name> as #crate_path ::props::UpdateElement<#dom_element_ty>>::State
                    })),
                    bounds: Some(Cow::Owned(quote! {
                        #crate_path ::props::IntrinsicComponentPollReactive
                    })),
                    initialize_state: Cow::Owned(quote! {
                        <#path ::Data<TypeDefs::#field_name> as #crate_path ::props::UpdateElement<#dom_element_ty>>::initialize_state(
                            this.#field_name, element, children_ctx
                        )
                    }),
                })
            }
        }
    }
}

pub struct DomState<'a> {
    pub inherit: Option<&'a FieldDeclarationInherit>,
    pub pin: Option<kw::pin>,
    pub field_name: &'a syn::Ident,
    pub ty: Cow<'a, syn::Type>,
    pub bounds: Option<Cow<'a, TokenStream>>,
    pub initialize_state: Cow<'a, TokenStream>,
}

impl FieldDeclaration {
    pub fn bounds(&self, crate_path: &impl ToTokens) -> Option<Cow<TokenStream>> {
        match self {
            FieldDeclaration::Maybe(FieldDeclarationMaybe {
                ty,
                no_cache,
                by_ref,
                ..
            }) => {
                let trait_name = if no_cache.is_none() {
                    quote!(MaybeUpdateValueWithState)
                } else if by_ref.is_some() {
                    quote!(MaybeUpdateValueByRef)
                } else {
                    quote!(MaybeUpdateValue)
                };
                Some(Cow::Owned(quote!(#crate_path ::#trait_name <#ty>)))
            }
            FieldDeclaration::Full(v) => v.bounds.as_ref().map(|b| &b.content).map(Cow::Borrowed),
            FieldDeclaration::EventListener(_) => None,
            FieldDeclaration::Inherit(v) => {
                //
                let path = &v.from_path;
                Some(Cow::Owned(quote! {
                    ?::core::marker::Sized + #path ::Types
                }))
            }
        }
    }

    pub fn initial_type(&self) -> Cow<syn::Type> {
        match self {
            FieldDeclaration::Maybe(FieldDeclarationMaybe { .. }) => {
                Cow::Owned(syn::Type::Verbatim(quote!(())))
            }
            FieldDeclaration::Full(v) => Cow::Borrowed(&v.initial_type),
            FieldDeclaration::EventListener(_) => Cow::Owned(syn::Type::Verbatim(quote!(()))),
            FieldDeclaration::Inherit(v) => {
                let path = &v.from_path;
                Cow::Owned(syn::Type::Verbatim(quote!(
                    #path ::TypesInitial
                )))
            }
        }
    }

    pub fn initial_value(&self) -> Cow<syn::Expr> {
        match self {
            FieldDeclaration::Maybe(_) | FieldDeclaration::EventListener(_) => {
                Cow::Owned(syn::Expr::Verbatim(quote!(())))
            }
            FieldDeclaration::Full(v) => Cow::Borrowed(&v.initial_value),
            FieldDeclaration::Inherit(v) => {
                let path = &v.from_path;
                Cow::Owned(syn::Expr::Verbatim(quote! {
                    #path ::build(#path ())
                }))
            }
        }
    }
}

impl IntrinsicComponentPropsData {
    pub fn into_ts(self, explicit_path: Option<&TokenStream>) -> TokenStream {
        let crate_path =
            explicit_path.map_or_else(|| Cow::Owned(quote!(::frender_html)), Cow::Borrowed);

        let Self {
            attrs,
            vis,
            struct_token: _,
            name,
            dom_element,
            fields,
            inherits,
        } = self;
        let span = name.span();

        let dom_element_type = &dom_element.content.ty;

        let fields = fields.content.0;

        let (field_names, (fields_initial_value, (fields_type_item, fields_type_initial))): (
            Vec<_>,
            (Vec<_>, (Vec<_>, Vec<_>)),
        ) = fields
            .iter()
            .map(|v| {
                let name = &v.name;
                let initial_value = v.declaration.initial_value();
                let initial_type = v.declaration.initial_type();
                let bounds = v
                    .declaration
                    .bounds(&crate_path)
                    .map(|bounds| quote!(: #bounds));

                let field_initial_value = quote!(#name : #initial_value);
                let field_type_item = quote!(type #name #bounds;);
                let field_type_initial = quote!(#name = #initial_type);

                (
                    name,
                    (field_initial_value, (field_type_item, field_type_initial)),
                )
            })
            .unzip();

        let impl_field_overwrite = fields.iter().enumerate().map(|(i, field)| {
            let field_name = &field.name;
            let fields = field_names.iter().enumerate().map(|(cur, name)| {
                if cur == i {
                    quote!(#name = Value)
                } else {
                    quote! {
                        #name = <TypeDefs as super::Types>:: #name
                    }
                }
            });

            let overwrite_sub_fields = match &field.declaration {
                FieldDeclaration::Inherit(v) => Some(
                    FieldDeclarationInherit::make_overwrite_type_items(
                        &v.from_path,
                        &v.fields,
                        field_name,
                    )
                    .collect(),
                ),
                _ => None::<TokenStream>,
            };

            quote! {
                pub type #field_name < TypeDefs, Value > = dyn super::Types<
                    #(#fields),*
                >;
                #overwrite_sub_fields
            }
        });

        let dom_initialize = fields
            .iter()
            .filter_map(|f| f.declaration.dom_initialize(&crate_path, &f.name))
            .collect::<Vec<_>>();

        let dom_states = fields
            .iter()
            .filter_map(|f| f.declaration.dom_state(&crate_path, &f.name))
            .collect::<Vec<_>>();

        // Only one field and this field is inherited.
        let only_one_inherit_field;
        let state_init;
        let dom_state_type;
        let dom_state_initialize;
        let mod_render_state;

        let ssr_attrs: Vec<_> = fields.iter().filter_map(Field::to_ssr_attr).collect();

        let ssr_ty_children;
        let ssr_ty_children_state;
        let ssr_ty_attrs;
        let impl_into_ssr_data;

        if let (
            true,
            Some(DomState {
                inherit: Some(only_inherit),
                field_name,
                ..
            }),
        ) = (dom_states.len() == 1, dom_states.first())
        {
            only_one_inherit_field = true;
            state_init = None;

            let inherit_path = &only_inherit.from_path;
            let dom_el_ty = &only_inherit.dom_element_ty;

            dom_state_type = quote! {
                <#inherit_path::Data<TypeDefs::#field_name> as #crate_path::props::UpdateElement<#dom_el_ty>>::State
            };
            dom_state_initialize = quote! {
                <#inherit_path::Data<TypeDefs::#field_name> as #crate_path::props::UpdateElement<#dom_el_ty>>::initialize_state(
                    this.#field_name,
                    element,
                    children_ctx,
                )
            };
            mod_render_state = quote!(pub use super::#inherit_path::render_state;);

            ssr_ty_children = todo!();
            ssr_ty_children_state = todo!();
            ssr_ty_attrs = todo!();
            impl_into_ssr_data = todo!();
        } else {
            only_one_inherit_field = false;
            state_init = Some(quote!( let state = state.pin_project(); ));

            let dom_state_type_items = dom_states.iter().map(
                |DomState {
                     field_name, bounds, ..
                 }| quote!( type #field_name : #bounds; ),
            );
            let dom_state_fields = dom_states.iter().map(
                |DomState {
                     field_name, pin, ..
                 }| {
                    let pin = pin.map(|pin| quote!(#[#pin]));
                    quote! {
                        #pin
                        pub #field_name: TypeDefs::#field_name,
                    }
                },
            );
            let dom_state_types = dom_states
                .iter()
                .map(|DomState { field_name, ty, .. }| quote!(#field_name = #ty));
            let dom_initialize_states = dom_states.iter().map(
                |DomState {
                     field_name,
                     initialize_state,
                     ..
                 }| quote!(#field_name : #initialize_state),
            );

            let impl_poll_reactive = fields
                .iter()
                .filter_map(|v| v.to_impl_poll_reactive(&crate_path));

            dom_state_type = quote! {
                super::render_state::RenderState<
                    dyn super::render_state::RenderStateTypes<
                        #(#dom_state_types),*
                    >
                >
            };

            dom_state_initialize = quote! {
                super::render_state::RenderState {
                    #(#dom_initialize_states),*
                }
            };

            mod_render_state = quote! {
                pub mod render_state {
                    #[allow(non_camel_case_types)]
                    pub trait RenderStateTypes {
                        #(#dom_state_type_items)*
                    }

                    #crate_path ::__private::pin_project! {
                        #[project = RenderStateProj]
                        pub struct RenderState<TypeDefs: RenderStateTypes>
                        where TypeDefs: ?::core::marker::Sized {
                            #(#dom_state_fields)*
                        }
                    }

                    impl<
                        TypeDefs: ?::core::marker::Sized + RenderStateTypes,
                    > RenderState<TypeDefs> {
                        #[inline(always)]
                        pub(crate) fn pin_project(self: ::core::pin::Pin<&mut Self>) -> RenderStateProj<'_, TypeDefs> {
                            self.project()
                        }
                    }

                    impl <
                        TypeDefs: ?::core::marker::Sized + RenderStateTypes,
                    > #crate_path ::props::IntrinsicComponentPollReactive for RenderState<TypeDefs> {
                        #[inline]
                        fn intrinsic_component_poll_reactive(
                            self: ::core::pin::Pin<&mut Self>,
                            cx: &mut ::core::task::Context<'_>,
                        ) -> ::core::task::Poll<bool> {
                            #(#impl_poll_reactive)*
                        }
                    }
                }
            };

            let first_inherit_field = fields.iter().find_map(|f| {
                if let Some(d) = f.declaration.as_inherit() {
                    Some((&f.name, d))
                } else {
                    None
                }
            });
            if let Some((name, d)) = first_inherit_field {
                let from_path = &d.from_path;
                ssr_ty_children = quote! {
                    <#from_path::Data<TypeDefs::#name> as ::frender_ssr::IntoSsrData<W>>::Children
                };
                ssr_ty_children_state = quote! {
                    <#from_path::Data<TypeDefs::#name> as ::frender_ssr::IntoSsrData<W>>::ChildrenRenderState
                };

                let attrs_count = ssr_attrs.len();

                ssr_ty_attrs = quote! {
                    ::core::iter::Chain<
                        <#from_path::Data<TypeDefs::#name> as ::frender_ssr::IntoSsrData<W>>::Attrs,
                        ::frender_ssr::utils::filter::FilterArray<
                            ::frender_ssr::element::html::HtmlAttrPair<'static>,
                            #attrs_count,
                        >,
                    >
                };
                impl_into_ssr_data = quote! {
                    let (children, attrs) = ::frender_ssr::IntoSsrData::into_ssr_data(this.#name);

                    (
                        children,
                        attrs.chain(::frender_ssr::utils::filter::FilterIdentity(
                            [
                                #(#ssr_attrs),*
                            ].into_iter()
                        )),
                    )
                };
            } else {
                ssr_ty_children = quote!(TypeDefs::children);
                ssr_ty_children_state = quote! {
                    <TypeDefs::children as ::frender_core::UpdateRenderState<
                        ::frender_ssr::SsrContext<W>,
                    >>::State
                };

                let attrs_count = ssr_attrs.len();

                ssr_ty_attrs = quote! {
                    ::frender_ssr::utils::filter::FilterArray<
                        ::frender_ssr::element::html::HtmlAttrPair<'static>,
                        #attrs_count,
                    >
                };

                impl_into_ssr_data = quote! {
                    (
                        this.children,
                        ::frender_ssr::utils::filter::FilterIdentity([
                            #(#ssr_attrs),*
                        ].into_iter())
                    )
                }
            };
        }

        let impl_builder_fn = fields
            .iter()
            .map(|field| field.to_builder_fns(&field_names, &crate_path));

        let dom_bounds = fields
            .iter()
            .map(|f| f.declaration.dom_bounds(&crate_path, &f.name))
            .filter_map(std::convert::identity);

        let ssr_bounds = fields
            .iter()
            .map(|f| f.declaration.ssr_bounds(&crate_path, &f.name))
            .filter_map(std::convert::identity);

        let dom_implementations = fields.iter().map(|f| {
            f.declaration
                .dom_implementation(&f.name, &crate_path, only_one_inherit_field)
        });

        let inherits_ts = inherits.into_iter().map(|inherit| {
            let mut inherit = inherit.content;

            let name = name.clone();
            inherit.fields_mut().0.insert(
                0,
                Field {
                    attrs: vec![],
                    name: name.clone(),
                    declaration: FieldDeclaration::Inherit(FieldDeclarationInherit {
                        from_path: name.into(),
                        dom_element_ty: dom_element_type.clone(),
                        fields: fields.iter().map(Clone::clone).collect(),
                    }),
                },
            );

            inherit.into_ts(Some(&crate_path))
        });

        let impl_components = dom_element
            .content
            .component_names()
            .map(|component_names| {
                let mut component_names = component_names.iter();
                let component_name = component_names.next().unwrap(); // TODO: tolerant
                quote! {
                    #crate_path::__impl_def_intrinsic_component! {
                        #vis
                        #component_name
                        // alias_component_name
                        [#(#component_names)*]
                        #name
                        #dom_element_type
                    }
                }
            });

        let struct_fields = fields.iter().map(Field::to_struct_field);

        quote_spanned! {span=>
            #[allow(non_snake_case)]
            #vis mod #name {
                #(#attrs)*
                #[allow(non_snake_case)]
                #[inline(always)]
                #vis fn #name () -> Building<TypesInitial> {
                    #[allow(unused_imports)]
                    use super::*;
                    self::Building {
                        #(#fields_initial_value ,)*
                    }
                }

                pub mod prelude {}

                pub mod overwrite {
                    #![allow(non_camel_case_types)]

                    #(#impl_field_overwrite)*
                }

                mod trait_types {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_camel_case_types)]
                    pub trait Types {
                        #( #fields_type_item )*
                    }
                }

                pub use trait_types::Types;
                pub use trait_types::Types as ValidTypes;

                pub mod data_struct {
                    #[non_exhaustive]
                    pub struct #name<TypeDefs: super::Types + ?::core::marker::Sized> {
                        #(#struct_fields),*
                    }
                }

                pub use data_struct::#name as Data;
                pub use data_struct::#name as Building;
                pub use ::core::convert::identity as Building;
                pub use ::core::convert::identity as build;

                pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(
                    pub Data<TypeDefs>
                );

                mod types_initial {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type TypesInitial = dyn super::Types<
                        #( #fields_type_initial ,)*
                    >;
                }
                pub use types_initial::TypesInitial;

                pub type DataInitial = Data<TypesInitial>;

                #[cfg(feature = "dom")]
                #mod_render_state

                mod builder_and_replacer {
                    #[allow(unused_imports)]
                    use super::super::*;

                    impl<TypeDefs: super::Types + ?::core::marker::Sized>
                    super::Building<TypeDefs> {
                        #(#impl_builder_fn)*
                    }
                }

                #[cfg(feature = "dom")]
                mod impl_update_element {
                    #[allow(unused_imports)]
                    use super::super::*;
                    impl<
                        TypeDefs: ?::core::marker::Sized + super::Types,
                    > #crate_path ::props::UpdateElement<#dom_element_type> for super::Data<TypeDefs>
                    where
                        #(#dom_bounds),*
                    {
                        type State = #dom_state_type;

                        fn initialize_state(this: Self, element: &#dom_element_type, children_ctx: &mut ::frender_dom::Dom) -> Self::State {
                            let dom_element: &::web_sys::Element = element.as_ref();

                            #(#dom_initialize)*

                            #dom_state_initialize
                        }

                        fn update_element(this: Self, element: &#dom_element_type, children_ctx: &mut ::frender_dom::Dom, state: ::core::pin::Pin<&mut Self::State>) {
                            #state_init

                            let dom_element: &::web_sys::Element = element.as_ref();

                            #(#dom_implementations)*
                        }
                    }
                }

                #[cfg(feature = "ssr")]
                mod impl_into_ssr_data {
                    #[allow(unused_imports)]
                    use super::super::*;
                    impl<
                            TypeDefs: ?::core::marker::Sized + super::Types,
                            W: ::frender_ssr::AsyncWrite + ::core::marker::Unpin,
                        > ::frender_ssr::IntoSsrData<W> for super::Data<TypeDefs>
                    where
                        #(#ssr_bounds),*
                    {
                        type Children = #ssr_ty_children;
                        type ChildrenRenderState = #ssr_ty_children_state;

                        type Attrs = #ssr_ty_attrs;

                        fn into_ssr_data(this: Self) -> (Self::Children, Self::Attrs) {
                            #impl_into_ssr_data
                        }
                    }
                }
            }

            #vis use #name::#name;

            #impl_components

            #(#inherits_ts)*
        }
    }
}

impl IntrinsicComponentProps {
    pub fn try_unzip(self) -> syn::Result<Vec<IntrinsicComponentPropsData>> {
        match self {
            IntrinsicComponentProps::Virtual(v) => v.try_unzip(),
            IntrinsicComponentProps::Data(d) => Ok(vec![d]),
        }
    }

    pub fn into_ts(self, explicit_path: Option<&TokenStream>) -> TokenStream {
        self.try_unzip().map_or_else(
            |err| err.into_compile_error(),
            |data| {
                data.into_iter()
                    .map(|data| data.into_ts(explicit_path))
                    .collect()
            },
        )
    }
}
