#![cfg(feature = "components")]
#![cfg(feature = "csr")]

use frender_html::cs;
use frender_html::csr::experimental::html::behavior_type_traits;

#[test]
const fn assert_impl()
where
    cs::div::Marker: behavior_type_traits::Node,
    cs::div::Marker: behavior_type_traits::Element,
    cs::div::Marker: behavior_type_traits::HtmlElement,
    cs::div::Marker: behavior_type_traits::HtmlDivElement,
{
}

const _: () = assert_impl();
