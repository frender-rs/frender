use async_str_iter::borrow_str::IterBorrowStr;

use frender_dom::ssr::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
use frender_reactive_value::static_or_into_static_str::StaticOrIntoStaticStr;

use crate::html::components::style;

impl<Children: StaticOrIntoStaticStr> SsrComponent<Children> for style::Marker {
    type OneElement<Attrs: IntoSpaceAndHtmlAttributesOrEmpty> = frender_ssr::html::element::StyleElement<
        //
        <Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty,
        IterBorrowStr<Children::StaticStr>,
    >;

    fn ssr_component<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>(self, attrs: Attrs, children: Children) -> Self::OneElement<Attrs> {
        Self::OneElement::<Attrs>::new(attrs.into_space_and_html_attributes_or_empty(), IterBorrowStr::new(children.static_or_into_static_str()))
    }
}
