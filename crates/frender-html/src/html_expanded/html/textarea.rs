#[inline]
pub fn textarea() -> Building<TypesInitial> {
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
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
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
