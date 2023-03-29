#[cfg(feature = "fully-typed")]
macro_rules! def_intrinsic_component {
    (
        $vis:vis
        $component_name:ident
        [$($alias_component_name:ident)*]
        $props_name:ident
        $dom_element_ty:ty
    ) => {
        $vis mod $component_name {
            #[inline(always)]
            pub fn $component_name (
            ) -> Building<TypesInitial> {
                super::$props_name ()
            }

            mod reuse {
                use super::super::*;
                pub use $props_name ::{
                    prelude, Building, Types, TypesInitial, ValidTypes,
                };
            }

            pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};

            pub struct ComponentType;

            impl crate::imports::frender_html::props::IntrinsicComponent for ComponentType {
                const INTRINSIC_TAG: &'static ::core::primitive::str = ::core::stringify!($component_name);
            }

            mod struct_data {
                use super::super::*;
                #[allow(non_camel_case_types)]
                pub struct $component_name <
                    TypeDefs: ?::core::marker::Sized + $props_name ::Types,
                    ComponentType: crate::imports::frender_html::props::IntrinsicComponent = super::ComponentType,
                >(
                    pub $props_name ::Data<TypeDefs>,
                    pub ComponentType,
                );
            }

            pub use struct_data::$component_name as Data;

            pub type DataInitial = Data<TypesInitial>;

            #[inline(always)]
            pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
                building: Building<TypeDefs>,
            ) -> Data<TypeDefs> {
                use super::*;
                self::Data($props_name ::build(building), self::ComponentType)
            }

            pub use build as build_element;

            #[inline(always)]
            pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
                building: Building<TypeDefs>,
            ) -> Data<TypeDefs> {
                build(building)
            }

            #[cfg(feature = "csr")]
            mod impl_update_render_state_dom {
                use super::super::*;
                impl<
                    TypeDefs: ?::core::marker::Sized + $component_name::Types,
                    ComponentType: crate::imports::frender_html::props::IntrinsicComponent,
                >
                    ::frender_core::UpdateRenderState<::frender_csr::Dom>
                    for $component_name::Data<TypeDefs, ComponentType>
                where
                    $props_name::Data<TypeDefs>: ::frender_csr::props::UpdateElement<$dom_element_ty>,
                {
                    type State =
                        ::frender_csr::element::intrinsic::IntrinsicComponentRenderState<
                            $dom_element_ty,
                            <$props_name::Data<TypeDefs> as ::frender_csr::props::UpdateElement<
                                $dom_element_ty,
                            >>::State,
                        >;

                    fn initialize_render_state(self, ctx: &mut ::frender_csr::Dom) -> Self::State {
                        Self::State::initialize_with_tag(
                            self.0,
                            ctx,
                            ComponentType::INTRINSIC_TAG,
                        )
                    }

                    fn update_render_state(
                        self,
                        ctx: &mut ::frender_csr::Dom,
                        state: ::core::pin::Pin<&mut Self::State>,
                    ) {
                        let (node_and_mounted, state) = state.pin_project();
                        node_and_mounted.update(
                            ctx,
                            |element, children_ctx| {
                                <$props_name::Data<TypeDefs> as ::frender_csr::props::UpdateElement<
                                    $dom_element_ty,
                                >>::update_element(self.0, element, children_ctx, state)
                            },
                        )
                    }
                }
            }

            #[cfg(feature = "ssr")]
            mod impl_update_render_state_ssr {
                use super::super::*;
                impl<
                        TypeDefs: ?::core::marker::Sized + $component_name::Types,
                        ComponentType: crate::imports::frender_html::props::IntrinsicComponent,
                        W: ::frender_ssr::AsyncWrite + ::core::marker::Unpin,
                    > ::frender_core::UpdateRenderState<::frender_ssr::SsrContext<W>>
                    for $component_name::Data<TypeDefs, ComponentType>
                where
                    $props_name::Data<TypeDefs>: ::frender_ssr::IntoSsrData<W>,
                {
                    type State = ::frender_ssr::element::html::HtmlElementRenderState<
                        'static,
                        <<$props_name::Data<TypeDefs> as ::frender_ssr::IntoSsrData<W>>::Children as
                        ::frender_core::UpdateRenderState<::frender_ssr::SsrContext<W>>
                        >::State,
                        <$props_name::Data<TypeDefs> as ::frender_ssr::IntoSsrData<W>>::Attrs,
                        W,
                    >;

                    fn initialize_render_state(self, ctx: &mut ::frender_ssr::SsrContext<W>) -> Self::State {
                        let (children, attributes) = ::frender_ssr::IntoSsrData::<W>::into_ssr_data(self.0);
                        ::frender_ssr::element::html::HtmlElement {
                            tag: ComponentType::INTRINSIC_TAG.into(),
                            attributes,
                            children: ::frender_ssr::element::html::HtmlElementChildren::Children(children),
                        }
                        .initialize_render_state(ctx)
                    }

                    fn update_render_state(
                        self,
                        ctx: &mut ::frender_ssr::SsrContext<W>,
                        state: std::pin::Pin<&mut Self::State>,
                    ) {
                        let (children, attributes) = ::frender_ssr::IntoSsrData::<W>::into_ssr_data(self.0);
                        // let children =  ::frender_core::UpdateRenderState::<::frender_ssr::SsrContext<W>>::initialize_render_state(children, ctx);
                        ::frender_ssr::element::html::HtmlElement {
                            tag: ComponentType::INTRINSIC_TAG.into(),
                            attributes,
                            children: ::frender_ssr::element::html::HtmlElementChildren::Children(children),
                        }
                        .update_render_state(ctx, state)
                    }
                }
            }
        }

        $vis use $component_name::$component_name;

        $(
            $vis mod $alias_component_name {
                #[inline(always)]
                pub fn $alias_component_name (
                ) -> Building<TypesInitial> {
                    super::$props_name ()
                }

                pub use super::$component_name::{prelude, Building, Types, TypesInitial, ValidTypes};

                pub struct ComponentType;

                impl crate::imports::frender_html::props::IntrinsicComponent for ComponentType {
                    const INTRINSIC_TAG: &'static ::core::primitive::str = ::core::stringify!($alias_component_name);
                }

                pub type Data<TypeDefs> = super::$component_name::Data<TypeDefs, ComponentType>;

                pub type DataInitial = Data<TypesInitial>;

                #[inline(always)]
                pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
                    building: Building<TypeDefs>,
                ) -> Data<TypeDefs> {
                    use super::*;
                    super::$component_name::Data($props_name ::build(building), self::ComponentType)
                }

                pub use build as build_element;

                #[inline(always)]
                pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
                    building: Building<TypeDefs>,
                ) -> Data<TypeDefs> {
                    build(building)
                }
            }

            $vis use $alias_component_name::$alias_component_name;
        )*
    };
}

#[cfg(feature = "simply-typed")]
mod simple {
    macro_rules! def_intrinsic_component_simple {
        (
            type Props = $props_name:ident;
            $item_type_element:item

            $(
                $vis:vis struct $component_name:ident
                $component_options_or_semi:tt
            )*
        ) => {$(
            crate::element_macros::def_intrinsic_component_simple_one! {
                $props_name
                $item_type_element
                $vis
                $component_name
                $component_options_or_semi
            }
        )*};
    }

    macro_rules! def_intrinsic_component_simple_one {
        (
            $props_name:ident
            $item_type_element:item
            $vis:vis
            $component_name:ident
            $component_options_or_semi:tt
        ) => {
            $vis mod $component_name {
                pub type Data<Children, Props> = ::frender_html_simple::IntrinsicElement<
                    ComponentType,
                    super::$props_name::Data<Children, Props>,
                >;

                pub use super::$props_name::{
                    prelude, Building,
                };

                mod struct_component_type {
                    pub struct $component_name;

                    impl crate::imports::frender_html::props::IntrinsicComponent for $component_name {
                        const INTRINSIC_TAG: &'static ::core::primitive::str = ::core::stringify!($component_name);
                    }

                    crate::element_macros::def_intrinsic_component_simple_with_children! {
                        $component_options_or_semi
                        $component_name
                    }
                }
                pub use struct_component_type::$component_name as ComponentType;

                #[cfg(feature = "csr")]
                mod impl_dom {
                    #[allow(unused_imports)]
                    use super::super::*;

                    impl crate::imports::frender_html_simple::DomIntrinsicComponent for super::ComponentType {
                        $item_type_element
                    }
                }

                #[cfg(feature = "ssr")]
                mod impl_ssr {
                    #[allow(unused_imports)]
                    use super::super::*;

                    impl crate::imports::frender_html_simple::SsrIntrinsicComponent for super::ComponentType {
                        // TODO: some components are void or self closing
                    }
                }

                #[inline(always)]
                pub fn build<Children, Props>(
                    building: Building<Children, Props>,
                ) -> Data<Children, Props> {
                    use super::*;
                    ::frender_html_simple::IntrinsicElement(
                        self::ComponentType,
                        $props_name ::build(building),
                    )
                }

                pub use build as build_element;
                pub use build as valid;

                pub use super::$props_name as Props;
            }

            #[inline(always)]
            $vis fn $component_name (
            ) -> $props_name::Building {
                $props_name ()
            }
        };
    }

    macro_rules! def_intrinsic_component_simple_with_children {
        (; $component_name:ident) => {
            ::frender_html_simple::impl_intrinsic_element_with_any_children! {$component_name}
        };
        (
            {
                special_children: __ $(,)?
            }
            $component_name:ident
        ) => {};
    }

    pub(crate) use def_intrinsic_component_simple;
    pub(crate) use def_intrinsic_component_simple_one;
    pub(crate) use def_intrinsic_component_simple_with_children;
}

#[cfg(feature = "fully-typed")]
pub(crate) use def_intrinsic_component;
#[cfg(feature = "simply-typed")]
pub(crate) use simple::*;
