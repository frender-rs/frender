#[inline]
pub fn details() -> Building<TypesInitial> {
    super::HtmlDetailsElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlDetailsElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "details";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct details<
        TypeDefs: ?::core::marker::Sized + HtmlDetailsElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(
        pub HtmlDetailsElementProps::Data<TypeDefs>,
        pub ComponentType,
    );
}
pub use struct_data::details as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(
        HtmlDetailsElementProps::build(building),
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
