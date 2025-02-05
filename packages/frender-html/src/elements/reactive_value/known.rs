use frender_common::{
    impl_many,
    reactive_value::ReactiveValue,
    strings::{CsrStr, NonReactiveStr},
    TempStr,
};
use frender_dom::string_element::StringElement;

use crate::element::CsrElement;

use super::{ReactiveValueIntoElement, ReactiveValueWithKind};

macro_rules! proxy_reactive_value_into_element {
    () => {
        proxy_reactive_value_into_element! {Self}
    };
    ($SelfTy:ty) => {
        type RenderStateKind = <ReactiveValueIntoElement<$SelfTy> as CsrElement>::RenderStateKind;
        type PinnedRenderInit<R: ?Sized + crate::RenderHtml> = <ReactiveValueIntoElement<$SelfTy> as CsrElement>::PinnedRenderInit<R>;

        crate::proxy_csr_element!(|this| ReactiveValueIntoElement(this));
    };
}

// region: &'static str -- self as value and also cache
impl ReactiveValueWithKind for &'static str {
    type ReactiveValueKind = &'static str;
}
impl CsrElement for &'static str {
    proxy_reactive_value_into_element! {}
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
        proxy_reactive_value_into_element! {}
    }
);
// endregion
// region: StringElement
impl ReactiveValueWithKind for StringElement {
    type ReactiveValueKind = StringElement;
}
impl CsrElement for StringElement {
    proxy_reactive_value_into_element! {}
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
    proxy_reactive_value_into_element!(TempStr<&'static str>);
}
// endregion
// region: NonReactiveStr
impl<S: CsrStr> ReactiveValueWithKind for NonReactiveStr<S> {
    type ReactiveValueKind = str;
}
impl<S: CsrStr> CsrElement for NonReactiveStr<S> {
    proxy_reactive_value_into_element! {}
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
        proxy_reactive_value_into_element! {}
    }
);
// endregion
