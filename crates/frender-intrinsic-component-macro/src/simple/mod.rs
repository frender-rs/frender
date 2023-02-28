use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use super::{Field, FieldDeclaration, FieldDeclarationInherit, IntrinsicComponentPropsData};

mod field_as_simple_prop;
use field_as_simple_prop::*;

impl IntrinsicComponentPropsData {
    pub fn into_ts_simply(self, crate_path: &TokenStream) -> TokenStream {
        let Self {
            attrs,
            vis,
            struct_token: _,
            name,
            dom_element,
            fields,
            inherits,
        } = self;
        let span = name.span();

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
                },
            );

            inherit.into_ts(&crate_path, Self::into_ts_simply)
        });

        let impl_components = dom_element
            .content
            .component_names()
            .map(|component_names| {
                let component_names = component_names.iter();
                quote! {
                    #crate_path::def_intrinsic_component_simple!(
                        type Props = #name;
                        type Element = #dom_element_type;
                        #(
                            #vis struct #component_names;
                        )*
                    );
                }
            });

        let props_inherits = fields
            .iter()
            .filter_map(|f| f.declaration.as_inherit().map(|v| (&f.name, v)))
            .map(|(_name, f)| {
                let from_path = &f.from_path;
                quote! {
                    super::inherit_props_from!(#from_path);
                }
            });
        let props_structs = fields
            .iter()
            .filter(|f| !f.declaration.is_inherit())
            .map(|f| &f.name);
        let props_structs = quote! {
            super::def_props!(
                #( #props_structs , )*
            );
        };

        let mut impl_builder_fn = TokenStream::new();
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
            impl_builder_fn.extend(p.builder_fns);
        }

        quote_spanned! {span=>
            #[allow(non_snake_case)]
            #vis mod #name {
                #(#attrs)*
                #[allow(non_snake_case)]
                #[inline(always)]
                #vis fn #name () -> Building {
                    Building(Default::default())
                }

                pub mod data_struct {
                    #[allow(unused_imports)]
                    use super::super::*;

                    #[derive(Debug, Clone, Copy, Default)]
                    #[repr(transparent)]
                    pub struct #name<
                        Children = #crate_path::frender_html_simple::AllowChildren,
                        Props = (),
                    > {
                        pub props: #crate_path::frender_html_simple::ElementProps<Children, Props>
                    }
                }

                pub mod building_struct {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[repr(transparent)]
                    pub struct #name<
                        Children = #crate_path::frender_html_simple::AllowChildren,
                        Props = (),
                    >(pub super::Data<Children, Props>);
                }

                pub use data_struct::#name as Data;
                pub use building_struct::#name as Building;
                pub type DataInitial = data_struct::#name;
                pub mod prelude {}

                #[inline(always)]
                pub fn build<Children, Props>(building: Building<Children, Props>) -> Data<Children, Props> {
                    building.0
                }
                pub use build as valid;

                pub mod props {
                    #(#props_inherits)*
                    #props_structs
                }

                #[cfg(feature = "dom")]
                mod props_impl_dom {
                    #[allow(unused_imports)]
                    use super::super::*;

                    #props_impl_dom
                }

                #[cfg(feature = "ssr")]
                mod props_impl_ssr {
                    #[allow(unused_imports)]
                    use super::super::*;

                    #props_impl_ssr
                }

                mod builder_and_replacer {
                    #[allow(unused_imports)]
                    use super::super::*;

                    impl<Props> super::Building<
                        #crate_path::frender_html_simple::AllowChildren,
                        Props
                    > {
                        #[inline(always)]
                        pub fn children<Children>(self, children: Children) -> super::Building<Children, Props> {
                            super::Building(super::Data {
                                props: self.0.props.children(children)
                            })
                        }
                    }

                    impl<Children, Props>
                    super::Building<Children, Props> {
                        #impl_builder_fn
                    }
                }

                #[cfg(feature = "dom")]
                impl_dom! {}

                #[cfg(feature = "ssr")]
                impl_ssr! {}

                mod imports {
                    use super::super::*;

                    pub(super) use #crate_path::frender_html_simple::{
                        def_props, inherit_props_from,
                        impl_dom, impl_ssr,
                    };
                }
                use imports::{
                    def_props, inherit_props_from,
                    impl_dom, impl_ssr,
                };
            }

            #vis use #name::#name;

            #impl_components

            #(#inherits_ts)*
        }
    }
}
