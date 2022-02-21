use crate::Element;

pub trait IntoOptionalElement {
    fn into_optional_element(self) -> Option<Element>;
}

pub trait IntoElement {
    fn into_element(self) -> Element;
}

impl<T: IntoElement> IntoOptionalElement for T {
    #[inline]
    fn into_optional_element(self) -> Option<Element> {
        Some(self.into_element())
    }
}

impl IntoElement for Element {
    #[inline]
    fn into_element(self) -> Element {
        self
    }
}

impl<T: IntoOptionalElement> IntoOptionalElement for Option<T> {
    #[inline]
    fn into_optional_element(self) -> Option<Element> {
        self.and_then(IntoOptionalElement::into_optional_element)
    }
}
