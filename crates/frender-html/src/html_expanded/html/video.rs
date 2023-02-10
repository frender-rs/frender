#[inline]
pub fn video() -> Building<TypesInitial> {
    super::HtmlVideoElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlVideoElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "video";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct video<
        TypeDefs: ?::core::marker::Sized + HtmlVideoElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(pub HtmlVideoElementProps::Data<TypeDefs>, pub ComponentType);
}
pub use struct_data::video as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlVideoElementProps::build(building), self::ComponentType)
}
pub use build as build_element;
#[inline]
pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    build(building)
}
#[cfg(feature = "dom")]
mod impl_update_render_state_dom {
    use super::super::*;
    impl<
            TypeDefs: ?::core::marker::Sized + video::Types,
            ComponentType: crate::props::IntrinsicComponent,
        > ::frender_core::UpdateRenderState<::frender_dom::Dom>
        for video::Data<TypeDefs, ComponentType>
    where
        HtmlVideoElementProps::Data<TypeDefs>:
            crate::props::UpdateElement<web_sys::HtmlVideoElement>,
    {
        type State = crate::props::IntrinsicComponentRenderState<
            web_sys::HtmlVideoElement,
            <HtmlVideoElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                web_sys::HtmlVideoElement,
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
                <HtmlVideoElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                    web_sys::HtmlVideoElement,
                >>::update_element(self.0, element, children_ctx, state)
            })
        }
    }
}
