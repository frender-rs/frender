use std::marker::PhantomData;

use crate::csr::{CsrAttributes, CsrAttributesStateUnmount, RenderAttributes};

use super::{ConstAttributes, HasConstAttributes};

pub trait CsrConstAttributes {
    fn remove_all<T: ?Sized + HasConstAttributes<Attributes = Self>>(
        renderer: &mut impl RenderAttributes,
    );
    fn set_all<T: ?Sized + HasConstAttributes<Attributes = Self>>(
        renderer: &mut impl RenderAttributes,
    );
}

pub struct State<T: ?Sized + HasConstAttributes>(PhantomData<T>);

impl<T: ?Sized + HasConstAttributes> CsrAttributesStateUnmount for State<T> {
    fn state_unmount(&mut self, renderer: &mut impl RenderAttributes) {
        T::Attributes::remove_all::<T>(renderer)
    }
}

impl<T: ?Sized + HasConstAttributes> CsrAttributes for ConstAttributes<T> {
    type State = State<T>;

    fn render_init(self, renderer: &mut impl RenderAttributes) -> Self::State {
        T::Attributes::set_all::<T>(renderer);
        State(PhantomData)
    }

    fn render_update(self, _: &mut impl RenderAttributes, _: &mut Self::State) {}
}
