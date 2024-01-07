use crate::err::RecordError;

use super::rsx_data::*;

use quote::{quote_spanned, ToTokens, TokenStreamExt};

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
            start_lt,
        } = self;

        let start_lt_span = start_lt.span;
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
            RsxComponentType::Path(mut component_path) => {
                if component_path
                    .get_ident()
                    .map_or(false, ident_is_intrinsic_component)
                {
                    assert!(component_path.leading_colon.is_none());
                    component_path.segments.insert(
                        0,
                        syn::PathSegment::from(syn::Ident::new(
                            "intrinsic_components",
                            start_lt_span,
                        )),
                    );
                }

                PureRsxElement {
                    start_lt,
                    component_path,
                    props,
                    children,
                }
                .into_ts(crate_path, errors)
            }
        };

        if let Some(key) = key {
            let value = key.value.value;
            quote_spanned! {start_gt_span=>
                #crate_path ::Keyed(#value, #element)
            }
        } else {
            element
        }
    }
}

impl PureRsxElement {
    fn into_ts(
        self,
        crate_path: &proc_macro2::TokenStream,
        errors: &mut impl RecordError<syn::Error>,
    ) -> proc_macro2::TokenStream {
        let Self {
            start_lt,
            component_path,
            props,
            children,
        } = self;

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

        let start_gt_span = children.start_gt().span;

        let props_children = children.try_into_ts(crate_path, errors).map(|value| {
            quote_spanned! {start_gt_span=>
                .children(#value)
            }
        });

        quote_spanned! { start_lt.span => {
            #[allow(unused_imports)]
            use #component_path::prelude::*;
            #component_path::build_element(
                // base expr
                #component_path()
                #props_chain
                #props_children
            )
        }}
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

fn tuple_of(
    items: impl Iterator<Item = proc_macro2::TokenStream>,
    span: proc_macro2::Span,
) -> proc_macro2::TokenStream {
    let mut group = proc_macro2::Group::new(
        proc_macro2::Delimiter::Parenthesis,
        items
            .flat_map(|ts| {
                [
                    ts,
                    proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                        ',',
                        proc_macro2::Spacing::Alone,
                    ))
                    .into(),
                ]
            })
            .collect(),
    );

    group.set_span(span);
    proc_macro2::TokenTree::Group(group).into()
}

fn children_into_tuple(
    children: Vec<RsxChild>,
    crate_path: &proc_macro2::TokenStream,
    errors: &mut impl RecordError<syn::Error>,
    span: proc_macro2::Span,
) -> proc_macro2::TokenStream {
    tuple_of(
        children
            .into_iter()
            .map(|child| child.try_into_ts(crate_path, errors)),
        span,
    )
}

/// At most 12 elements.
fn children_into_auto_tuple(
    mut children: Vec<RsxChild>,
    crate_path: &proc_macro2::TokenStream,
    errors: &mut impl RecordError<syn::Error>,
    span: proc_macro2::Span,
) -> proc_macro2::TokenStream {
    if children.len() == 1 {
        let child = children.into_iter().next().unwrap();
        child.try_into_ts(crate_path, errors)
    } else if children.len() <= 12 {
        children_into_tuple(children, crate_path, errors, span)
    } else {
        let rest = children.split_off(12);
        tuple_of(
            [
                children_into_auto_tuple(children, crate_path, errors, span),
                children_into_auto_tuple(rest, crate_path, errors, span),
            ]
            .into_iter(),
            span,
        )
    }
}

impl RsxElementChildren {
    pub fn try_into_ts(
        self,
        crate_path: &proc_macro2::TokenStream,
        errors: &mut impl RecordError<syn::Error>,
    ) -> Option<proc_macro2::TokenStream> {
        self.unwrap_children_and_span()
            .map(|(children, span)| children_into_auto_tuple(children, crate_path, errors, span))
    }
}

impl RsxChild {
    pub fn into_ts(self, crate_path: Option<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
        let mut errors: Vec<syn::Error> = vec![];

        let crate_path = crate_path.unwrap_or_else(|| quote::quote!(::frender));
        let value = self.try_into_ts(&crate_path, &mut errors);
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
