use frender_common::convert::{FromMut, IntoMut};

use crate::{csr::element::FormControlElement, FormControlValueKind};

use super::super::InputElement;

pub trait InputValueKindCsr: FormControlValueKind {
    type AsMutFormControlElement<E: ?Sized + InputElement<R>, R: ?Sized>: ?Sized
        + FormControlElement<Self, R>
        + FromMut<E>
        + IntoMut<E>;
    fn as_mut_form_control_element<E: ?Sized + InputElement<R>, R: ?Sized>(
        el: &mut E,
    ) -> &mut Self::AsMutFormControlElement<E, R>;
}
