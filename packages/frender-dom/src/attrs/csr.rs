use frender_attrs::{
    experimental::csr::{CsrAttributes, RenderAttributes},
    IntoAttributes,
};

use crate::csr::behaviors::Element;

use super::Attrs;

impl<T: IntoAttributes> Attrs<T> {
    pub fn csr_render_init<R: ?Sized>(
        self,
        renderer: &mut R,
        element: &mut (impl ?Sized + Element<R>),
    ) -> <T::IntoAttributes as CsrAttributes>::State {
        T::into_attributes(self.0).render_init(&mut Render { renderer, element })
    }

    pub fn csr_render_update<R: ?Sized>(
        self,
        renderer: &mut R,
        element: &mut (impl ?Sized + Element<R>),
        state: &mut <T::IntoAttributes as CsrAttributes>::State,
    ) {
        T::into_attributes(self.0).render_update(&mut Render { renderer, element }, state)
    }
}

struct Render<'a, R: ?Sized, E: ?Sized> {
    renderer: &'a mut R,
    element: &'a mut E,
}

impl<R: ?Sized, E: ?Sized + Element<R>> RenderAttributes for Render<'_, R, E> {
    fn set_attribute(&mut self, name: &str, value: &str) {
        self.element.set_attribute(self.renderer, name, value)
    }

    fn remove_attribute(&mut self, name: &str) {
        self.element.remove_attribute(self.renderer, name)
    }
}
