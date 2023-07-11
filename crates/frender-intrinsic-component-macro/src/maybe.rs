use std::borrow::Cow;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;

use crate::utils::grouped::Braced;

use super::FieldDeclarationMaybeDetails;

#[derive(Clone)]
pub struct FieldDeclarationMaybe {
    pub question_token: syn::Token![?],
    pub no_cache: Option<syn::Token![-]>,
    pub by_ref: Option<syn::Token![&]>,
    pub ty: syn::Type,
    pub details: Option<Braced<FieldDeclarationMaybeDetails>>,
}

impl Parse for FieldDeclarationMaybe {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            question_token: input.parse()?,
            no_cache: input.parse()?,
            by_ref: input.parse()?,
            ty: input.parse()?,
            details: input.parse()?,
        })
    }
}

impl FieldDeclarationMaybe {
    pub fn to_html_prop_name(&self, field_name: &syn::Ident) -> Cow<syn::LitStr> {
        self.details
            .as_ref()
            .and_then(|d| d.content.html_prop_name.as_ref())
            .map_or_else(
                || Cow::Owned(syn::LitStr::new(&field_name.to_string(), field_name.span())),
                Cow::Borrowed,
            )
    }

    pub fn to_ts_closure_update(
        &self,
        crate_path: impl ToTokens,
        html_prop_name: impl ToTokens,
    ) -> TokenStream {
        let FieldDeclarationMaybe {
            question_token,
            no_cache,
            by_ref,
            ty: _,
            details,
        } = self;
        let deref_star = if no_cache.is_none() && by_ref.is_none() {
            Some(syn::Token![*](question_token.span))
        } else {
            None
        };
        let html_element_method = details
            .as_ref()
            .and_then(|d| d.content.html_element_method.as_ref());
        details
                .as_ref()
                .and_then(|d| d.content.impl_update.as_ref().map(|imp| &imp.content))
                .map_or_else(
                    || {
                        let imp=
                        if let Some(element_method) = html_element_method {
                            quote! {
                                dom_element.#element_method(#deref_star v)
                            }
                        } else {
                            quote! {
                                #crate_path::frender_csr::props::UpdateElementAttribute::update_element_attribute(#deref_star v, dom_element, #html_prop_name)
                            }
                        };

                        quote!(|v| #imp)
                    },
                    |update| {
                    let maybe_update_element = &update.element_pat_ident;
                    let maybe_update = &update.rest;
                    let v = quote! {
                        match dom_element { #maybe_update_element => #maybe_update }
                    };
                    if let Some(html_element_method) = html_element_method {
                        let error = syn::Error::new(html_element_method.span(), "html_element_method not respected because custom update is implemented")
                            .into_compile_error();
                        quote! {
                            (#v, #error).0
                        }
                    } else {
                        v
                    }
                })
    }

    pub fn to_ts_closure_remove(&self, html_prop_name: impl ToTokens) -> TokenStream {
        let FieldDeclarationMaybe {
            question_token: _,
            no_cache,
            by_ref,
            ty,
            details,
        } = self;

        details
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
                        match dom_element { #maybe_remove_element => #maybe_remove }
                    }
                },
            )
    }

    pub fn to_ts_dom_initialize_state(
        &self,
        crate_path: &impl ToTokens,
        field_name: &syn::Ident,
    ) -> Option<TokenStream> {
        self.to_ts_dom_initialize_state_custom(
            crate_path,
            field_name,
            quote!(TypeDefs::#field_name),
            quote!(this.#field_name),
        )
    }

    pub fn to_ts_dom_initialize_state_custom(
        &self,
        crate_path: &impl ToTokens,
        field_name: &syn::Ident,
        field_ty: impl ToTokens,
        field_expr: impl ToTokens,
    ) -> Option<TokenStream> {
        let FieldDeclarationMaybe {
            question_token: _,
            no_cache,
            by_ref,
            ty,
            details,
        } = self;

        if no_cache.is_some() {
            return None;
        }

        let html_prop_name = self.to_html_prop_name(field_name);

        let update = self.to_ts_closure_update(crate_path, &html_prop_name);
        let remove = self.to_ts_closure_remove(&html_prop_name);

        Some(quote! {
            <#field_ty as #crate_path::frender_html::props::MaybeUpdateValueWithState<#ty>>::initialize_state_and_update(
                #field_expr,
                #update,
                #remove
            )
        })
    }

    pub fn to_ts_update_element(
        &self,
        crate_path: &impl ToTokens,
        field_name: &syn::Ident,
    ) -> TokenStream {
        self.to_ts_update_element_custom(
            crate_path,
            field_name,
            quote!(TypeDefs::#field_name),
            quote!(this.#field_name),
            quote!(state.#field_name),
        )
    }

    pub fn to_ts_update_element_custom(
        &self,
        crate_path: &impl ToTokens,
        field_name: &syn::Ident,
        field_ty: impl ToTokens,
        field_expr: impl ToTokens,
        state_expr_for_maybe_with_state: impl ToTokens,
    ) -> TokenStream {
        let FieldDeclarationMaybe {
            question_token: _,
            no_cache,
            by_ref,
            ty,
            details,
        } = self;

        let html_prop_name = self.to_html_prop_name(field_name);

        let update = self.to_ts_closure_update(crate_path, &html_prop_name);
        let remove = self.to_ts_closure_remove(&html_prop_name);

        let (trait_name, trait_method, state_and_comma, by_ref) = if no_cache.is_none() {
            (
                quote!(MaybeUpdateValueWithState),
                quote!(maybe_update_value_with_state),
                Some(quote!(#state_expr_for_maybe_with_state ,)),
                None,
            )
        } else if by_ref.is_some() {
            (
                quote!(MaybeUpdateValueByRef),
                quote!(maybe_update_value_by_ref),
                None,
                by_ref.as_ref(),
            )
        } else {
            (
                quote!(MaybeUpdateValue),
                quote!(maybe_update_value),
                None,
                None,
            )
        };

        quote! {
            <#field_ty as #crate_path::frender_html::props::#trait_name<#ty>>::#trait_method(
                #by_ref #field_expr,
                #state_and_comma
                #update,
                #remove
            );
        }
    }
}
