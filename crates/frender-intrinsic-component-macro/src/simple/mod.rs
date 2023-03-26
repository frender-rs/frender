use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use super::{Field, FieldDeclaration, FieldDeclarationInherit, IntrinsicComponentPropsData};

mod field_as_simple_prop;
use field_as_simple_prop::*;

impl IntrinsicComponentPropsData {
    pub fn into_ts_simply(self, crate_path: &TokenStream) -> TokenStream {
        let Self {
            attrs: mut fn_attrs,
            vis,
            struct_token: _,
            name,
            dom_element,
            fields,
            inherits,
        } = self;
        let span = name.span();

        for fn_attr in &mut fn_attrs {
            assert!(matches!(fn_attr.style, syn::AttrStyle::Outer));

            fn_attr.style = syn::AttrStyle::Inner(syn::Token![!](span));
        }

        let dom_element_type = &dom_element.content.ty;

        let fields = fields.content.0;

        let inherits_ts = inherits.into_iter().map(|inherit| {
            let mut inherit = inherit.content;

            let name = name.clone();
            inherit.fields_mut().0.insert(
                0,
                Field {
                    attrs: vec![],
                    name: name.clone(),
                    declaration: FieldDeclaration::Inherit(FieldDeclarationInherit {
                        from_path: name.into(),
                        dom_element_ty: dom_element_type.clone(),
                        fields: fields.iter().map(Clone::clone).collect(),
                    }),
                    options: Default::default(),
                },
            );

            inherit.into_ts(&crate_path, Self::into_ts_simply)
        });

        let impl_components = dom_element
            .content
            .component_names()
            .map(|component_names| {
                let component_names = component_names.iter().map(|c| {
                    let component_name = &c.component_name;
                    let options_or_semi = c
                        .options
                        .as_ref()
                        .map_or_else(|| quote!(;), darling::ToTokens::to_token_stream);

                    quote!(#component_name #options_or_semi)
                });
                quote! {
                    #crate_path::def_intrinsic_component_simple!(
                        type Props = #name;
                        type Element = #dom_element_type;
                        #(
                            #vis struct #component_names
                        )*
                    );
                }
            });

        let mut all_field_info = TokenStream::new();
        let mut props_impl_dom = TokenStream::new();
        let mut props_impl_ssr = TokenStream::new();

        for p in fields.iter().map(|field| {
            FieldAsSimpleProp {
                crate_path,
                field,
                dom_element_ty: dom_element_type,
            }
            .into_simple_prop()
        }) {
            props_impl_dom.extend(p.impl_dom);
            props_impl_ssr.extend(p.impl_ssr);
            all_field_info.extend(p.field_info);
        }

        quote_spanned! {span=>
            #[allow(non_snake_case)]
            #vis mod #name {
                def_props_type!(
                    #(#fn_attrs)*
                    #[derive(Debug, Clone, Copy, Default)]
                    #name (
                        #all_field_info
                    )
                );

                #[cfg(feature = "dom")]
                mod impl_dom_for_props {
                    #![allow(unused_variables)]
                    #[allow(unused_imports)]
                    use super::super::*;

                    #props_impl_dom
                }

                #[cfg(feature = "ssr")]
                mod impl_ssr_for_props {
                    #![allow(unused_variables)]
                    #[allow(unused_imports)]
                    use super::super::*;

                    #props_impl_ssr
                }

                mod imports {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub(super) use #crate_path::frender_html_simple::def_props_type;
                }
                use imports::def_props_type;
            }

            #vis use #name::#name;

            #impl_components

            #(#inherits_ts)*
        }
    }
}
