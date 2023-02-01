#[inline]
pub fn a() -> Building<TypesInitial> {
    super::HtmlAnchorElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlAnchorElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "a";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct a<
        TypeDefs: ?::core::marker::Sized + HtmlAnchorElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(
        pub HtmlAnchorElementProps::Data<TypeDefs>,
        pub ComponentType,
    );
}
pub use struct_data::a as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlAnchorElementProps::build(building), self::ComponentType)
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
            TypeDefs: ?::core::marker::Sized + a::Types,
            ComponentType: crate::props::IntrinsicComponent,
        > ::frender_core::UpdateRenderState<::frender_dom::Dom> for a::Data<TypeDefs, ComponentType>
    where
        HtmlAnchorElementProps::Data<TypeDefs>:
            crate::props::UpdateElement<web_sys::HtmlAnchorElement>,
    {
        type State = crate::props::IntrinsicComponentRenderState<
            web_sys::HtmlAnchorElement,
            <HtmlAnchorElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                web_sys::HtmlAnchorElement,
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
                    <HtmlAnchorElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                        web_sys::HtmlAnchorElement,
                    >>::update_element(self.0, element, children_ctx, state)
                },
            )
        }
    }
}
