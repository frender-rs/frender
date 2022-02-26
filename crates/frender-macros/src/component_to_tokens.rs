use quote::{quote, quote_spanned, ToTokens};

use crate::err::OutputError;

use super::component_data::*;

impl ComponentDefinition {
    pub fn into_ts(self) -> proc_macro2::TokenStream {
        let Self {
            errors,
            options:
                ComponentOptions {
                    //
                    display_name,
                    no_debug_props,
                    main,
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
            props_arg,
            fn_token,
            ..
        } = sig;

        let span = ident.span();
        let component_name = ident.to_string();
        let display_name = display_name.as_ref().unwrap_or(&component_name);

        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        let (props_ty, use_render_arg) = if let Some(props_arg) = props_arg {
            let use_render_arg = props_arg.to_token_stream();
            let pat_type = props_arg.into_value();
            let ty = *pat_type.ty;
            let ty = match ty {
                syn::Type::Reference(ty_ref) => {
                    let syn::TypeReference { elem, .. } = ty_ref;
                    *elem
                }
                _ => ty,
            };
            (ty.into_token_stream(), use_render_arg)
        } else {
            (quote!(::frender::react::NoProps), quote!(_: &Self::RenderArg))
        };

        let (arrow, render_output_ty, custom_element_ty) = match output {
            syn::ReturnType::Default => {
                let ty: syn::Type = syn::parse_quote_spanned!(span=> ::frender::react::Element);
                (Default::default(), ty.clone(), ty)
            }
            syn::ReturnType::Type(arrow, ty) => {
                let render_output_ty = *ty;
                let ty = infer_custom_element_ty(&render_output_ty);
                (arrow, render_output_ty, ty)
            }
        };

        let debugs = if cfg!(debug_assertions) && no_debug_props.is_none() {
            quote_spanned! {span=>
                let debug_component_name = Some(JsValue::from_str( #display_name ));
                let debug_props = ::frender::auto_debug_props!(props);
            }
        } else {
            quote_spanned! {span=>
                let debug_component_name = None;
                let debug_props = None;
            }
        };

        let component_struct = if generics.params.is_empty() {
            quote_spanned! {span=>
                #[derive(Debug, Clone, Copy)]
                #vis struct #ident #impl_generics #where_clause;
            }
        } else {
            let (debug_arg1, debug_arg2) = {
                let debug_arg1 = "{}, ".repeat(generics.params.len());
                let debug_arg1 = format!("<{}>", debug_arg1);

                let tps = generics.params.iter();
                let debug_arg2 = quote_spanned!(span=> #( ::std::any::type_name::<#tps>() ),* );
                (debug_arg1, debug_arg2)
            };

            quote_spanned! {span=>
                #[derive(Debug, Clone, Copy)]
                #vis struct #ident #impl_generics #where_clause {
                    inner: ::core::marker::PhantomData<#type_generics>
                }

                impl #impl_generics ::core::fmt::Debug for #ident #type_generics #where_clause {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        write!(f, #debug_arg1, #debug_arg2)
                    }
                }

                impl #impl_generics Clone for #ident #type_generics #where_clause {
                    fn clone(&self) -> Self {
                        Self {
                            inner: ::core::marker::PhantomData,
                        }
                    }
                }

                impl #impl_generics Copy for #ident #type_generics #where_clause {}
            }
        };

        let main_block = if let Some(main) = main {
            let span = main.span();
            let ComponentMainOptions {
                //
                no_strict_mode,
                mount_element_id,
            } = main.as_ref();

            let wrap_strict_mode = if no_strict_mode.is_some() {
                None
            } else {
                let ts = quote_spanned! {span=>
                    let frender_element = <::frender::react::StrictMode as ::frender::react::ComponentStatic>::create_element(
                        ::frender::react::StrictModeProps {
                            children: ::frender::react::Node::into_react_children_js(frender_element),
                        },
                        None,
                    );
                };
                Some(ts)
            };

            let ts = quote_spanned! {span=>
                #vis #fn_token main() {
                    let frender_element = <#ident as ::frender::react::ComponentStatic>::create_element(::frender::react::NoProps, None);
                    #wrap_strict_mode
                    ::frender::react::render_into_dom_by_id(frender_element, #mount_element_id);
                }
            };

            Some(ts)
        } else {
            None
        };

        let errors = errors.output_error().map(darling::Error::write_errors);
        quote_spanned! {span=>
            #component_struct

            impl #impl_generics ::frender::react::UseRenderStatic for #ident #type_generics #where_clause {
                type RenderArg = #props_ty;
                type RenderOutput = #render_output_ty;

                fn use_render(#use_render_arg) #arrow #render_output_ty #block
            }

            impl #impl_generics ::frender::react::ComponentStatic for #ident #type_generics #where_clause {
                type Props = #props_ty;
                type Element = #custom_element_ty;

                fn create_element(props: Self::Props, key: Option<::frender::react::Key>) -> Self::Element {
                    #[allow(unused_imports)]
                    use frender::react::UseRenderStatic;
                    #[allow(unused_imports)]
                    use frender::react::IntoOptionalElement;
                    #debugs
                    Self::Element::private_from_element(::frender::react::UseRenderElement::wrap_use_render(
                        move || Self::use_render(&props).into_optional_element(),
                        key,
                        debug_component_name,
                        debug_props,
                    ).into())
                }
            }

            #main_block

            #errors
        }
    }
}

/// For `render_output_ty` = `Option<MyElement>`, extract `MyElement`.
/// Otherwise clone `render_output_ty`
fn infer_custom_element_ty(render_output_ty: &syn::Type) -> syn::Type {
    if let syn::Type::Path(path_ty) = render_output_ty {
        let p = &path_ty.path;
        if p.leading_colon.is_none() {
            if p.segments.len() == 1 {
                let seg = &p.segments[0];
                if seg.ident == "Option" {
                    if let syn::PathArguments::AngleBracketed(bracketed_args) = &seg.arguments {
                        if bracketed_args.args.len() == 1 {
                            if let syn::GenericArgument::Type(ty) = &bracketed_args.args[0] {
                                return ty.clone();
                            }
                        }
                    }
                }
            }
        }
    }

    render_output_ty.clone()
}
