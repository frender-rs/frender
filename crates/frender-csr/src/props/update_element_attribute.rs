pub trait UpdateElementAttribute {
    fn update_element_attribute(this: &Self, element: &web_sys::Element, attribute_name: &str);
}

impl UpdateElementAttribute for str {
    #[inline]
    fn update_element_attribute(this: &Self, element: &web_sys::Element, attribute_name: &str) {
        element.set_attribute(attribute_name, this).unwrap()
    }
}

impl UpdateElementAttribute for bool {
    #[inline]
    fn update_element_attribute(this: &Self, element: &web_sys::Element, attribute_name: &str) {
        if *this {
            element.set_attribute(attribute_name, "").unwrap()
        } else {
            element.remove_attribute(attribute_name).unwrap()
        }
    }
}

impl UpdateElementAttribute for u32 {
    #[inline]
    fn update_element_attribute(this: &Self, element: &web_sys::Element, attribute_name: &str) {
        // TODO: pass u32 directly and let js runtime convert it into a string. https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttribute
        element
            .set_attribute(attribute_name, &this.to_string())
            .unwrap()
    }
}
