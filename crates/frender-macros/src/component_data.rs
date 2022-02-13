use darling::FromMeta;
use syn::{punctuated::Pair, spanned::Spanned};

use crate::err::{
    maybe_with_error, OptionCombineExt, OutputError, ResultUnwrapValueAndErrorExt,
    ResultUnwrapValueExt,
};

#[derive(Debug, FromMeta, Default)]
pub struct ComponentOptions {
    #[darling(default)]
    pub display_name: Option<String>,
    // path: String,
}

pub struct ComponentDefinition {
    pub options: ComponentOptions,
    pub item_fn: ComponentItemFn,
}

impl ComponentDefinition {
    pub fn try_from_attrs_and_fn(
        attr_args: syn::AttributeArgs,
        input_fn: syn::ItemFn,
    ) -> Result<Self, (Self, darling::Error)> {
        let mut errors = vec![];
        let options = match ComponentOptions::from_list(&attr_args) {
            Ok(v) => v,
            Err(err) => {
                errors.push(err);
                ComponentOptions::default()
            }
        };

        let item_fn = ComponentItemFn::try_from_input_fn(input_fn)
            .unwrap_value_and_record_error(&mut errors)
            .unwrap_value();

        let ret = Self { options, item_fn };

        errors.into_value_result(ret)
    }
}

pub struct ComponentItemFn {
    // pub attrs: Vec<Attribute>,
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

        for attr in attrs {
            error.combine_or_replace(syn::Error::new_spanned(
                attr,
                "frender component doesn't support attrs yet",
            ));
        }

        let sig = ComponentItemFnSignature::try_from_sig(sig)
            .unwrap_value_and_record_error(&mut error)
            .unwrap_value();

        let ret = Self { vis, sig, block };

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
            generics,
            ident,
            output,
            mut inputs,
            fn_token,
            paren_token,
            ..
        } = sig;
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
                            "`self` argument is not allowed in frender component fn",
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
