use crate::behaviors;

pub trait SetAsAttributeValue {
    fn set_as_attribute_value<E: ?Sized + behaviors::Element<RR>, RR: ?Sized>(
        element: &mut E,
        renderer: &mut RR,
        attr_name: &str,
        value: &Self,
    );
}

impl SetAsAttributeValue for str {
    fn set_as_attribute_value<E: ?Sized + behaviors::Element<RR>, RR: ?Sized>(
        element: &mut E,
        renderer: &mut RR,
        attr_name: &str,
        value: &Self,
    ) {
        element.set_attribute(renderer, attr_name, value)
    }
}

impl SetAsAttributeValue for u32 {
    fn set_as_attribute_value<E: ?Sized + behaviors::Element<RR>, RR: ?Sized>(
        element: &mut E,
        renderer: &mut RR,
        attr_name: &str,
        value: &Self,
    ) {
        element.set_attribute(renderer, attr_name, &value.to_string())
    }
}

impl SetAsAttributeValue for bool {
    fn set_as_attribute_value<E: ?Sized + behaviors::Element<RR>, RR: ?Sized>(
        element: &mut E,
        renderer: &mut RR,
        attr_name: &str,
        value: &Self,
    ) {
        if *value {
            element.set_attribute(renderer, attr_name, "")
        } else {
            element.remove_attribute(renderer, attr_name)
        }
    }
}
