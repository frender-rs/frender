use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

use super::component_data::*;

impl ToTokens for ComponentDefinition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            options:
                ComponentOptions {
                    //
                    display_name,
                    no_debug_props,
                },
            item_fn:
                ComponentItemFn {
                    //
                    vis,
                    sig,
                    block,
                },
        } = self;

        let ComponentItemFnSignature {
            generics,
            ident,
            output,
            ..
        } = sig;

        let span = ident.span();
        let component_name = ident.to_string();
        let display_name = display_name.as_ref().unwrap_or(&component_name);

        let props_ty = sig.props_ty();
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        let (component_struct_data, component_init) = props_ty.map_or((None, None), |props_ty| {
            (Some(quote!( (#props_ty) )), Some(quote!((props))))
        });

        let props_ty = props_ty.map_or_else(
            || quote!(::frender::react::NoProps),
            ToTokens::to_token_stream,
        );

        let (arrow, element_ty) = match output {
            syn::ReturnType::Default => {
                (Default::default(), quote!(::frender::react::sys::Element))
            }
            syn::ReturnType::Type(arrow, ty) => (arrow.clone(), ty.to_token_stream()),
        };

        #[cfg(debug_assertions)]
        let debugs = quote_spanned! {span=>
            let debug_component_name = JsValue::from_str( #display_name );
            let debug_component_name = Some(&debug_component_name);
            let debug_props = ::frender::DebugProps::as_debug_props(&self.0);
            let debug_props = debug_props.as_ref();
        };
        #[cfg(not(debug_assertions))]
        let debugs = quote_spanned! {span=>
            let debug_component_name = None;
            let debug_props = None;
        };

        tokens.append_all( quote_spanned! {span=>
            #vis struct #ident #impl_generics #component_struct_data #where_clause;

            impl #impl_generics ::frender::react::Component for MyComponent #type_generics {
                type Props = #props_ty;
                type ElementType = #element_ty;

                fn use_render(&self) #arrow #element_ty #block

                fn new_with_props(props: Self::Props) -> Self {
                    Self #component_init
                }

                fn call_create_element(self, key: Option<&::frender::react::__private::JsValue>) -> ::frender::react::sys::Element {
                    #debugs
                    react::bridge_rust_only_component(
                        move || self.use_render(),
                        key,
                        debug_component_name,
                        debug_props,
                    )
                }
            }

        });
    }
}
