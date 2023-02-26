#[inline(always)]
pub fn link() -> Building<TypesInitial> {
    super::HtmlLinkElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlLinkElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::imports::frender_html::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "link";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct link<
        TypeDefs: ?::core::marker::Sized + HtmlLinkElementProps::Types,
        ComponentType: crate::imports::frender_html::props::IntrinsicComponent
            = super::ComponentType,
    >(
        pub HtmlLinkElementProps::Data<TypeDefs>,
        pub ComponentType,
    );
}
pub use struct_data::link as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline(always)]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlLinkElementProps::build(building), self::ComponentType)
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
            TypeDefs: ?::core::marker::Sized + link::Types,
            ComponentType: crate::imports::frender_html::props::IntrinsicComponent,
        > ::frender_core::UpdateRenderState<::frender_dom::Dom>
        for link::Data<TypeDefs, ComponentType>
    where
        HtmlLinkElementProps::Data<TypeDefs>:
            ::frender_dom::props::UpdateElement<web_sys::HtmlLinkElement>,
    {
        type State = ::frender_dom::element::intrinsic::IntrinsicComponentRenderState<
            web_sys::HtmlLinkElement,
            <HtmlLinkElementProps::Data<TypeDefs> as ::frender_dom::props::UpdateElement<
                web_sys::HtmlLinkElement,
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
            node_and_mounted.update(ctx, |element, children_ctx| {
                <HtmlLinkElementProps::Data<TypeDefs> as ::frender_dom::props::UpdateElement<
                    web_sys::HtmlLinkElement,
                >>::update_element(self.0, element, children_ctx, state)
            })
        }
    }
}
#[cfg(feature = "ssr")]
mod impl_update_render_state_ssr {
    use super::super::*;
    impl<
            TypeDefs: ?::core::marker::Sized + link::Types,
            ComponentType: crate::imports::frender_html::props::IntrinsicComponent,
            W: ::frender_ssr::AsyncWrite + ::core::marker::Unpin,
        > ::frender_core::UpdateRenderState<::frender_ssr::SsrContext<W>>
        for link::Data<TypeDefs, ComponentType>
    where
        HtmlLinkElementProps::Data<TypeDefs>: ::frender_ssr::IntoSsrData<W>,
    {
        type State = ::frender_ssr::element::html::HtmlElementRenderState<
            'static,
            <<HtmlLinkElementProps::Data<
                TypeDefs,
            > as ::frender_ssr::IntoSsrData<
                W,
            >>::Children as ::frender_core::UpdateRenderState<
                ::frender_ssr::SsrContext<W>,
            >>::State,
            <HtmlLinkElementProps::Data<
                TypeDefs,
            > as ::frender_ssr::IntoSsrData<W>>::Attrs,
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
