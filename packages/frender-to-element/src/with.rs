use frender_fn_traits::{Fn1, FnOnce1};

use super::ToElement;

#[derive(Debug, Clone, Copy)]
pub struct ToElementWithFn<E, F: for<'e> Fn1<&'e E>>(pub E, pub F);

impl<E, F: for<'e> Fn1<&'e E>> ToElement for ToElementWithFn<E, F> {
    type ToElement<'a>
        = <F as FnOnce1<&'a E>>::Output_
    where
        Self: 'a;

    fn to_element(&self) -> Self::ToElement<'_> {
        (self.1)(&self.0)
    }
}

#[derive(Debug)]
pub struct RefToElementWithFn<'e, E: ?Sized, F: ?Sized + Fn1<&'e E>>(pub &'e E, pub F);

impl<'e, E, F: Copy + Fn1<&'e E>> Copy for RefToElementWithFn<'e, E, F> {}

impl<'e, E, F: Clone + Fn1<&'e E>> Clone for RefToElementWithFn<'e, E, F> {
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}

impl<'e, E: ?Sized, F: ?Sized + Fn1<&'e E>> ToElement for RefToElementWithFn<'e, E, F> {
    type ToElement<'a>
        = <F as FnOnce1<&'e E>>::Output_
    where
        Self: 'a;

    fn to_element(&self) -> Self::ToElement<'_> {
        (self.1)(self.0)
    }
}
