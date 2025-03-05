use frender_common::{HandleEvent, MaybeHandleEvent};
use hooks::ShareValue;

#[derive(Debug, Clone)]
pub struct Toggle<S: ShareValue<Value = bool>>(pub S);

impl<S: ShareValue<Value = bool>> PartialEq for Toggle<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0.equivalent_to(&other.0)
    }
}

impl<E: ?Sized, S: ShareValue<Value = bool>> HandleEvent<E> for Toggle<S> {
    fn handle_event(&mut self, _: &E) {
        self.0.map_mut(|v| *v = !*v)
    }
}

impl<E: ?Sized, S: ShareValue<Value = bool>> MaybeHandleEvent<E> for Toggle<S> {
    type HandleEvent = Self;
}
