#![cfg(feature = "components")]

use frender_dom::HasIntrinsicComponentTag;
use frender_html::cs;

#[test]
fn tag_name() {
    assert_eq!(
        <cs::div::Marker as HasIntrinsicComponentTag>::INTRINSIC_COMPONENT_TAG,
        "div"
    );
}
