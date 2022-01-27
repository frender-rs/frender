use crate::Node;

pub trait AsNullableElement: Node {
    fn as_nullable_element(&self) -> Option<react_sys::Element>;
}

impl AsNullableElement for react_sys::Element {
    fn as_nullable_element(&self) -> Option<react_sys::Element> {
        Some(self.clone())
    }
}

impl<T: AsNullableElement> AsNullableElement for Option<T> {
    fn as_nullable_element(&self) -> Option<react_sys::Element> {
        self.as_ref()
            .and_then(AsNullableElement::as_nullable_element)
    }
}
