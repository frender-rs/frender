use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
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

    let tokens = quote! {
        #vis struct #{ident}Props #generics {
        }

        #vis struct #ident #generics {
            props: #{ident}Props,
        }

        impl #generics serde::Serialize for SerializeWith #generics #where_clause {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                #path(self.value, serializer)
            }
        }

        SerializeWith {
            value: #value,
            phantom: core::marker::PhantomData::<#item_ty>,
        }
    };

    tokens.into()
}
