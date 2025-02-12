use frender_form_control::textarea::TextAreaValue;

use crate::html::components::{textarea, HtmlTextAreaElement};

impl<Attrs, EL> HtmlTextAreaElement::Props<crate::Empty, Attrs, EL> {
    /// Alias for [`Self::children`]
    pub fn value<V: TextAreaValue>(self, value: V) -> HtmlTextAreaElement::Props<V, Attrs, EL> {
        self.children(value)
    }
}

impl<Attrs, EL> textarea::Element<crate::Empty, Attrs, EL> {
    /// Alias for [`Self::children`]
    pub fn value<V: TextAreaValue>(self, value: V) -> textarea::Element<V, Attrs, EL> {
        self.children(value)
    }
}
