use frender_dom::ssr::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
use frender_ssr::SsrElement;

impl<TM, Children, Attrs, AttrsWithPinnedState> SsrElement for super::Intrinsic<TM, Children, Attrs, AttrsWithPinnedState>
where
    TM: SsrComponent<Children>,
    Attrs: IntoSpaceAndHtmlAttributesOrEmpty,
    // AttrsWithPinnedState are considered csr only
{
    type HtmlChildren = TM::OneElement<Attrs>;

    fn into_html_children(self) -> Self::HtmlChildren {
        let Self {
            type_marker,
            attributes,
            attributes_with_pinned_state: _,
            children,
        } = self;

        type_marker.ssr_component(attributes, children)
    }
}
