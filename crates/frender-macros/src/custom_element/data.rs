use darling::{util::Flag, FromMeta};
use syn::{parse::Parse, spanned::Spanned};

use crate::err::{OptionCombineExt, OutputError, RecordError};

#[derive(FromMeta, Default)]
pub struct CustomElementOptions {
    /// the module path of `react`
    ///
    /// default to `::frender::react`
    #[darling(default)]
    pub react: Option<syn::Path>,
    #[darling(default)]
    pub no_derive_clone: Flag,
    #[darling(default)]
    pub no_derive_debug: Flag,
    #[darling(default)]
    pub no_from_element: Flag,
}

pub fn default_react_mod_path(span: proc_macro2::Span) -> syn::Path {
    syn::parse_quote_spanned!(span=> ::frender::react)
}

pub struct CustomElementDefinition {
    pub errors: Vec<darling::Error>,
    pub options: CustomElementOptions,
    pub item_struct: CustomElementItemStruct,
}

impl CustomElementDefinition {
    pub fn parse_input(
        args: proc_macro::TokenStream,
        input: proc_macro::TokenStream,
    ) -> darling::Result<CustomElementDefinition> {
        let mut errors = vec![];

        let args = match syn::parse_macro_input::parse::<syn::AttributeArgs>(args) {
            Ok(args) => args,
            Err(error) => {
                errors.record_error(error);
                vec![]
            }
        };

        let options = match CustomElementOptions::from_list(&args) {
            Ok(v) => v,
            Err(error) => {
                errors.record_error(error);
                Default::default()
            }
        };

        let item_struct = match syn::parse::<CustomElementItemStruct>(input) {
            Ok(v) => v,
            Err(error) => return Err(errors.record_and_output(error)),
        };

        Ok(Self {
            errors,
            options,
            item_struct,
        })
    }
}

pub struct CustomElementItemStruct {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub struct_token: syn::Token![struct],
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub fields: syn::FieldsUnnamed,
    pub semi_token: syn::Token![;],
}

impl Parse for CustomElementItemStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let struct_data: syn::ItemStruct = input.parse()?;
        let syn::ItemStruct {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            fields,
            semi_token,
        } = struct_data;

        let mut error = None;

        let semi_token = if let Some(semi_token) = semi_token {
            semi_token
        } else {
            error.record_error(syn::Error::new(ident.span(), "Expect semi `;`"));
            Default::default()
        };

        let fields = match fields {
            syn::Fields::Unnamed(fields) => fields,
            syn::Fields::Unit => syn::FieldsUnnamed {
                paren_token: syn::token::Paren(ident.span()),
                unnamed: syn::punctuated::Punctuated::new(),
            },
            _ => {
                return Err(error.unwrap_combined(syn::Error::new(
                    fields.span(),
                    "Custom element type cannot have named fields.",
                )));
            }
        };

        let ret = Self {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            fields,
            semi_token,
        };

        error.map_or(Ok(ret), Err)
    }
}
