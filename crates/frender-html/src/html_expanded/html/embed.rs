#[inline(always)]
pub fn embed() -> Building<TypesInitial> {
    super::HtmlEmbedElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlEmbedElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "embed";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct embed<
        TypeDefs: ?::core::marker::Sized + HtmlEmbedElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(pub HtmlEmbedElementProps::Data<TypeDefs>, pub ComponentType);
}
pub use struct_data::embed as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline(always)]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlEmbedElementProps::build(building), self::ComponentType)
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
            TypeDefs: ?::core::marker::Sized + embed::Types,
            ComponentType: crate::props::IntrinsicComponent,
        > ::frender_core::UpdateRenderState<::frender_dom::Dom>
        for embed::Data<TypeDefs, ComponentType>
    where
        HtmlEmbedElementProps::Data<TypeDefs>:
            crate::props::UpdateElement<web_sys::HtmlEmbedElement>,
    {
        type State = crate::props::IntrinsicComponentRenderState<
            web_sys::HtmlEmbedElement,
            <HtmlEmbedElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                web_sys::HtmlEmbedElement,
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
                <HtmlEmbedElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                    web_sys::HtmlEmbedElement,
                >>::update_element(self.0, element, children_ctx, state)
            })
        }
    }
}
