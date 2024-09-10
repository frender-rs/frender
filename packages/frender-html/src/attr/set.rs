use frender_attr_value::csr::ValueKind;

use frender_dom::behaviors::Element;

pub(crate) trait SetAttribute: ValueKind {
    fn set_attribute<E: ?Sized + Element<RR>, RR: ?Sized>(element: &mut E, renderer: &mut RR, attr_name: &str, value: Self::Value<'_>);
}

impl SetAttribute for str {
    fn set_attribute<E: ?Sized + Element<RR>, RR: ?Sized>(element: &mut E, renderer: &mut RR, attr_name: &str, value: &Self) {
        element.set_attribute(renderer, attr_name, value)
    }
}

impl SetAttribute for bool {
    fn set_attribute<E: ?Sized + Element<RR>, RR: ?Sized>(element: &mut E, renderer: &mut RR, attr_name: &str, (): ()) {
        element.set_attribute(renderer, attr_name, "")
    }
}
