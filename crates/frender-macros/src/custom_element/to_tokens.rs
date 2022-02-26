use proc_macro2::TokenStream;
use quote::quote_spanned;

use super::data::*;
impl CustomElementDefinition {
    pub fn into_ts(self) -> TokenStream {
        let Self {
            errors,
            item_struct,
            options,
        } = self;
        let CustomElementItemStruct {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            mut fields,
            semi_token,
        } = item_struct;

        let CustomElementOptions {
            react,
            no_derive_clone,
            no_derive_debug,
            no_from_element,
        } = options;

        let span = ident.span();
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        let react = react.unwrap_or_else(|| default_react_mod_path(span));

        let derive = if no_derive_clone.is_none() && no_derive_debug.is_none() {
            None
        } else {
            let debug = no_derive_clone.map(|_| syn::Ident::new("Debug", span));
            let clone = no_derive_clone.map(|_| syn::Ident::new("Clone", span));
            Some(quote_spanned! {span=>
                #[derive( #debug #clone )]
            })
        };

        if fields.unnamed.len() == 0 {
            fields.unnamed.push(syn::Field {
                attrs: vec![],
                colon_token: None,
                ident: None,
                ty: syn::parse_quote_spanned!(span=> #react::Element),
                vis: syn::Visibility::Inherited,
            })
        }
        let fields = fields;

        let element_ty = &fields.unnamed.first().unwrap().ty;

        let fields_len = fields.unnamed.len();
        let impl_from_element = if no_from_element.is_none() {
            None
        } else {
            let ts = if fields_len > 1 {
                let default = quote_spanned!(span=> ::core::default::Default::default());
                let init_fields = (1..fields_len).map(|_| default.clone());
                quote_spanned! {span=>
                    impl #impl_generics #ident #type_generics #where_clause {
                        fn private_from_element(el: #element_ty) -> Self {
                            Self( el, #(#init_fields)* )
                        }
                    }
                }
            } else {
                quote_spanned!(span=> el)
            };

            Some(ts)
        };

        let error_ts = if errors.len() > 0 {
            Some(
                darling::Error::multiple(errors)
                    .with_span(&ident)
                    .write_errors(),
            )
        } else {
            None
        };

        quote_spanned! {span=>
            #(#attrs)*
            #derive
            #vis #struct_token #ident #generics #fields #semi_token

            #impl_from_element

            impl #impl_generics #react::IntoElement for #ident #type_generics #where_clause {
                #[inline]
                fn into_element(self) -> #react::Element {
                    #react::IntoElement::into_element(self.0)
                }
            }

            impl #impl_generics Into<#element_ty> for #ident #type_generics #where_clause {
                #[inline]
                fn into(self) -> #element_ty {
                    self.0
                }
            }

            impl #impl_generics AsRef<#element_ty> for #ident #type_generics #where_clause {
                #[inline]
                fn as_ref(&self) -> &#element_ty {
                    &self.0
                }
            }

            impl #impl_generics #react::Node for #ident #type_generics #where_clause {
                #[inline]
                fn to_node(&self) -> #react::AnyNode {
                    #react::Node::to_node(&self.0)
                }

                #[inline]
                fn to_children(&self) -> Option<#react::Children> {
                    #react::Node::to_children(&self.0)
                }

                #[inline]
                fn into_node(self) -> #react::AnyNode {
                    #react::Node::into_node(self.0)
                }

                #[inline]
                fn into_children(self) -> Option<#react::Children> {
                    #react::Node::into_children(self.0)
                }
            }

            impl #impl_generics ::core::ops::Deref for #ident #type_generics #where_clause {
                type Target = #element_ty;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            #error_ts
        }
    }
}
