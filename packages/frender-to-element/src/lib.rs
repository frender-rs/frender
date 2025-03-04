pub use self::with::{RefToElementWithFn, ToElementWithFn};

mod with;

pub trait ToElement {
    type ToElement<'a>
    where
        Self: 'a;
    fn to_element(&self) -> Self::ToElement<'_>;
}

impl<E: ?Sized + ToElement> ToElement for &E {
    type ToElement<'a>
        = E::ToElement<'a>
    where
        Self: 'a;

    fn to_element(&self) -> Self::ToElement<'_> {
        E::to_element(self)
    }
}

/* TODO: export ToCsrElement and ToSsrElement?
use frender_html::csr::CsrElement;
use frender_ssr::SsrElement;

pub trait ToCsrElement: for<'a> ToElement<ToElement<'a>: CsrElement> {}
impl<E: ?Sized + for<'a> ToElement<ToElement<'a>: CsrElement>> ToCsrElement for E {}

pub trait ToSsrElement: for<'a> ToElement<ToElement<'a>: SsrElement> {}
impl<E: ?Sized + for<'a> ToElement<ToElement<'a>: SsrElement>> ToSsrElement for E {}
*/

mod imps;

#[cfg(test)]
mod tests;
