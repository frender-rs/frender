use frender_reactive_value::non_reactive::Uncached;

use crate::SsrElement;

/// [`Uncached<V>`] just derives ssr.
///
/// Note the difference between `Uncached<TempRef<str>>` and `Uncached<TempIntoStatic<&str>>`:
///
/// | `for<'a>`                           | ssr                                                       | csr                    |
/// | ----------------------------------- | --------------------------------------------------------- | ---------------------- |
/// | `Uncached<TempRef<'a, str>>`        | Zero cost. <br>`HtmlChildren = impl use<'a>`              | State = impl ZeroSized |
/// | `Uncached<TempIntoStatic<&'a str>>` | Will call `IntoStatic`. <br>`HtmlChildren = impl 'static` | State = impl ZeroSized |
impl<V: SsrElement> SsrElement for Uncached<V> {
    type HtmlChildren = V::HtmlChildren;

    fn into_html_children(self) -> Self::HtmlChildren {
        self.0.into_html_children()
    }
}

impl<V: SsrElement + Copy> super::KnownCopySsrElement for Uncached<V> {}
