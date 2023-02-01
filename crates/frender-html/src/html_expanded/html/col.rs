#[inline]
pub fn col() -> Building<TypesInitial> {
    super::HtmlTableColElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlTableColElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "col";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct col<
        TypeDefs: ?::core::marker::Sized + HtmlTableColElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(
        pub HtmlTableColElementProps::Data<TypeDefs>,
        pub ComponentType,
    );
}
pub use struct_data::col as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(
        HtmlTableColElementProps::build(building),
        self::ComponentType,
    )
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
            TypeDefs: ?::core::marker::Sized + col::Types,
            ComponentType: crate::props::IntrinsicComponent,
        > ::frender_core::UpdateRenderState<::frender_dom::Dom>
        for col::Data<TypeDefs, ComponentType>
    where
        HtmlTableColElementProps::Data<TypeDefs>:
            crate::props::UpdateElement<web_sys::HtmlTableColElement>,
    {
        type State = crate::props::IntrinsicComponentRenderState<
            web_sys::HtmlTableColElement,
            <HtmlTableColElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                web_sys::HtmlTableColElement,
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
                    <HtmlTableColElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                        web_sys::HtmlTableColElement,
                    >>::update_element(self.0, element, children_ctx, state)
                },
            )
        }
    }
}
