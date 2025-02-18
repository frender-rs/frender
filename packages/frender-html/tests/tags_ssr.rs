#![cfg(feature = "components")]
#![cfg(feature = "ssr")]

use frender_dom::component::HasIntrinsicComponentTag;
use frender_html::html::markers;

#[test]
fn tag_name() {
    assert_eq!(
        <markers::div as HasIntrinsicComponentTag>::INTRINSIC_COMPONENT_TAG,
        "div"
    );
}
