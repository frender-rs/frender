use frender_dom::component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
use frender_ssr::SsrElement;

impl<TM, Children, Attrs, AttrsWithPinnedState> SsrElement for super::Intrinsic<TM, Children, Attrs, AttrsWithPinnedState>
where
    TM: SsrComponent<Attrs, Children>,
    Attrs: IntoSpaceAndHtmlAttributesOrEmpty,
    // AttrsWithPinnedState are considered csr only
{
    type HtmlChildren = TM::OneElement;

    fn into_html_children(self) -> Self::HtmlChildren {
        let Self {
            type_marker: _,
            attributes,
            attributes_with_pinned_state: _,
            children,
        } = self;

        TM::ssr_component(attributes, children)
    }
}
