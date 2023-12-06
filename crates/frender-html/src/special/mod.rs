mod script;
mod style;
mod void_elements;

mod utils {
    pub(super) struct UpdateInnerText<'a, E, R>(pub(super) &'a mut E, pub(super) &'a mut R);

    impl<'a, E: crate::html::behaviors::HtmlElement<R>, R> frender_html_common::ValueUpdater<str> for UpdateInnerText<'a, E, R> {
        fn update(self, value: &str) {
            self.0.set_inner_text(self.1, value)
        }

        fn remove(self) {
            self.0.set_inner_text(self.1, "")
        }
    }
}
