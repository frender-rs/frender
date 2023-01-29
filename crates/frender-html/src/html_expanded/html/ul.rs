#[inline]
pub fn ul() -> Building<TypesInitial> {
    super::HtmlUListElementProps()
}
mod reuse {
    use super::super::*;
    pub use HtmlUListElementProps::{prelude, Building, Types, TypesInitial, ValidTypes};
}
pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};
pub struct ComponentType;
impl crate::props::IntrinsicComponent for ComponentType {
    const INTRINSIC_TAG: &'static ::core::primitive::str = "ul";
}
mod struct_data {
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub struct ul<
        TypeDefs: ?::core::marker::Sized + HtmlUListElementProps::Types,
        ComponentType: crate::props::IntrinsicComponent = super::ComponentType,
    >(pub HtmlUListElementProps::Data<TypeDefs>, pub ComponentType);
}
pub use struct_data::ul as Data;
pub type DataInitial = Data<TypesInitial>;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    use super::*;
    self::Data(HtmlUListElementProps::build(building), self::ComponentType)
}
pub use build as build_element;
#[inline]
pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    build(building)
}
