#[inline(always)]
pub fn li() -> Building<TypesInitial> {
    super::HtmlLiElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlLiElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "li";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct li<
        TypeDefs: ?::core::marker::Sized + HtmlLiElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(pub HtmlLiElementProps::Data<TypeDefs>, pub ComponentType);
}
pub use struct_data::li as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline(always)]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlLiElementProps::build(building), self::ComponentType)
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
            TypeDefs: ?::core::marker::Sized + li::Types,
            ComponentType: crate::props::IntrinsicComponent,
        > ::frender_core::UpdateRenderState<::frender_dom::Dom>
        for li::Data<TypeDefs, ComponentType>
    where
        HtmlLiElementProps::Data<TypeDefs>: crate::props::UpdateElement<web_sys::HtmlLiElement>,
    {
        type State = crate::props::IntrinsicComponentRenderState<
            web_sys::HtmlLiElement,
            <HtmlLiElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                web_sys::HtmlLiElement,
            >>::State,
        >;
        fn initialize_render_state(self, ctx: &mut ::frender_dom::Dom) -> Self::State {
            Self::State::initialize_with_tag(self.0, ctx, ComponentType::INTRINSIC_TAG)
        }
        fn update_render_state(
            self,
            ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let (node_and_mounted, state) = state.pin_project();
            crate::utils::dom::update_element(node_and_mounted, ctx, |element, children_ctx| {
                <HtmlLiElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                    web_sys::HtmlLiElement,
                >>::update_element(self.0, element, children_ctx, state)
            })
        }
    }
}
#[cfg(feature = "ssr")]
mod impl_update_render_state_ssr {
    use super::super::*;
    impl<
            TypeDefs: ?::core::marker::Sized + li::Types,
            ComponentType: crate::props::IntrinsicComponent,
            W: ::frender_ssr::AsyncWrite + ::core::marker::Unpin,
        > ::frender_core::UpdateRenderState<::frender_ssr::SsrContext<W>>
        for li::Data<TypeDefs, ComponentType>
    where
        HtmlLiElementProps::Data<TypeDefs>: ::frender_ssr::IntoSsrData<W>,
    {
        type State = ::frender_ssr::element::html::HtmlElementRenderState<
            'static,
            <<HtmlLiElementProps::Data<
                TypeDefs,
            > as ::frender_ssr::IntoSsrData<
                W,
            >>::Children as ::frender_core::UpdateRenderState<
                ::frender_ssr::SsrContext<W>,
            >>::State,
            <HtmlLiElementProps::Data<TypeDefs> as ::frender_ssr::IntoSsrData<W>>::Attrs,
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
            ::frender_ssr::element::html::HtmlElement {
                tag: ComponentType::INTRINSIC_TAG.into(),
                attributes,
                children: ::frender_ssr::element::html::HtmlElementChildren::Children(children),
            }
            .update_render_state(ctx, state)
        }
    }
}
