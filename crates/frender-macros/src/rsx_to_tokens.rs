use super::rsx_data::*;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::spanned::Spanned;

impl ToTokens for LitOrBracedExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            LitOrBracedExpr::Lit(lit) => lit.to_tokens(tokens),
            LitOrBracedExpr::BracedExpr { expr, .. } => expr.to_tokens(tokens),
        }
    }
}

impl ToTokens for RsxPropValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { colon, value, .. } = self;
        if colon.is_some() {
            value.to_tokens(tokens)
        } else {
            let span = colon.span();
            tokens.append_all(quote_spanned! {span=>
                ::frender::IntoPropValue::into_prop_value( #value )
            })
        }
    }
}

impl RsxComponentType {
    pub fn type_to_token_stream(
        &self,
        span_for_fragment: Option<Span>,
    ) -> proc_macro2::TokenStream {
        match self {
            RsxComponentType::Fragment(_) => {
                let span = span_for_fragment.unwrap_or_else(Span::call_site);
                quote_spanned!(span=>
                    self::rsx_runtime::Fragment)
            }
            RsxComponentType::Intrinsic(tag) => {
                let span = tag.span();
                quote_spanned!(span=>
                    self::intrinsic_components::#tag::Component)
            }
            RsxComponentType::TypePath(tp) => tp.to_token_stream(),
        }
    }

    pub fn intrinsic_pre_build_token_stream(
        &self,
        builder_ident: &syn::Ident,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            RsxComponentType::Fragment(_) => None,
            RsxComponentType::Intrinsic(tag) => {
                let span = tag.span();
                let value = tag.to_string();
                Some(quote_spanned! {span=>
                    use self::intrinsic_components::#tag::prelude::*;
                    let #builder_ident = self::rsx_runtime_impl_rsx_prop!( #builder_ident .__set_intrinsic_component( #value )  );
                })
            }
            RsxComponentType::TypePath(_) => None,
        }
    }
}

impl ToTokens for RsxElement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let RsxElement {
            component_type,
            key,
            props,
            children,
            start_lt,
        } = self;

        let component_type_span = component_type
            .optional_span()
            .unwrap_or_else(|| start_lt.span());

        let builder_ident = syn::Ident::new("__rsx_props_builder", Span::call_site());

        let pre_build = component_type.intrinsic_pre_build_token_stream(&builder_ident);
        let component_type = component_type.type_to_token_stream(Some(component_type_span));

        let prop_init = quote! {
            let mut #builder_ident = self::rsx_runtime::init_props_builder::< #component_type >();
        };
        let props_chain = props.iter().map(|prop| {
            let RsxProp { name, value } = prop;

            let span = name.span();

            let value = if let Some(value) = value {
                value.to_token_stream()
            } else {
                quote_spanned!(span=>
                    true)
            };
            quote! {
                let #builder_ident = self::rsx_runtime_impl_rsx_prop!( #builder_ident . #name ( #value )  );
            }
        });

        let key_value = if let Some(key) = key {
            let value = &key.value;
            quote!(Some(::frender::AsKey::as_key(&#value)))
        } else {
            quote!(None)
        };

        let children_span_and_value = match children {
            RsxElementChildren::No { .. } => None,
            RsxElementChildren::Yes {
                children, start_gt, ..
            } => {
                if children.is_empty() {
                    None
                } else {
                    let span = start_gt.span();
                    let values = children.iter();
                    let value = if values.len() == 1 {
                        quote_spanned! {span=>
                            ::frender::IntoPropValue::into_prop_value(#(#values),*)
                        }
                    } else {
                        quote_spanned! {span=>
                            ::frender::IntoPropValue::into_prop_value( (#(#values),*) )
                        }
                    };
                    Some((span, value))
                }
            }
        };

        let props_children = if let Some((span, value)) = children_span_and_value {
            Some(quote_spanned! {span=>
                let #builder_ident = self::rsx_runtime_impl_rsx_prop!( #builder_ident . children ( #value )  );
            })
        } else {
            None
        };

        tokens.append_all(quote_spanned! {component_type_span=>
            {
                #prop_init
                self::rsx_runtime::impl_rsx::< #component_type, _ >(
                    {
                        #pre_build
                        #(#props_chain)*
                        #props_children
                        #builder_ident
                    },
                    #key_value,
                )
            }
        });
    }
}

impl ToTokens for RsxChild {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            RsxChild::LitOrBracedExpr(le) => le.to_tokens(tokens),
            RsxChild::Element(el) => el.to_tokens(tokens),
        }
    }
}
