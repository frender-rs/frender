use crate::{
    csr::element::FormControlElement,
    value::{KindOfChecked, KindOfValue, KindOfValueAsNumber},
};

/// A trait alias
pub trait InputElement<Renderer: ?Sized>:
    FormControlElement<KindOfValue, Renderer>
    + FormControlElement<KindOfChecked, Renderer>
    + FormControlElement<KindOfValueAsNumber, Renderer>
{
}

impl<E, Renderer: ?Sized> InputElement<Renderer> for E where
    E: FormControlElement<KindOfValue, Renderer>
        + FormControlElement<KindOfChecked, Renderer>
        + FormControlElement<KindOfValueAsNumber, Renderer>
{
}
