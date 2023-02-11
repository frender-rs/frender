#[inline]
pub const fn textarea() -> Building<TypesInitial> {
    super::HtmlTextAreaElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlTextAreaElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "textarea";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct textarea<
        TypeDefs: ?::core::marker::Sized + HtmlTextAreaElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(
        pub HtmlTextAreaElementProps::Data<TypeDefs>,
        pub ComponentType,
    );
}
pub use struct_data::textarea as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub const fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(
        HtmlTextAreaElementProps::build(building),
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
            TypeDefs: ?::core::marker::Sized + textarea::Types,
            ComponentType: crate::props::IntrinsicComponent,
        > ::frender_core::UpdateRenderState<::frender_dom::Dom>
        for textarea::Data<TypeDefs, ComponentType>
    where
        HtmlTextAreaElementProps::Data<TypeDefs>:
            crate::props::UpdateElement<web_sys::HtmlTextAreaElement>,
    {
        type State = crate::props::IntrinsicComponentRenderState<
            web_sys::HtmlTextAreaElement,
            <HtmlTextAreaElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                web_sys::HtmlTextAreaElement,
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
                <HtmlTextAreaElementProps::Data<TypeDefs> as crate::props::UpdateElement<
                    web_sys::HtmlTextAreaElement,
                >>::update_element(self.0, element, children_ctx, state)
            })
        }
    }
}
