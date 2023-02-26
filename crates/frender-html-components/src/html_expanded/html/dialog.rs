#[inline(always)]
pub fn dialog() -> Building<TypesInitial> {
    super::HtmlDialogElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlDialogElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::imports::frender_html::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "dialog";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct dialog<
        TypeDefs: ?::core::marker::Sized + HtmlDialogElementProps::Types,
        ComponentType: crate::imports::frender_html::props::IntrinsicComponent
            = super::ComponentType,
    >(
        pub HtmlDialogElementProps::Data<TypeDefs>,
        pub ComponentType,
    );
}
pub use struct_data::dialog as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline(always)]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlDialogElementProps::build(building), self::ComponentType)
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
            TypeDefs: ?::core::marker::Sized + dialog::Types,
            ComponentType: crate::imports::frender_html::props::IntrinsicComponent,
        > ::frender_core::UpdateRenderState<::frender_dom::Dom>
        for dialog::Data<TypeDefs, ComponentType>
    where
        HtmlDialogElementProps::Data<TypeDefs>:
            ::frender_dom::props::UpdateElement<web_sys::HtmlDialogElement>,
    {
        type State = ::frender_dom::element::intrinsic::IntrinsicComponentRenderState<
            web_sys::HtmlDialogElement,
            <HtmlDialogElementProps::Data<TypeDefs> as ::frender_dom::props::UpdateElement<
                web_sys::HtmlDialogElement,
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
                <HtmlDialogElementProps::Data<TypeDefs> as ::frender_dom::props::UpdateElement<
                    web_sys::HtmlDialogElement,
                >>::update_element(self.0, element, children_ctx, state)
            })
        }
    }
}
#[cfg(feature = "ssr")]
mod impl_update_render_state_ssr {
    use super::super::*;
    impl<
            TypeDefs: ?::core::marker::Sized + dialog::Types,
            ComponentType: crate::imports::frender_html::props::IntrinsicComponent,
            W: ::frender_ssr::AsyncWrite + ::core::marker::Unpin,
        > ::frender_core::UpdateRenderState<::frender_ssr::SsrContext<W>>
        for dialog::Data<TypeDefs, ComponentType>
    where
        HtmlDialogElementProps::Data<TypeDefs>: ::frender_ssr::IntoSsrData<W>,
    {
        type State = ::frender_ssr::element::html::HtmlElementRenderState<
            'static,
            <<HtmlDialogElementProps::Data<
                TypeDefs,
            > as ::frender_ssr::IntoSsrData<
                W,
            >>::Children as ::frender_core::UpdateRenderState<
                ::frender_ssr::SsrContext<W>,
            >>::State,
            <HtmlDialogElementProps::Data<
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
