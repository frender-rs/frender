use frender_ssr::SsrElement;

use crate::ContextKeyInner;

use super::{ElementWithContext, IntoContextValue};

impl<
        T,
        Inner: ContextKeyInner<Value = T> + 'static,
        F: IntoContextValue<ContextValue = T>,
        FE: FnOnce() -> E,
        E: SsrElement,
    > SsrElement for ElementWithContext<Inner, F, FE>
{
    type HtmlChildren = E::HtmlChildren;

    fn into_html_children(self) -> Self::HtmlChildren {
        let Self {
            context_key,
            into_value,
            get_element,
        } = self;
        let mut value = Inner::make_swap_value(into_value.into_context_value());
        context_key.provide_value(&mut value, || get_element().into_html_children())
    }
}
