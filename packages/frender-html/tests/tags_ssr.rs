#![cfg(feature = "components")]

use frender_dom::HasIntrinsicComponentTag;
use frender_html::html::markers;

#[test]
fn tag_name() {
    assert_eq!(
        <markers::div as HasIntrinsicComponentTag>::INTRINSIC_COMPONENT_TAG,
        "div"
    );
}
