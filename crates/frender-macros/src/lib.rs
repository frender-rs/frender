mod case;

use darling::FromMeta;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs, ItemFn, Signature};

#[derive(Debug, FromMeta)]
struct ComponentOptions {
    #[darling(default)]
    display_name: Option<String>,
    path: String,
}

#[proc_macro_attribute]
pub fn component(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let input_fn = parse_macro_input!(input as ItemFn);

    let opts = match ComponentOptions::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let ItemFn {
        vis, sig, block, ..
    } = input_fn;

    if sig.constness.is_some()
        || sig.asyncness.is_some()
        || sig.unsafety.is_some()
        || sig.abi.is_some()
    {
        return quote_spanned! {
            sig.span() =>
              compiler_error!("frender component doesn't support `const` / `async` / `unsafe` / `extern` fn")
        }
        .into();
    };

    let Signature {
        generics,
        ident,
        output,
        inputs,
        ..
    } = sig;

    todo!();

    // let tokens = quote! {
    //     #vis struct #{ident}Props #generics {
    //     }

    //     #vis struct #ident #generics {
    //         props: #{ident}Props,
    //     }

    //     impl #generics serde::Serialize for SerializeWith #generics #where_clause {
    //         fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    //         where
    //             S: serde::Serializer,
    //         {
    //             #path(self.value, serializer)
    //         }
    //     }

    //     SerializeWith {
    //         value: #value,
    //         phantom: core::marker::PhantomData::<#item_ty>,
    //     }
    // };

    // tokens.into()
}

#[proc_macro]
pub fn snake_to_camel(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as syn::LitStr);
    let lit = case::snake_to_camel_lit_str(&lit.value(), lit.span());
    lit.to_token_stream().into()
}

#[proc_macro]
pub fn ident_snake_to_camel(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as syn::Ident);
    let lit = case::snake_to_camel_lit_str(&ident.to_string(), ident.span());
    lit.to_token_stream().into()
}
