#![allow(non_snake_case)]
use frender_common::Empty;
pub mod Node {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::Node as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::Node, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod Element {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::Element as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::Element, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithHrefAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithHrefAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithHrefAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithTargetAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithTargetAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithTargetAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithTypeAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithTypeAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithTypeAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithCiteAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithCiteAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithCiteAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithPlaceHolderAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithPlaceHolderAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithPlaceHolderAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithMaxMinLengthAttributes {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithMaxMinLengthAttributes as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithMaxMinLengthAttributes, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithHeightWidthStrAttributes {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithHeightWidthStrAttributes as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithHeightWidthStrAttributes, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithHeightWidthU32Attributes {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithHeightWidthU32Attributes as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithHeightWidthU32Attributes, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithMaxF64Attribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithMaxF64Attribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithMaxF64Attribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithValueF64Attribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithValueF64Attribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithValueF64Attribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithValueStrAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithValueStrAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithValueStrAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithOpenAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithOpenAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithOpenAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithNameAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithNameAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithNameAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithDisabledAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithDisabledAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithDisabledAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithCrossOriginAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithCrossOriginAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithCrossOriginAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithRelAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithRelAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithRelAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithReferrerPolicyAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithReferrerPolicyAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithReferrerPolicyAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithAltAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithAltAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithAltAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithLoadingAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithLoadingAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithLoadingAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithAcceptAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithAcceptAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithAcceptAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithAutoCompleteAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithAutoCompleteAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithAutoCompleteAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithAutoCorrectAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithAutoCorrectAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithAutoCorrectAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithFormAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithFormAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithFormAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithFormAttributes {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithFormAttributes as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithFormAttributes, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithFetchPriorityAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithFetchPriorityAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithFetchPriorityAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithHrefLangAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithHrefLangAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithHrefLangAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithSizesAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithSizesAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithSizesAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithUseMapAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithUseMapAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithUseMapAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithLabelAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithLabelAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithLabelAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithForAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithForAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithForAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithIntegrityAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithIntegrityAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithIntegrityAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithBlockingAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithBlockingAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithBlockingAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithMultipleAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithMultipleAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithMultipleAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithRequiredAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithRequiredAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithRequiredAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithSizeU32Attribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithSizeU32Attribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithSizeU32Attribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithSrcAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithSrcAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithSrcAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithSrcsetAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithSrcsetAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithSrcsetAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithBgColorAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithBgColorAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithBgColorAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithAlignAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithAlignAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithAlignAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithMediaAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithMediaAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithMediaAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithReadOnlyAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithReadOnlyAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithReadOnlyAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod ElementWithDateTimeAttribute {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ElementWithDateTimeAttribute as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::ElementWithDateTimeAttribute, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod HtmlElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::abbr;
        pub use super::super::address;
        pub use super::super::article;
        pub use super::super::aside;
        pub use super::super::b;
        pub use super::super::bdi;
        pub use super::super::bdo;
        pub use super::super::cite;
        pub use super::super::code;
        pub use super::super::dd;
        pub use super::super::dfn;
        pub use super::super::dt;
        pub use super::super::em;
        pub use super::super::figcaption;
        pub use super::super::figure;
        pub use super::super::footer;
        pub use super::super::header;
        pub use super::super::hgroup;
        pub use super::super::i;
        pub use super::super::kbd;
        pub use super::super::main;
        pub use super::super::mark;
        pub use super::super::nav;
        pub use super::super::noscript;
        pub use super::super::rp;
        pub use super::super::rt;
        pub use super::super::ruby;
        pub use super::super::s;
        pub use super::super::samp;
        pub use super::super::section;
        pub use super::super::small;
        pub use super::super::strong;
        pub use super::super::sub;
        pub use super::super::summary;
        pub use super::super::sup;
        pub use super::super::u;
        pub use super::super::var;
        pub use super::super::wbr;
    }
}
pub const fn abbr() -> abbr::Element<Empty, (), ()> {
    abbr::ELEMENT
}
pub mod abbr {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::abbr as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::abbr, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn address() -> address::Element<Empty, (), ()> {
    address::ELEMENT
}
pub mod address {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::address as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::address, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn article() -> article::Element<Empty, (), ()> {
    article::ELEMENT
}
pub mod article {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::article as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::article, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn aside() -> aside::Element<Empty, (), ()> {
    aside::ELEMENT
}
pub mod aside {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::aside as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::aside, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn b() -> b::Element<Empty, (), ()> {
    b::ELEMENT
}
pub mod b {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::b as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::b, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn bdi() -> bdi::Element<Empty, (), ()> {
    bdi::ELEMENT
}
pub mod bdi {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::bdi as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::bdi, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn bdo() -> bdo::Element<Empty, (), ()> {
    bdo::ELEMENT
}
pub mod bdo {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::bdo as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::bdo, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn cite() -> cite::Element<Empty, (), ()> {
    cite::ELEMENT
}
pub mod cite {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::cite as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::cite, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn code() -> code::Element<Empty, (), ()> {
    code::ELEMENT
}
pub mod code {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::code as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::code, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn dd() -> dd::Element<Empty, (), ()> {
    dd::ELEMENT
}
pub mod dd {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::dd as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::dd, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn dfn() -> dfn::Element<Empty, (), ()> {
    dfn::ELEMENT
}
pub mod dfn {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::dfn as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::dfn, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn dt() -> dt::Element<Empty, (), ()> {
    dt::ELEMENT
}
pub mod dt {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::dt as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::dt, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn em() -> em::Element<Empty, (), ()> {
    em::ELEMENT
}
pub mod em {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::em as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::em, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn figcaption() -> figcaption::Element<Empty, (), ()> {
    figcaption::ELEMENT
}
pub mod figcaption {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::figcaption as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::figcaption, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn figure() -> figure::Element<Empty, (), ()> {
    figure::ELEMENT
}
pub mod figure {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::figure as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::figure, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn footer() -> footer::Element<Empty, (), ()> {
    footer::ELEMENT
}
pub mod footer {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::footer as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::footer, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn header() -> header::Element<Empty, (), ()> {
    header::ELEMENT
}
pub mod header {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::header as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::header, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn hgroup() -> hgroup::Element<Empty, (), ()> {
    hgroup::ELEMENT
}
pub mod hgroup {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::hgroup as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::hgroup, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn i() -> i::Element<Empty, (), ()> {
    i::ELEMENT
}
pub mod i {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::i as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::i, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn kbd() -> kbd::Element<Empty, (), ()> {
    kbd::ELEMENT
}
pub mod kbd {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::kbd as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::kbd, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn main() -> main::Element<Empty, (), ()> {
    main::ELEMENT
}
pub mod main {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::main as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::main, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn mark() -> mark::Element<Empty, (), ()> {
    mark::ELEMENT
}
pub mod mark {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::mark as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::mark, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn nav() -> nav::Element<Empty, (), ()> {
    nav::ELEMENT
}
pub mod nav {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::nav as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::nav, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn noscript() -> noscript::Element<Empty, (), ()> {
    noscript::ELEMENT
}
pub mod noscript {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::noscript as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::noscript, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn rp() -> rp::Element<Empty, (), ()> {
    rp::ELEMENT
}
pub mod rp {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::rp as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::rp, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn rt() -> rt::Element<Empty, (), ()> {
    rt::ELEMENT
}
pub mod rt {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::rt as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::rt, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn ruby() -> ruby::Element<Empty, (), ()> {
    ruby::ELEMENT
}
pub mod ruby {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ruby as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::ruby, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn s() -> s::Element<Empty, (), ()> {
    s::ELEMENT
}
pub mod s {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::s as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::s, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn samp() -> samp::Element<Empty, (), ()> {
    samp::ELEMENT
}
pub mod samp {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::samp as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::samp, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn section() -> section::Element<Empty, (), ()> {
    section::ELEMENT
}
pub mod section {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::section as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::section, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn small() -> small::Element<Empty, (), ()> {
    small::ELEMENT
}
pub mod small {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::small as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::small, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn strong() -> strong::Element<Empty, (), ()> {
    strong::ELEMENT
}
pub mod strong {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::strong as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::strong, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn sub() -> sub::Element<Empty, (), ()> {
    sub::ELEMENT
}
pub mod sub {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::sub as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::sub, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn summary() -> summary::Element<Empty, (), ()> {
    summary::ELEMENT
}
pub mod summary {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::summary as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::summary, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn sup() -> sup::Element<Empty, (), ()> {
    sup::ELEMENT
}
pub mod sup {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::sup as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::sup, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn u() -> u::Element<Empty, (), ()> {
    u::ELEMENT
}
pub mod u {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::u as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::u, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn var() -> var::Element<Empty, (), ()> {
    var::ELEMENT
}
pub mod var {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::var as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::var, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn wbr() -> wbr::Element<Empty, (), ()> {
    wbr::ELEMENT
}
pub mod wbr {
    use super::super::markers;
    pub use super::HtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::wbr as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::wbr, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlDataListElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlDataListElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlDataListElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::datalist;
    }
}
pub const fn datalist() -> datalist::Element<Empty, (), ()> {
    datalist::ELEMENT
}
pub mod datalist {
    use super::super::markers;
    pub use super::HtmlDataListElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::datalist as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::datalist, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlDivElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlDivElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlDivElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::div;
    }
}
pub const fn div() -> div::Element<Empty, (), ()> {
    div::ELEMENT
}
pub mod div {
    use super::super::markers;
    pub use super::HtmlDivElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::div as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::div, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlDListElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlDListElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlDListElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::dl;
    }
}
pub const fn dl() -> dl::Element<Empty, (), ()> {
    dl::ELEMENT
}
pub mod dl {
    use super::super::markers;
    pub use super::HtmlDListElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::dl as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::dl, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlHeadingElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlHeadingElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlHeadingElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::h1;
        pub use super::super::h2;
        pub use super::super::h3;
        pub use super::super::h4;
        pub use super::super::h5;
        pub use super::super::h6;
    }
}
pub const fn h1() -> h1::Element<Empty, (), ()> {
    h1::ELEMENT
}
pub mod h1 {
    use super::super::markers;
    pub use super::HtmlHeadingElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::h1 as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::h1, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn h2() -> h2::Element<Empty, (), ()> {
    h2::ELEMENT
}
pub mod h2 {
    use super::super::markers;
    pub use super::HtmlHeadingElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::h2 as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::h2, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn h3() -> h3::Element<Empty, (), ()> {
    h3::ELEMENT
}
pub mod h3 {
    use super::super::markers;
    pub use super::HtmlHeadingElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::h3 as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::h3, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn h4() -> h4::Element<Empty, (), ()> {
    h4::ELEMENT
}
pub mod h4 {
    use super::super::markers;
    pub use super::HtmlHeadingElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::h4 as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::h4, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn h5() -> h5::Element<Empty, (), ()> {
    h5::ELEMENT
}
pub mod h5 {
    use super::super::markers;
    pub use super::HtmlHeadingElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::h5 as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::h5, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn h6() -> h6::Element<Empty, (), ()> {
    h6::ELEMENT
}
pub mod h6 {
    use super::super::markers;
    pub use super::HtmlHeadingElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::h6 as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::h6, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlHeadElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlHeadElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlHeadElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::head;
    }
}
pub const fn head() -> head::Element<Empty, (), ()> {
    head::ELEMENT
}
pub mod head {
    use super::super::markers;
    pub use super::HtmlHeadElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::head as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::head, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlHrElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlHrElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlHrElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::hr;
    }
}
pub const fn hr() -> hr::Element<Empty, (), ()> {
    hr::ELEMENT
}
pub mod hr {
    use super::super::markers;
    pub use super::HtmlHrElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::hr as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::hr, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlLegendElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlLegendElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlLegendElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::legend;
    }
}
pub const fn legend() -> legend::Element<Empty, (), ()> {
    legend::ELEMENT
}
pub mod legend {
    use super::super::markers;
    pub use super::HtmlLegendElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::legend as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::legend, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlMenuElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlMenuElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlMenuElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::menu;
    }
}
pub const fn menu() -> menu::Element<Empty, (), ()> {
    menu::ELEMENT
}
pub mod menu {
    use super::super::markers;
    pub use super::HtmlMenuElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::menu as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::menu, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlParagraphElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlParagraphElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlParagraphElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::p;
    }
}
pub const fn p() -> p::Element<Empty, (), ()> {
    p::ELEMENT
}
pub mod p {
    use super::super::markers;
    pub use super::HtmlParagraphElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::p as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::p, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlPictureElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlPictureElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlPictureElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::picture;
    }
}
pub const fn picture() -> picture::Element<Empty, (), ()> {
    picture::ELEMENT
}
pub mod picture {
    use super::super::markers;
    pub use super::HtmlPictureElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::picture as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::picture, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlPreElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlPreElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlPreElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::pre;
    }
}
pub const fn pre() -> pre::Element<Empty, (), ()> {
    pre::ELEMENT
}
pub mod pre {
    use super::super::markers;
    pub use super::HtmlPreElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::pre as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::pre, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlSpanElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlSpanElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlSpanElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::span;
    }
}
pub const fn span() -> span::Element<Empty, (), ()> {
    span::ELEMENT
}
pub mod span {
    use super::super::markers;
    pub use super::HtmlSpanElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::span as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::span, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlTemplateElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlTemplateElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlTemplateElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::template;
    }
}
pub const fn template() -> template::Element<Empty, (), ()> {
    template::ELEMENT
}
pub mod template {
    use super::super::markers;
    pub use super::HtmlTemplateElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::template as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::template, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlTitleElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlTitleElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlTitleElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::title;
    }
}
pub const fn title() -> title::Element<Empty, (), ()> {
    title::ELEMENT
}
pub mod title {
    use super::super::markers;
    pub use super::HtmlTitleElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::title as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::title, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlElementWithHref {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlElementWithHref as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlElementWithHref, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod HtmlAnchorElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlAnchorElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlAnchorElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::a;
    }
}
pub const fn a() -> a::Element<Empty, (), ()> {
    a::ELEMENT
}
pub mod a {
    use super::super::markers;
    pub use super::HtmlAnchorElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::a as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::a, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlAreaElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlAreaElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlAreaElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::area;
    }
}
pub const fn area() -> area::Element<Empty, (), ()> {
    area::ELEMENT
}
pub mod area {
    use super::super::markers;
    pub use super::HtmlAreaElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::area as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::area, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlMediaElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlMediaElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlMediaElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod HtmlBaseElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlBaseElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlBaseElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::base;
    }
}
pub const fn base() -> base::Element<Empty, (), ()> {
    base::ELEMENT
}
pub mod base {
    use super::super::markers;
    pub use super::HtmlBaseElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::base as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::base, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlQuoteElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlQuoteElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlQuoteElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::blockquote;
        pub use super::super::q;
    }
}
pub const fn blockquote() -> blockquote::Element<Empty, (), ()> {
    blockquote::ELEMENT
}
pub mod blockquote {
    use super::super::markers;
    pub use super::HtmlQuoteElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::blockquote as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::blockquote, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn q() -> q::Element<Empty, (), ()> {
    q::ELEMENT
}
pub mod q {
    use super::super::markers;
    pub use super::HtmlQuoteElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::q as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::q, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlBodyElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlBodyElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlBodyElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::body;
    }
}
pub const fn body() -> body::Element<Empty, (), ()> {
    body::ELEMENT
}
pub mod body {
    use super::super::markers;
    pub use super::HtmlBodyElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::body as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::body, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlBrElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlBrElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlBrElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::br;
    }
}
pub const fn br() -> br::Element<Empty, (), ()> {
    br::ELEMENT
}
pub mod br {
    use super::super::markers;
    pub use super::HtmlBrElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::br as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::br, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlButtonElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlButtonElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlButtonElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::button;
    }
}
pub const fn button() -> button::Element<Empty, (), ()> {
    button::ELEMENT
}
pub mod button {
    use super::super::markers;
    pub use super::HtmlButtonElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::button as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::button, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlCanvasElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlCanvasElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlCanvasElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::canvas;
    }
}
pub const fn canvas() -> canvas::Element<Empty, (), ()> {
    canvas::ELEMENT
}
pub mod canvas {
    use super::super::markers;
    pub use super::HtmlCanvasElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::canvas as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::canvas, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlTableCaptionElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlTableCaptionElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlTableCaptionElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::caption;
    }
}
pub const fn caption() -> caption::Element<Empty, (), ()> {
    caption::ELEMENT
}
pub mod caption {
    use super::super::markers;
    pub use super::HtmlTableCaptionElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::caption as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::caption, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlDataElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlDataElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlDataElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::data;
    }
}
pub const fn data() -> data::Element<Empty, (), ()> {
    data::ELEMENT
}
pub mod data {
    use super::super::markers;
    pub use super::HtmlDataElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::data as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::data, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlModElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlModElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlModElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::del;
        pub use super::super::ins;
    }
}
pub const fn del() -> del::Element<Empty, (), ()> {
    del::ELEMENT
}
pub mod del {
    use super::super::markers;
    pub use super::HtmlModElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::del as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::del, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn ins() -> ins::Element<Empty, (), ()> {
    ins::ELEMENT
}
pub mod ins {
    use super::super::markers;
    pub use super::HtmlModElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ins as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::ins, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlDetailsElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlDetailsElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlDetailsElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::details;
    }
}
pub const fn details() -> details::Element<Empty, (), ()> {
    details::ELEMENT
}
pub mod details {
    use super::super::markers;
    pub use super::HtmlDetailsElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::details as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::details, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlDialogElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlDialogElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlDialogElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::dialog;
    }
}
pub const fn dialog() -> dialog::Element<Empty, (), ()> {
    dialog::ELEMENT
}
pub mod dialog {
    use super::super::markers;
    pub use super::HtmlDialogElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::dialog as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::dialog, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlEmbedElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlEmbedElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlEmbedElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::embed;
    }
}
pub const fn embed() -> embed::Element<Empty, (), ()> {
    embed::ELEMENT
}
pub mod embed {
    use super::super::markers;
    pub use super::HtmlEmbedElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::embed as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::embed, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlFieldSetElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlFieldSetElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlFieldSetElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::fieldset;
    }
}
pub const fn fieldset() -> fieldset::Element<Empty, (), ()> {
    fieldset::ELEMENT
}
pub mod fieldset {
    use super::super::markers;
    pub use super::HtmlFieldSetElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::fieldset as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::fieldset, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlFormElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlFormElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlFormElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::form;
    }
}
pub const fn form() -> form::Element<Empty, (), ()> {
    form::ELEMENT
}
pub mod form {
    use super::super::markers;
    pub use super::HtmlFormElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::form as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::form, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlHtmlElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlHtmlElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlHtmlElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::html;
    }
}
pub const fn html() -> html::Element<Empty, (), ()> {
    html::ELEMENT
}
pub mod html {
    use super::super::markers;
    pub use super::HtmlHtmlElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::html as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::html, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlIFrameElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlIFrameElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlIFrameElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::iframe;
    }
}
pub const fn iframe() -> iframe::Element<Empty, (), ()> {
    iframe::ELEMENT
}
pub mod iframe {
    use super::super::markers;
    pub use super::HtmlIFrameElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::iframe as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::iframe, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlImageElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlImageElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlImageElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::img;
    }
}
pub const fn img() -> img::Element<Empty, (), ()> {
    img::ELEMENT
}
pub mod img {
    use super::super::markers;
    pub use super::HtmlImageElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::img as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::img, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlInputElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlInputElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlInputElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::input;
    }
}
pub const fn input() -> input::Element<Empty, (), ()> {
    input::ELEMENT
}
pub mod input {
    use super::super::markers;
    pub use super::HtmlInputElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::input as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::input, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlLabelElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlLabelElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlLabelElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::label;
    }
}
pub const fn label() -> label::Element<Empty, (), ()> {
    label::ELEMENT
}
pub mod label {
    use super::super::markers;
    pub use super::HtmlLabelElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::label as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::label, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlLiElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlLiElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlLiElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::li;
    }
}
pub const fn li() -> li::Element<Empty, (), ()> {
    li::ELEMENT
}
pub mod li {
    use super::super::markers;
    pub use super::HtmlLiElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::li as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::li, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlLinkElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlLinkElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlLinkElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::link;
    }
}
pub const fn link() -> link::Element<Empty, (), ()> {
    link::ELEMENT
}
pub mod link {
    use super::super::markers;
    pub use super::HtmlLinkElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::link as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::link, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlMapElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlMapElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlMapElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::map;
    }
}
pub const fn map() -> map::Element<Empty, (), ()> {
    map::ELEMENT
}
pub mod map {
    use super::super::markers;
    pub use super::HtmlMapElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::map as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::map, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlMetaElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlMetaElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlMetaElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::meta;
    }
}
pub const fn meta() -> meta::Element<Empty, (), ()> {
    meta::ELEMENT
}
pub mod meta {
    use super::super::markers;
    pub use super::HtmlMetaElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::meta as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::meta, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlMeterElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlMeterElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlMeterElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::meter;
    }
}
pub const fn meter() -> meter::Element<Empty, (), ()> {
    meter::ELEMENT
}
pub mod meter {
    use super::super::markers;
    pub use super::HtmlMeterElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::meter as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::meter, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlObjectElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlObjectElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlObjectElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::object;
    }
}
pub const fn object() -> object::Element<Empty, (), ()> {
    object::ELEMENT
}
pub mod object {
    use super::super::markers;
    pub use super::HtmlObjectElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::object as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::object, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlOListElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlOListElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlOListElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::ol;
    }
}
pub const fn ol() -> ol::Element<Empty, (), ()> {
    ol::ELEMENT
}
pub mod ol {
    use super::super::markers;
    pub use super::HtmlOListElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ol as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::ol, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlOptGroupElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlOptGroupElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlOptGroupElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::optgroup;
    }
}
pub const fn optgroup() -> optgroup::Element<Empty, (), ()> {
    optgroup::ELEMENT
}
pub mod optgroup {
    use super::super::markers;
    pub use super::HtmlOptGroupElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::optgroup as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::optgroup, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlOptionElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlOptionElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlOptionElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::option;
    }
}
pub const fn option() -> option::Element<Empty, (), ()> {
    option::ELEMENT
}
pub mod option {
    use super::super::markers;
    pub use super::HtmlOptionElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::option as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::option, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlOutputElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlOutputElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlOutputElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::output;
    }
}
pub const fn output() -> output::Element<Empty, (), ()> {
    output::ELEMENT
}
pub mod output {
    use super::super::markers;
    pub use super::HtmlOutputElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::output as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::output, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlProgressElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlProgressElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlProgressElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::progress;
    }
}
pub const fn progress() -> progress::Element<Empty, (), ()> {
    progress::ELEMENT
}
pub mod progress {
    use super::super::markers;
    pub use super::HtmlProgressElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::progress as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::progress, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlScriptElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlScriptElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlScriptElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::script;
    }
}
pub const fn script() -> script::Element<Empty, (), ()> {
    script::ELEMENT
}
pub mod script {
    use super::super::markers;
    pub use super::HtmlScriptElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::script as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::script, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlSelectElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlSelectElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlSelectElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::select;
    }
}
pub const fn select() -> select::Element<Empty, (), ()> {
    select::ELEMENT
}
pub mod select {
    use super::super::markers;
    pub use super::HtmlSelectElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::select as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::select, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlSlotElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlSlotElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlSlotElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::slot;
    }
}
pub const fn slot() -> slot::Element<Empty, (), ()> {
    slot::ELEMENT
}
pub mod slot {
    use super::super::markers;
    pub use super::HtmlSlotElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::slot as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::slot, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlSourceElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlSourceElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlSourceElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::source;
    }
}
pub const fn source() -> source::Element<Empty, (), ()> {
    source::ELEMENT
}
pub mod source {
    use super::super::markers;
    pub use super::HtmlSourceElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::source as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::source, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlStyleElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlStyleElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlStyleElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::style;
    }
}
pub const fn style() -> style::Element<Empty, (), ()> {
    style::ELEMENT
}
pub mod style {
    use super::super::markers;
    pub use super::HtmlStyleElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::style as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::style, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlTableElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlTableElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlTableElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::table;
    }
}
pub const fn table() -> table::Element<Empty, (), ()> {
    table::ELEMENT
}
pub mod table {
    use super::super::markers;
    pub use super::HtmlTableElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::table as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::table, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlTableChildElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlTableChildElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlTableChildElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {}
}
pub mod HtmlTableSectionElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlTableSectionElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlTableSectionElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::tbody;
        pub use super::super::tfoot;
        pub use super::super::thead;
    }
}
pub const fn tbody() -> tbody::Element<Empty, (), ()> {
    tbody::ELEMENT
}
pub mod tbody {
    use super::super::markers;
    pub use super::HtmlTableSectionElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::tbody as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::tbody, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn tfoot() -> tfoot::Element<Empty, (), ()> {
    tfoot::ELEMENT
}
pub mod tfoot {
    use super::super::markers;
    pub use super::HtmlTableSectionElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::tfoot as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::tfoot, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn thead() -> thead::Element<Empty, (), ()> {
    thead::ELEMENT
}
pub mod thead {
    use super::super::markers;
    pub use super::HtmlTableSectionElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::thead as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::thead, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlTableRowElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlTableRowElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlTableRowElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::tr;
    }
}
pub const fn tr() -> tr::Element<Empty, (), ()> {
    tr::ELEMENT
}
pub mod tr {
    use super::super::markers;
    pub use super::HtmlTableRowElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::tr as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::tr, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlTableColElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlTableColElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlTableColElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::col;
        pub use super::super::colgroup;
    }
}
pub const fn col() -> col::Element<Empty, (), ()> {
    col::ELEMENT
}
pub mod col {
    use super::super::markers;
    pub use super::HtmlTableColElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::col as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::col, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn colgroup() -> colgroup::Element<Empty, (), ()> {
    colgroup::ELEMENT
}
pub mod colgroup {
    use super::super::markers;
    pub use super::HtmlTableColElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::colgroup as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::colgroup, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlTableCellElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlTableCellElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlTableCellElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::td;
        pub use super::super::th;
    }
}
pub const fn td() -> td::Element<Empty, (), ()> {
    td::ELEMENT
}
pub mod td {
    use super::super::markers;
    pub use super::HtmlTableCellElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::td as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::td, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub const fn th() -> th::Element<Empty, (), ()> {
    th::ELEMENT
}
pub mod th {
    use super::super::markers;
    pub use super::HtmlTableCellElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::th as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::th, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlTextAreaElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlTextAreaElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlTextAreaElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::textarea;
    }
}
pub const fn textarea() -> textarea::Element<Empty, (), ()> {
    textarea::ELEMENT
}
pub mod textarea {
    use super::super::markers;
    pub use super::HtmlTextAreaElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::textarea as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::textarea, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlTimeElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlTimeElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlTimeElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::time;
    }
}
pub const fn time() -> time::Element<Empty, (), ()> {
    time::ELEMENT
}
pub mod time {
    use super::super::markers;
    pub use super::HtmlTimeElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::time as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::time, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlTrackElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlTrackElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlTrackElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::track;
    }
}
pub const fn track() -> track::Element<Empty, (), ()> {
    track::ELEMENT
}
pub mod track {
    use super::super::markers;
    pub use super::HtmlTrackElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::track as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::track, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlUListElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlUListElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlUListElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::ul;
    }
}
pub const fn ul() -> ul::Element<Empty, (), ()> {
    ul::ELEMENT
}
pub mod ul {
    use super::super::markers;
    pub use super::HtmlUListElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::ul as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::ul, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlAudioElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlAudioElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlAudioElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::audio;
    }
}
pub const fn audio() -> audio::Element<Empty, (), ()> {
    audio::ELEMENT
}
pub mod audio {
    use super::super::markers;
    pub use super::HtmlAudioElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::audio as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::audio, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
pub mod HtmlVideoElement {
    use super::super::markers;
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::HtmlVideoElement as Marker;
    pub type Props<C, A, P> = Intrinsic<markers::HtmlVideoElement, C, A, P>;
    pub const PROPS: Props<Empty, (), ()> = Intrinsic::new_empty(Marker);
    pub mod tags {
        pub use super::super::video;
    }
}
pub const fn video() -> video::Element<Empty, (), ()> {
    video::ELEMENT
}
pub mod video {
    use super::super::markers;
    pub use super::HtmlVideoElement::{self as props, Props, PROPS};
    use crate::{intrinsic::Intrinsic, Empty};
    pub use markers::video as Marker;
    pub type Element<C, A, P> = Intrinsic<markers::video, C, A, P>;
    pub const ELEMENT: Element<Empty, (), ()> = Intrinsic::new_empty(Marker);
}
