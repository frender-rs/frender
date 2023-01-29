#[inline]
pub fn video() -> Building<TypesInitial> {
    super::HtmlVideoElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlVideoElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "video";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct video<
        TypeDefs: ?::core::marker::Sized + HtmlVideoElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(pub HtmlVideoElementProps::Data<TypeDefs>, pub ComponentType);
}
pub use struct_data::video as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlVideoElementProps::build(building), self::ComponentType)
}
pub use build as build_element;
#[inline]
pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    build(building)
}
