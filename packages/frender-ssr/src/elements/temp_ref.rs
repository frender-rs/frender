use frender_reactive_value::temp_ref::TempRef;

use crate::SsrElement;

impl<'a, T: ?Sized + TempRefToSsrElement> SsrElement for TempRef<'a, T> {
    type HtmlChildren = <T::TempRefToSsrElement<'a> as SsrElement>::HtmlChildren;

    fn into_html_children(self) -> Self::HtmlChildren {
        self.0.temp_ref_to_ssr_element().into_html_children()
    }
}

impl<'a, T: ?Sized + TempRefToSsrElement> super::KnownCopySsrElement for TempRef<'a, T> {}

pub trait TempRefToSsrElement: 'static {
    type TempRefToSsrElement<'a>: SsrElement;
    fn temp_ref_to_ssr_element(&self) -> Self::TempRefToSsrElement<'_>;
}

impl TempRefToSsrElement for str {
    type TempRefToSsrElement<'a> = TempStrIntoSsrElement<'a>;

    fn temp_ref_to_ssr_element(&self) -> Self::TempRefToSsrElement<'_> {
        TempStrIntoSsrElement(self)
    }
}

pub struct TempStrIntoSsrElement<'a>(&'a str);

impl<'a> SsrElement for TempStrIntoSsrElement<'a> {
    type HtmlChildren = frender_ssr_html::encode::Encode<
        //
        frender_ssr_html::escape_safe::Safe,
        &'a str,
    >;

    fn into_html_children(self) -> Self::HtmlChildren {
        Self::HtmlChildren::new(frender_ssr_html::escape_safe::Safe, self.0)
    }
}
