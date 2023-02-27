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

            #[cfg(feature = "dom")]
            mod impl_update_render_state_dom {
                use super::super::*;
                impl<
                    TypeDefs: ?::core::marker::Sized + $component_name::Types,
                    ComponentType: crate::imports::frender_html::props::IntrinsicComponent,
                >
                    ::frender_core::UpdateRenderState<::frender_dom::Dom>
                    for $component_name::Data<TypeDefs, ComponentType>
                where
                    $props_name::Data<TypeDefs>: ::frender_dom::props::UpdateElement<$dom_element_ty>,
                {
                    type State =
                        ::frender_dom::element::intrinsic::IntrinsicComponentRenderState<
                            $dom_element_ty,
                            <$props_name::Data<TypeDefs> as ::frender_dom::props::UpdateElement<
                                $dom_element_ty,
                            >>::State,
                        >;

                    fn initialize_render_state(self, ctx: &mut ::frender_dom::Dom) -> Self::State {
                        Self::State::initialize_with_tag(
                            self.0,
                            ctx,
                            ComponentType::INTRINSIC_TAG,
                        )
                    }

                    fn update_render_state(
                        self,
                        ctx: &mut ::frender_dom::Dom,
                        state: ::core::pin::Pin<&mut Self::State>,
                    ) {
                        let (node_and_mounted, state) = state.pin_project();
                        node_and_mounted.update(
                            ctx,
                            |element, children_ctx| {
                                <$props_name::Data<TypeDefs> as ::frender_dom::props::UpdateElement<
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

pub(crate) use def_intrinsic_component;