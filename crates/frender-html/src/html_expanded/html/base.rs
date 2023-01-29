#[inline]
pub fn base() -> Building<TypesInitial> {
    super::HtmlBaseElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlBaseElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "base";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct base<
        TypeDefs: ?::core::marker::Sized + HtmlBaseElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(pub HtmlBaseElementProps::Data<TypeDefs>, pub ComponentType);
}
pub use struct_data::base as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlBaseElementProps::build(building), self::ComponentType)
}
pub use build as build_element;
#[inline]
pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    build(building)
}
