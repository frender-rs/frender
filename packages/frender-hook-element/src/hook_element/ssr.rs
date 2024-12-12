use frender_ssr::{html::assert::HtmlChildren, SsrElement};

use super::{HookElement, UseHookData};

impl<F, HC> SsrElement for HookElement<F>
where
    HC: HtmlChildren,
    // Note: F must output SsrElement of same kind.
    F: for<'hook> UseHookData<Value<'hook>: SsrElement<HtmlChildren = HC>>,
    F::HookData: Default,
{
    type HtmlChildren = HC;

    fn into_html_children(self) -> Self::HtmlChildren {
        let Self(mut f) = self;
        let hook_data = std::pin::pin!(<F::HookData>::default());
        let element = f.use_hook_data(hook_data);
        element.into_html_children()
    }
}
