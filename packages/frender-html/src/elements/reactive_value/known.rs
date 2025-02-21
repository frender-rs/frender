use frender_common::{
    impl_many,
    reactive_value::{non_reactive::Uncached, ReactiveValueWithKind, UncachedNonReactiveValueWithKind},
    strings::{CsrStr, NonReactiveStr},
    IntoStaticStrCache, TempStr,
};
use frender_dom::string_element::StringElement;

use crate::{csr::CsrElement, html::RenderHtml};

use super::{ReactiveValueIntoElement, ValueKindStatelessRender};

trait KnownReactiveValue: ReactiveValueWithKind<ReactiveValueKind: ValueKindStatelessRender> {}

impl<T: KnownReactiveValue> CsrElement for T {
    type RenderStateKind = <ReactiveValueIntoElement<Self> as CsrElement>::RenderStateKind;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = <ReactiveValueIntoElement<Self> as CsrElement>::PinnedRenderInit<R>;

    crate::proxy_csr_element!(|this| ReactiveValueIntoElement(this));
}

// region: static text
impl_many!(
    impl<__> KnownReactiveValue
        for each_of![
            // static strings
            &'static str,
            std::borrow::Cow<'static, str>,
            String,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
            StringElement,
            // scalar
            i8,
            u8,
            i16,
            u16,
            i32,
            u32,
            i64,
            u64,
            i128,
            u128,
            isize,
            usize,
            f32,
            f64,
            char
        ]
    {
    }
);
// endregion
// region: TempStr
/// <code>where TempStr\<S>: [CsrStr](CsrStr)</code>
impl<S: IntoStaticStrCache> KnownReactiveValue for TempStr<S> {}
// endregion
// region: NonReactiveStr
impl<S: CsrStr> KnownReactiveValue for NonReactiveStr<S> {}
// endregion
// region: Uncached
impl<T: UncachedNonReactiveValueWithKind> KnownReactiveValue for Uncached<T>
//
where
    T::UncachedNonReactiveValueKind: ValueKindStatelessRender,
{
}
// endregion
