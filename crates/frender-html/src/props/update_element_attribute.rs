pub trait UpdateElementAttribute {
    fn update_element_attribute(&self, element: &web_sys::Element, attribute_name: &str);
}

impl UpdateElementAttribute for str {
    #[inline]
    fn update_element_attribute(&self, element: &web_sys::Element, attribute_name: &str) {
        element.set_attribute(attribute_name, self).unwrap()
    }
}

impl UpdateElementAttribute for bool {
    #[inline]
    fn update_element_attribute(&self, element: &web_sys::Element, attribute_name: &str) {
        if *self {
            element.set_attribute(attribute_name, "").unwrap()
        } else {
            element.remove_attribute(attribute_name).unwrap()
        }
    }
}

impl UpdateElementAttribute for u32 {
    #[inline]
    fn update_element_attribute(&self, element: &web_sys::Element, attribute_name: &str) {
        element
            .set_attribute(attribute_name, &self.to_string())
            .unwrap()
    }
}
