use async_str_iter::any_str::IterAnyStr;

use frender_common::{strings::SsrStr, IntoStaticStr};

use frender_dom::component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};

use crate::html::components::style;

impl<Children: SsrStr> SsrComponent<Children> for style::Marker {
    type OneElement<Attrs: IntoSpaceAndHtmlAttributesOrEmpty> = frender_ssr::html::element::StyleElement<<Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty, IterAnyStr<Children::StaticStr>>;

    fn ssr_component<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>(self, attrs: Attrs, children: Children) -> Self::OneElement<Attrs> {
        Self::OneElement::<Attrs>::new(attrs.into_space_and_html_attributes_or_empty(), IterAnyStr::new(children.into_into_static_str().into_static_str()))
    }
}
