use std::borrow::Cow;

use crate::err::RecordError;

use super::rsx_data::*;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::spanned::Spanned;

impl ToTokens for LitOrBraced {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            LitOrBraced::Lit(lit) => lit.to_tokens(tokens),
            LitOrBraced::Braced { inner, .. } => inner.to_tokens(tokens),
        }
    }

    fn into_token_stream(self) -> proc_macro2::TokenStream
    where
        Self: Sized,
    {
        match self {
            LitOrBraced::Lit(lit) => lit.into_token_stream(),
            LitOrBraced::Braced { inner, .. } => inner,
        }
    }
}

impl RsxElement {
    pub fn try_into_ts(
        self,
        crate_path: &proc_macro2::TokenStream,
        errors: &mut impl RecordError<syn::Error>,
    ) -> proc_macro2::TokenStream {
        let RsxElement {
            component_type,
            key,
            props,
            children,
            start_lt: _,
        } = self;

        let start_gt_span = children.start_gt().span;

        let element = match component_type {
            RsxComponentType::Fragment(pound) => {
                for prop in props {
                    errors.record_error(syn::Error::new_spanned(
                        prop.name,
                        if pound.is_some() {
                            "<#> fragments only support `key` property"
                        } else {
                            "<> fragments only support `key` property"
                        },
                    ));
                }

                children
                    .try_into_ts(crate_path, errors)
                    .unwrap_or_else(|| quote_spanned!(start_gt_span=> ()))
            }
            RsxComponentType::Path(component_path) => {
                let props_chain: proc_macro2::TokenStream = props
                    .into_iter()
                    .map(|prop| {
                        let RsxProp { name, value } = prop;

                        let span = name.span();

                        let value = if let Some(value) = value {
                            value.value.into_token_stream()
                        } else {
                            quote_spanned!(span=>
                                #crate_path ::Omitted
                            )
                        };

                        quote_spanned! {span=>
                            . #name ( #value )
                        }
                    })
                    .collect();

                let props_children = children.try_into_ts(crate_path, errors).map(|value| {
                    quote_spanned! {start_gt_span=>
                        .children(#value)
                    }
                });

                quote_spanned! {start_gt_span=>
                    #crate_path ::rsx_build_element! {
                        #component_path ()
                            #props_chain
                            #props_children
                    }
                }
            }
        };

        if let Some(key) = key {
            let value = key.value.value;
            quote_spanned! {start_gt_span=>
                #crate_path ::Keyed(#element, #value)
            }
        } else {
            element
        }
    }
}

impl RsxChild {
    pub fn try_into_ts(
        self,
        crate_path: &proc_macro2::TokenStream,
        errors: &mut impl RecordError<syn::Error>,
    ) -> proc_macro2::TokenStream {
        match self {
            RsxChild::LitOrBraced(le) => le.into_token_stream(),
            RsxChild::Element(el) => el.try_into_ts(crate_path, errors),
        }
    }
}

impl RsxElementChildren {
    pub fn try_into_ts(
        self,
        crate_path: &proc_macro2::TokenStream,
        errors: &mut impl RecordError<syn::Error>,
    ) -> Option<proc_macro2::TokenStream> {
        if let Some((children, span)) = self.unwrap_children_and_span() {
            let mut group = proc_macro2::Group::new(
                proc_macro2::Delimiter::Parenthesis,
                children
                    .into_iter()
                    .map(|child| child.try_into_ts(crate_path, errors))
                    .map(|mut ts| {
                        ts.append(proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone));
                        ts
                    })
                    .collect(),
            );
            group.set_span(span);
            Some(proc_macro2::TokenTree::Group(group).into())
        } else {
            None
        }
    }
}

impl OptionalCratePathAndRsxChild {
    pub fn into_ts(self) -> proc_macro2::TokenStream {
        let mut errors: Vec<syn::Error> = vec![];

        let crate_path = self
            .crate_path
            .map_or_else(|| quote::quote!(::frender), |v| v.path);
        let value = self.child.try_into_ts(&crate_path, &mut errors);
        if errors.is_empty() {
            value
        } else {
            let errors = errors.into_iter().map(|error| error.into_compile_error());
            quote::quote! {
                (
                    #value,
                    {
                        #(#errors ;)*
                    }
                ).0
            }
        }
    }
}
