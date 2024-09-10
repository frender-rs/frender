mod inner_html;
mod input;

mod script;
mod style;
mod textarea;
mod void_elements;

mod utils {
    use frender_attr_value::csr::UpdateAttrValue;

    pub(super) struct UpdateInnerText<'a, E: ?Sized, R: ?Sized>(pub(super) &'a mut E, pub(super) &'a mut R);

    impl<'a, E: frender_dom::behaviors::HtmlElement<R> + ?Sized, R: ?Sized> UpdateAttrValue for UpdateInnerText<'a, E, R> {
        type Kind = str;
        fn set(self, value: &str) {
            self.0.set_inner_text(self.1, value)
        }

        fn remove(self) {
            self.0.set_inner_text(self.1, "")
        }
    }
}
