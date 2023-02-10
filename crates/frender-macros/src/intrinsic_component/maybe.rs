use std::borrow::Cow;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::utils::grouped::Braced;

use super::FieldDeclarationMaybeDetails;

#[derive(Clone)]
pub struct FieldDeclarationMaybe {
    pub question_token: syn::Token![?],
    pub by_ref: Option<syn::Token![&]>,
    pub ty: syn::Type,
    pub details: Option<Braced<FieldDeclarationMaybeDetails>>,
}

impl FieldDeclarationMaybe {
    pub fn to_ts_update_element(
        &self,
        crate_path: impl ToTokens,
        field_name: &syn::Ident,
    ) -> TokenStream {
        let FieldDeclarationMaybe {
            question_token: _,
            by_ref,
            ty,
            details,
        } = self;

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
                                #crate_path::props::UpdateElementAttribute::update_element_attribute(v, dom_element, #html_prop_name)
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
                        || dom_element.remove_attribute(#html_prop_name).unwrap()
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

        let (trait_name, trait_method) = if by_ref.is_some() {
            (
                quote!(MaybeUpdateValueByRef),
                quote!(maybe_update_value_by_ref),
            )
        } else {
            (quote!(MaybeUpdateValue), quote!(maybe_update_value))
        };

        quote! {
            <TypeDefs::#field_name as ::frender_dom::props::#trait_name<#ty>>::#trait_method(
                #by_ref this.#field_name,
                #update,
                #remove
            );
        }
    }
}
