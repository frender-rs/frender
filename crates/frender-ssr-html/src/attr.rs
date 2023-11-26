use crate::assert::{HtmlAttributeEqValueOrEmpty, SpaceAndHtmlAttributeName};

frender_common::Strings!(
    enum SpaceAndHtmlAttributeState {}
    pub struct SpaceAndHtmlAttribute<N: SpaceAndHtmlAttributeName, V: HtmlAttributeEqValueOrEmpty>(
        name!(N),
        eq_value!(V),
    );
);

#[allow(non_snake_case)]
pub fn SpaceAndHtmlAttribute<N: SpaceAndHtmlAttributeName, V: HtmlAttributeEqValueOrEmpty>(
    name: N,
    eq_value: V,
) -> SpaceAndHtmlAttribute<N, V> {
    SpaceAndHtmlAttribute {
        _state: SpaceAndHtmlAttributeState(),
        name,
        eq_value,
    }
}
