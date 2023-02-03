use std::borrow::Cow;

use proc_macro2::{Group, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse_quote;

use crate::utils::grouped::{Braced, Bracketed};

use super::{
    kw, Field, FieldDeclaration, FieldDeclarationDefinitions, FieldDeclarationDomDefinitions,
    FieldDeclarationDomState, FieldDeclarationFull, IntrinsicComponentPropsData,
};

impl FieldDeclaration {
    pub fn dom_pat(&self) -> Cow<TokenTree> {
        match self {
            FieldDeclaration::Maybe { .. } => {
                Cow::Owned(TokenTree::Group(proc_macro2::Group::new(
                    proc_macro2::Delimiter::Brace,
                    quote!(data, dom_element, state, element, ..),
                )))
            }
            FieldDeclaration::Full(v) => Cow::Borrowed(
                &v.definitions
                    .content
                    .dom_definitions
                    .content
                    .implementation
                    .pat,
            ),
        }
    }

    pub fn dom_implementation(
        &self,
        field_name: &syn::Ident,
        crate_path: &impl ToTokens,
    ) -> Cow<TokenTree> {
        match self {
            FieldDeclaration::Maybe {
                question_token,
                ty,
                details,
            } => {
                let html_prop_name = details
                    .as_ref()
                    .and_then(|d| d.content.html_prop_name.as_ref())
                    .map_or_else(
                        || Cow::Owned(syn::LitStr::new(&field_name.to_string(), field_name.span())),
                        Cow::Borrowed,
                    );
                let html_element_method = details
                    .as_ref()
                    .and_then(|d| d.content.html_element_method.as_ref());
                let update = details
                    .as_ref()
                    .and_then(|d| d.content.impl_update.as_ref().map(|imp| &imp.content))
                    .map_or_else(
                        || {
                            let imp=
                            if let Some(html_element_method) = html_element_method {
                                let element_method=&html_element_method.name;
                                let deref_star = html_element_method.deref_star_token.as_ref();
                                quote! {
                                    element.#element_method(#deref_star v)
                                }
                            } else {
                                quote! {
                                    #crate_path::props::UpdateElementAttribute::update_element_attribute(v, dom_element, ATTR_NAME)
                                }
                            };

                            quote!(|v| #imp)
                        },
                        |update| {
                        let maybe_update_element = &update.element_pat_ident;
                        let maybe_update = &update.rest;
                        let v = quote! {
                            match element { #maybe_update_element => #maybe_update }
                        };
                        if let Some(html_element_method) = html_element_method {
                            let error = syn::Error::new(html_element_method.name.span(), "html_element_method not respected because custom update is implemented")
                                .into_compile_error();
                            quote! {
                                (#v, #error).0
                            }
                        } else {
                            v
                        }
                    });

                let remove = details
                    .as_ref()
                    .and_then(|d| d.content.impl_remove.as_ref().map(|imp| &imp.content))
                    .map_or_else(
                        || {
                            quote! {
                                || dom_element.remove_attribute(ATTR_NAME).unwrap()
                            }
                        },
                        |remove| {
                            let maybe_remove_element = &remove.element_pat_ident;
                            let maybe_remove = &remove.rest;
                            quote! {
                                match element { #maybe_remove_element => #maybe_remove }
                            }
                        },
                    );

                Cow::Owned(TokenTree::Group(Group::new(
                    proc_macro2::Delimiter::Brace,
                    quote! {
                        #[allow(unused)]
                        const ATTR_NAME: &::core::primitive::str = #html_prop_name ;
                        <TypeDefs::#field_name as ::frender_dom::props::MaybeUpdateValue<#ty>>::maybe_update_value(
                            data,
                            state,
                            #update,
                            #remove
                        )
                    },
                )))
            }
            FieldDeclaration::Full(v) => Cow::Borrowed(
                &v.definitions
                    .content
                    .dom_definitions
                    .content
                    .implementation
                    .impl_body,
            ),
        }
    }

    pub fn dom_bounds(&self) -> Option<&TokenStream> {
        match self {
            FieldDeclaration::Maybe { .. } => None,
            FieldDeclaration::Full(v) => v
                .definitions
                .content
                .dom_definitions
                .content
                .bounds
                .as_ref()
                .map(|bounds| &bounds.content.content),
        }
    }

    pub fn dom_state_initial_value(&self) -> Cow<syn::Expr> {
        self.dom_state()
            .and_then(|state| state.implementation.as_ref())
            .map_or_else(
                || {
                    Cow::Owned(syn::Expr::Verbatim(quote!(
                        ::core::default::Default::default()
                    )))
                },
                |im| Cow::Borrowed(&im.initial_value.content),
            )
    }

    pub fn dom_state(&self) -> Option<&FieldDeclarationDomState> {
        match self {
            FieldDeclaration::Maybe { .. } => None,
            FieldDeclaration::Full(v) => v
                .definitions
                .content
                .dom_definitions
                .content
                .state
                .as_ref()
                .map(|b| &b.content),
        }
    }

    pub fn dom_state_bounds(&self) -> Cow<TokenStream> {
        self.dom_state()
            .and_then(|v| v.implementation.as_ref().map(|im| &im.bounds.content))
            .map_or_else(
                || Cow::Owned(quote!(::core::default::Default)),
                Cow::Borrowed,
            )
    }

    pub fn dom_state_type(&self, field_name: &syn::Ident) -> Option<Cow<syn::Type>> {
        match self {
            FieldDeclaration::Maybe {
                question_token,
                ty,
                details,
            } => Some(Cow::Owned(syn::Type::Verbatim(
                quote! {<TypeDefs::#field_name as ::frender_dom::props::MaybeUpdateValue<#ty>>::State},
            ))),
            FieldDeclaration::Full(v) => v
                .definitions
                .content
                .dom_definitions
                .content
                .state
                .as_ref()
                .map(|state| Cow::Borrowed(&state.content.ty)),
        }
    }
}

impl Field {
    pub fn dom_render_state_type_item(&self) -> TokenStream {
        let name = &self.name;
        let dom_state_bounds = self.declaration.dom_state_bounds();
        quote! {
            type #name : #dom_state_bounds;
        }
    }

    pub fn builder_fn(
        &self,
        all_fields: &Vec<&syn::Ident>,
        crate_path: &impl ToTokens,
    ) -> TokenStream {
        let Self {
            attrs,
            name,
            declaration,
        } = self;
        let bounds = declaration
            .bounds(crate_path)
            .map(|bounds| quote!(: #bounds));
        let new_value = all_fields.iter().map(|field| {
            if *field == name {
                name.into_token_stream()
            } else {
                quote!(#field: self.0.#field)
            }
        });
        quote! {
            #(#attrs)*
            pub fn #name <V #bounds>(
                self,
                #name: V,
            ) -> super::Building<
                super::overwrite::#name<TypeDefs, V>,
            > {
                super::Building(
                    super::Data {
                        #(#new_value),*
                    }
                )
            }
        }
    }
}

impl FieldDeclaration {
    pub fn dom_state_pin(&self) -> Option<&kw::pin> {
        match self {
            FieldDeclaration::Maybe { .. } => None,
            FieldDeclaration::Full(v) => v
                .definitions
                .content
                .dom_definitions
                .content
                .state
                .as_ref()
                .and_then(|state| state.content.pin.as_ref()),
        }
    }

    pub fn bounds(&self, crate_path: &impl ToTokens) -> Option<Cow<TokenStream>> {
        match self {
            FieldDeclaration::Maybe { ty, .. } => {
                Some(Cow::Owned(quote!(#crate_path ::MaybeUpdateValue<#ty>)))
            }
            FieldDeclaration::Full(v) => v.bounds.as_ref().map(|b| &b.content).map(Cow::Borrowed),
        }
    }

    pub fn initial_type(&self) -> Cow<syn::Type> {
        match self {
            FieldDeclaration::Maybe { .. } => Cow::Owned(syn::Type::Verbatim(quote!(()))),
            FieldDeclaration::Full(v) => Cow::Borrowed(&v.initial_type),
        }
    }

    pub fn initial_value(&self) -> Cow<syn::Expr> {
        match self {
            FieldDeclaration::Maybe { .. } => Cow::Owned(syn::Expr::Verbatim(quote!(()))),
            FieldDeclaration::Full(v) => Cow::Borrowed(&v.initial_value),
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

        let field_dom_state_init = fields.iter().map(|field| {
            let name = &field.name;
            let value = field.declaration.dom_state_initial_value();
            quote! {
                #name : #value
            }
        });

        let impl_field_overwrite = field_names.iter().enumerate().map(|(i, field)| {
            let fields = field_names.iter().enumerate().map(|(cur, name)| {
                if cur == i {
                    quote!(#name = Value)
                } else {
                    quote! {
                        #name = <TypeDefs as super::Types>:: #name
                    }
                }
            });
            quote! {
                pub type #field < TypeDefs, Value > = dyn super::Types<
                    #(#fields),*
                >;
            }
        });

        let dom_state_type_items = fields.iter().map(Field::dom_render_state_type_item);
        let dom_state_fields = fields.iter().map(|field| {
            let pin = field.declaration.dom_state_pin().map(|pin| quote!(#[#pin]));
            let name = &field.name;
            quote! {
                #pin
                pub #name: TypeDefs::#name,
            }
        });

        let impl_builder_fn = fields
            .iter()
            .map(|field| field.builder_fn(&field_names, &crate_path));

        let dom_bounds = fields
            .iter()
            .map(|f| f.declaration.dom_bounds().map(|bounds| (f, bounds)))
            .filter_map(|v| {
                v.map(|(field, bounds)| {
                    let name = &field.name;
                    quote! { TypeDefs::#name : #bounds ,}
                })
            });

        let dom_state_types = fields.iter().map(|f| {
            let name = &f.name;
            let ty = f
                .declaration
                .dom_state_type(name)
                .unwrap_or_else(|| Cow::Owned(syn::Type::Verbatim(quote!(()))));
            quote!(#name = #ty)
        });

        let dom_pats = fields.iter().map(|f| f.declaration.dom_pat());
        let dom_implementations = fields
            .iter()
            .map(|f| f.declaration.dom_implementation(&f.name, &crate_path));

        let inherits_ts = inherits.into_iter().map(|inherit| {
            let mut inherit = inherit.content;
            let mut inherit_fields = fields.clone();
            let fields = &mut inherit.fields.content.0;

            inherit_fields.extend(std::mem::take(fields));

            *fields = inherit_fields;

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

        quote_spanned! {span=>
            #[allow(non_snake_case)]
            #vis mod #name {
                #(#attrs)*
                #[allow(non_snake_case)]
                #vis fn #name () -> Building<TypesInitial> {
                    #[allow(unused_imports)]
                    use super::*;
                    self::Building(
                        self::Data {
                            #(#fields_initial_value ,)*
                        }
                    )
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
                        #(
                            pub #field_names : TypeDefs::#field_names,
                        )*
                    }
                }

                pub use data_struct::#name as Data;

                pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(
                    pub Data<TypeDefs>
                );

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
                        #[inline]
                        pub(crate) fn pin_project(self: ::core::pin::Pin<&mut Self>) -> RenderStateProj<'_, TypeDefs> {
                            self.project()
                        }
                    }

                    impl<
                        TypeDefs: ?::core::marker::Sized + RenderStateTypes,
                    > ::core::default::Default for RenderState<TypeDefs> {
                        fn default() -> Self {
                            Self {
                                #(#field_dom_state_init),*
                            }
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
                            ::frender_core::RenderState::poll_reactive(
                                self.project().children,
                                cx
                            )
                        }
                    }
                }

                #[inline]
                pub fn build<TypeDefs: ?::core::marker::Sized + Types>(building: Building<TypeDefs>) -> Data::<TypeDefs> {
                    building.0
                }

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
                        #(#dom_bounds)*
                    {
                        type State = super::render_state::RenderState<
                            dyn super::render_state::RenderStateTypes<
                                #(#dom_state_types),*
                            >
                        >;

                        fn update_element(this: Self, element: &#dom_element_type, children_ctx: &mut ::frender_dom::Dom, state: ::core::pin::Pin<&mut Self::State>) {
                            let state = state.pin_project();

                            let dom_element: &::web_sys::Element = element.as_ref();

                            #(
                                #[allow(unused_variables)]
                                match (#crate_path ::props::FieldData {
                                    data: this. #field_names,
                                    state: state. #field_names,
                                    element,
                                    dom_element,
                                    children_ctx: &mut *children_ctx,
                                }) {
                                    #crate_path::props::FieldData #dom_pats => #dom_implementations
                                }
                            )*
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
