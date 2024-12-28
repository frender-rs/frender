use frender_ssr::SsrElement;

use crate::fn_traits::{FnOnce1, FnOnce2};

use super::{Memo, MemoAndProvideFirstArgument};

impl<
        F: for<'a> FnOnce1<&'a Dep, Output: SsrElement<HtmlChildren = C>>,
        Dep,
        C: frender_ssr::html::assert::HtmlChildren,
    > SsrElement for Memo<F, Dep>
{
    type HtmlChildren = C;

    fn into_html_children(self) -> Self::HtmlChildren {
        (self.0)(&self.1).into_html_children()
    }
}

impl<
        F: for<'a> FnOnce2<A, &'a Dep, Output: SsrElement<HtmlChildren = C>>,
        A,
        Dep,
        C: frender_ssr::html::assert::HtmlChildren,
    > SsrElement for MemoAndProvideFirstArgument<F, A, Dep>
{
    type HtmlChildren = C;

    fn into_html_children(self) -> Self::HtmlChildren {
        (self.0)(self.1, &self.2).into_html_children()
    }
}
