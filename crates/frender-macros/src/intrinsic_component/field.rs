use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;

use super::{Field, FieldDeclarationInherit};

impl Field {
    pub fn to_struct_field(&self) -> TokenStream {
        let Self {
            name, declaration, ..
        } = self;
        match declaration {
            super::FieldDeclaration::Inherit(v) => {
                let path = &v.from_path;
                quote! {
                    pub #name : super::super:: #path ::Data<TypeDefs::#name>
                }
            }
            _ => quote! {
                pub #name : TypeDefs::#name
            },
        }
    }

    pub fn to_impl_poll_reactive(&self, crate_path: &impl ToTokens) -> Option<TokenStream> {
        let name = &self.name;

        match &self.declaration {
            super::FieldDeclaration::Inherit(_) => Some(quote! {
                #crate_path::props::IntrinsicComponentPollReactive::intrinsic_component_poll_reactive(
                    self.project(). #name,
                    cx,
                )
            }),
            _ if name == "children" => Some(quote! {
                #crate_path::__private::RenderState::poll_reactive(
                    self.project(). #name,
                    cx
                )
            }),
            _ => None,
        }
    }

    pub fn to_builder_fn(
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
            #[inline]
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

    pub fn to_builder_fns(
        &self,
        all_fields: &Vec<&syn::Ident>,
        crate_path: &impl ToTokens,
    ) -> TokenStream {
        match &self.declaration {
            super::FieldDeclaration::Inherit(v) => FieldDeclarationInherit::make_builder_fns(
                crate_path,
                &v.from_path,
                &v.fields,
                &self.name,
                all_fields,
            ),
            _ => self.to_builder_fn(all_fields, crate_path),
        }
    }
}
