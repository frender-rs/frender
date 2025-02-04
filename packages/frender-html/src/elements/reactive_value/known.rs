use frender_common::{
    impl_many,
    reactive_value::ReactiveValue,
    strings::{CsrStr, NonReactiveStr},
    TempStr,
};
use frender_dom::string_element::StringElement;

use crate::element::CsrElement;

use super::{ReactiveValueIntoElement, ReactiveValueWithKind};

// region: &'static str -- self as value and also cache
impl ReactiveValueWithKind for &'static str {
    type ReactiveValueKind = &'static str;
}

impl CsrElement for &'static str {
    type RenderStateKind = <ReactiveValueIntoElement<&'static str> as CsrElement>::RenderStateKind;

    crate::proxy_csr_element!(|this| ReactiveValueIntoElement(this));
}
// endregion
// region: other static strings -- self as cache but lends as value temporarily
impl_many!(
    impl<__> ReactiveValueWithKind
        for each_of![
            //
            std::borrow::Cow<'static, str>,
            String,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type ReactiveValueKind = str;
    }
);
impl_many!(
    impl<__> CsrElement
        for each_of![
            //
            std::borrow::Cow<'static, str>,
            String,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type RenderStateKind = <ReactiveValueIntoElement<Self> as CsrElement>::RenderStateKind;
        crate::proxy_csr_element!(|this| ReactiveValueIntoElement(this));
    }
);
// endregion
// region: StringElement
impl ReactiveValueWithKind for StringElement {
    type ReactiveValueKind = StringElement;
}
impl CsrElement for StringElement {
    type RenderStateKind = <ReactiveValueIntoElement<Self> as CsrElement>::RenderStateKind;
    crate::proxy_csr_element!(|this| ReactiveValueIntoElement(this));
}
// endregion
// region: TempStr
impl<S> ReactiveValueWithKind for TempStr<S>
where
    S: frender_common::IntoStaticStrCache,
{
    type ReactiveValueKind = str;
}
/// <code>where TempStr\<S>: [CsrStr](CsrStr)</code>
impl<S> CsrElement for TempStr<S>
where
    S: frender_common::IntoStaticStrCache,
{
    // No matter what S is, TempStr<S> acts like TempStr<&'static str>
    type RenderStateKind = <ReactiveValueIntoElement<TempStr<&'static str>> as CsrElement>::RenderStateKind;
    crate::proxy_csr_element!(|this| ReactiveValueIntoElement(this));
}
// endregion
// region: NonReactiveStr
impl<S: CsrStr> ReactiveValueWithKind for NonReactiveStr<S> {
    type ReactiveValueKind = str;
}
impl<S: CsrStr> CsrElement for NonReactiveStr<S> {
    type RenderStateKind = <ReactiveValueIntoElement<Self> as CsrElement>::RenderStateKind;
    crate::proxy_csr_element!(|this| ReactiveValueIntoElement(this));
}
// endregion
// region: scalar
impl_many!(
    impl<__> ReactiveValueWithKind
        for each_of![
            i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, //
            f32, f64, //
            char
        ]
    {
        type ReactiveValueKind = Self;
    }
);
impl_many!(
    impl<__> CsrElement
        for each_of![
            i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, //
            f32, f64, //
            char
        ]
    {
        type RenderStateKind = <ReactiveValueIntoElement<Self> as CsrElement>::RenderStateKind;
        crate::proxy_csr_element!(|this| ReactiveValueIntoElement(this));
    }
);
// endregion
