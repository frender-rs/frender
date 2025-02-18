#![cfg(feature = "components")]
#![cfg(feature = "csr")]

use frender_html::html::{behavior_type_traits, markers};

#[test]
const fn assert_impl()
where
    markers::div: behavior_type_traits::Node,
    markers::div: behavior_type_traits::Element,
    markers::div: behavior_type_traits::HtmlElement,
    markers::div: behavior_type_traits::HtmlDivElement,
{
}

const _: () = assert_impl();
