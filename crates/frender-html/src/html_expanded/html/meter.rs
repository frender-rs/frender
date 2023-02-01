#[inline]
pub fn meter() -> Building<TypesInitial> {
    super::HtmlMeterElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlMeterElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "meter";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct meter<
        TypeDefs: ?::core::marker::Sized + HtmlMeterElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(pub HtmlMeterElementProps::Data<TypeDefs>, pub ComponentType);
}
pub use struct_data::meter as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlMeterElementProps::build(building), self::ComponentType)
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
            TypeDefs: ?::core::marker::Sized + meter::Types,
            ComponentType: crate::props::IntrinsicComponent,
        > ::frender_core::UpdateRenderState<::frender_dom::Dom>
        for meter::Data<TypeDefs, ComponentType>
    where
        HtmlMeterElementProps::Data<TypeDefs>:
            crate::props::UpdateElement<web_sys::HtmlMeterElement>,
    {
        type State = crate::props::IntrinsicComponentRenderState<
            web_sys::HtmlMeterElement,
            <HtmlMeterElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                web_sys::HtmlMeterElement,
            >>::State,
        >;
        fn update_render_state(
            self,
            ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let (node_and_mounted, state) = state.pin_project();
            crate::utils::dom::insert_element_and_update_with_tag(
                node_and_mounted,
                ctx,
                ComponentType::INTRINSIC_TAG,
                |element, children_ctx| {
                    <HtmlMeterElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                        web_sys::HtmlMeterElement,
                    >>::update_element(self.0, element, children_ctx, state)
                },
            )
        }
    }
}
