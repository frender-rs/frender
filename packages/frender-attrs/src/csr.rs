pub trait RenderAttributes {
    fn set_attribute(&mut self, name: &str, value: &str);
    fn remove_attribute(&mut self, name: &str);
}

pub trait CsrAttributesStateUnmount {
    fn state_unmount(&mut self, renderer: &mut impl RenderAttributes);
}

pub trait CsrAttributes {
    type State: CsrAttributesStateUnmount;

    fn render_init(self, renderer: &mut impl RenderAttributes) -> Self::State;
    fn render_update(self, renderer: &mut impl RenderAttributes, state: &mut Self::State);
}
