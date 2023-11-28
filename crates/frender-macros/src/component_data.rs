use darling::{util::WithOriginal, FromMeta};
use syn::{punctuated::Pair, spanned::Spanned};

use crate::{
    bg::BgOptions,
    err::{
        maybe_with_error, OptionCombineExt, OutputError, RecordError, ResultUnwrapValueAndErrorExt,
        ResultUnwrapValueExt,
    },
    utils::value_or_path::ValueOrPath,
};

#[derive(Debug, FromMeta)]
pub struct ComponentMainOptions {
    pub get_dom_element: WithOriginal<ValueOrPath<syn::LitStr>, syn::Meta>,
}

pub type MainOptionsWithOriginal = darling::util::WithOriginal<ComponentMainOptions, syn::Meta>;

#[derive(Debug, FromMeta, Default)]
pub struct ComponentOptions {
    #[darling(default)]
    pub main: Option<MainOptionsWithOriginal>,
    /// Defaults to `::frender::hook_element`
    pub hook_element_path: Option<syn::Path>,
    pub ssr_only: darling::util::Flag,
    /// use `FnOnce` instead of `FnMut`
    #[darling(rename = "FnOnce")]
    pub use_fn_once: darling::util::Flag,
    pub bg: Option<BgOptions>,
}

pub struct ComponentDefinition {
    pub errors: Vec<darling::Error>,
    pub options: ComponentOptions,
    pub item_fn: syn::ItemFn,
}

impl ComponentDefinition {
    pub fn from_attrs_and_fn(attr_args: syn::AttributeArgs, item_fn: syn::ItemFn) -> Self {
        let mut errors = vec![];
        let options = match ComponentOptions::from_list(&attr_args) {
            Ok(v) => v,
            Err(err) => {
                errors.push(err);
                ComponentOptions::default()
            }
        };

        Self {
            options,
            item_fn,
            errors,
        }
    }
}

pub struct ComponentItemFn {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub sig: ComponentItemFnSignature,
    pub block: Box<syn::Block>,
}

pub struct ComponentItemFnSignature {
    pub fn_token: syn::Token![fn],
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub paren_token: syn::token::Paren,
    pub props_arg: Option<Pair<syn::PatType, syn::Token![,]>>,
    pub output: syn::ReturnType,
}

impl ComponentItemFn {
    pub fn try_from_input_fn(input_fn: syn::ItemFn) -> Result<Self, (Self, syn::Error)> {
        let syn::ItemFn {
            vis,
            sig,
            block,
            attrs,
            ..
        } = input_fn;

        let mut error = None;

        let sig = ComponentItemFnSignature::try_from_sig(sig)
            .unwrap_value_and_record_error(&mut error)
            .unwrap_value();

        let ret = Self {
            attrs,
            vis,
            sig,
            block,
        };

        error.into_value_result(ret)
    }
}

macro_rules! check_sig_field_is_none {
    ($error:ident $msg:literal  = $sig:ident . $field:ident) => {
        if let Some($field) = &$sig.$field {
            $crate::err::OptionCombineExt::combine_or_replace(
                $error,
                syn::Error::new(
                    $field.span(),
                    concat!("frender component doesn't support `", $msg, "` fn"),
                ),
            );
        }
    };
}

impl ComponentItemFnSignature {
    pub fn try_from_sig(sig: syn::Signature) -> Result<Self, (Self, syn::Error)> {
        let mut error = None;

        let error_mut = &mut error;
        check_sig_field_is_none!(error_mut "const" = sig.constness);
        check_sig_field_is_none!(error_mut "async" = sig.asyncness);
        check_sig_field_is_none!(error_mut "unsafe" = sig.unsafety);
        check_sig_field_is_none!(error_mut "extern" = sig.abi);
        check_sig_field_is_none!(error_mut "variadic" = sig.variadic);

        let syn::Signature {
            mut generics,
            ident,
            output,
            mut inputs,
            fn_token,
            paren_token,
            ..
        } = sig;

        if generics.params.first().map_or(false, |tp| match tp {
            syn::GenericParam::Lifetime(_) => true,
            _ => false,
        }) {
            generics.params = generics
                .params
                .into_pairs()
                .filter(|p| match p.value() {
                    syn::GenericParam::Lifetime(lt) => {
                        error.record_error(syn::Error::new(
                            lt.span(),
                            "frender component currently does not support lifetimes",
                        ));
                        false
                    }
                    _ => true,
                })
                .collect();
        }

        let arg_props = inputs.pop();

        for other_arg in inputs {
            error.combine_or_replace(syn::Error::new(
                other_arg.span(),
                "frender component fn expects at most one argument as props",
            ));
        }

        let props_arg = if let Some((arg_props, comma)) = arg_props.map(Pair::into_tuple) {
            match arg_props {
                syn::FnArg::Receiver(v) => {
                    error.combine_or_replace(syn::Error::new(
                        v.span(),
                        "`self` argument is not allowed in frender component fn",
                    ));
                    None
                }
                syn::FnArg::Typed(v) => {
                    for attr in &v.attrs {
                        error.combine_or_replace(syn::Error::new(
                            attr.span(),
                            "attributes are not allowed for arguments of frender component fn",
                        ));
                    }
                    Some(Pair::new(v, comma))
                }
            }
        } else {
            None
        };

        let sig = Self {
            fn_token,
            ident,
            generics,
            paren_token,
            props_arg,
            output,
        };

        maybe_with_error(sig, error)
    }
}
