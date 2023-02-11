#[inline]
pub const fn fieldset() -> Building<TypesInitial> {
    super::HtmlFieldSetElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlFieldSetElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "fieldset";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct fieldset<
        TypeDefs: ?::core::marker::Sized + HtmlFieldSetElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(
        pub HtmlFieldSetElementProps::Data<TypeDefs>,
        pub ComponentType,
    );
}
pub use struct_data::fieldset as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub const fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(
        HtmlFieldSetElementProps::build(building),
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
            TypeDefs: ?::core::marker::Sized + fieldset::Types,
            ComponentType: crate::props::IntrinsicComponent,
        > ::frender_core::UpdateRenderState<::frender_dom::Dom>
        for fieldset::Data<TypeDefs, ComponentType>
    where
        HtmlFieldSetElementProps::Data<TypeDefs>:
            crate::props::UpdateElement<web_sys::HtmlFieldSetElement>,
    {
        type State = crate::props::IntrinsicComponentRenderState<
            web_sys::HtmlFieldSetElement,
            <HtmlFieldSetElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                web_sys::HtmlFieldSetElement,
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
                <HtmlFieldSetElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                    web_sys::HtmlFieldSetElement,
                >>::update_element(self.0, element, children_ctx, state)
            })
        }
    }
}
