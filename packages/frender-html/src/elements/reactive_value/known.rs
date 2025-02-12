use frender_common::{
    impl_many,
    reactive_value::{non_reactive::Uncached, UncachedNonReactiveValueWithKind},
    strings::{CsrStr, NonReactiveStr},
    IntoStaticStrCache, TempStr,
};
use frender_dom::string_element::StringElement;

use crate::element::CsrElement;

use super::{ReactiveValueIntoElement, ValueKindStatelessRender};

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

// region: static text
impl_many!(
    impl<__> CsrElement
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
        proxy_reactive_value_into_element! {}
    }
);
// endregion
// region: TempStr
/// <code>where TempStr\<S>: [CsrStr](CsrStr)</code>
impl<S: IntoStaticStrCache> CsrElement for TempStr<S> {
    proxy_reactive_value_into_element! {}
}
// endregion
// region: NonReactiveStr
impl<S: CsrStr> CsrElement for NonReactiveStr<S> {
    proxy_reactive_value_into_element! {}
}
// endregion
// region: Uncached
impl<T: UncachedNonReactiveValueWithKind> CsrElement for Uncached<T>
where
    T::UncachedNonReactiveValueKind: ValueKindStatelessRender,
{
    proxy_reactive_value_into_element! {}
}
// endregion
