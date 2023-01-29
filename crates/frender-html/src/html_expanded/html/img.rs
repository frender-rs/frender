#[inline]
pub fn img() -> Building<TypesInitial> {
    super::HtmlImageElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlImageElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "img";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct img<
        TypeDefs: ?::core::marker::Sized + HtmlImageElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(pub HtmlImageElementProps::Data<TypeDefs>, pub ComponentType);
}
pub use struct_data::img as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlImageElementProps::build(building), self::ComponentType)
}
pub use build as build_element;
#[inline]
pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    build(building)
}
