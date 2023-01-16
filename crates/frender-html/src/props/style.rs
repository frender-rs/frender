pub trait UpdateStyleProperty {
    type State;

    fn update_style_property(self, state: &mut Self::State, element: web_sys::Element);
}

impl UpdateStyleProperty for () {
    type State = ();

    #[inline]
    fn update_style_property(self, _: &mut Self::State, _: web_sys::Element) {}
}
