use frender_common::impl_many;
use frender_dom::string_element::StringElement;
use frender_reactive_value::{
    non_reactive::Uncached,
    static_or_temp_ref::StaticOrTempRef,
    temp_into_static::{IntoStaticCache, IntoStaticWithKind, TempIntoStatic},
    temp_ref::TempRef,
    ReactiveValueWithKind, UncachedNonReactiveValueWithKind,
};

use crate::{
    csr::{
        stateless_render::{StatelessRender, StatelessRenderStateKind},
        CsrElement,
    },
    html::RenderHtml,
};

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
// region: StaticOrTempRef
impl<T: ?Sized + 'static + ToOwned + PartialEq, K: StatelessRenderStateKind> KnownReactiveValue for StaticOrTempRef<'_, T>
//
where
    for<'a> StaticOrTempRef<'a, T>: StatelessRender<StatelessRenderStateKind = K>,
{
}
// endregion
// region: TempRef
impl<T: IntoStaticWithKind + IntoStaticCache<T::IntoStaticValue>, K: StatelessRenderStateKind> KnownReactiveValue for TempIntoStatic<T>
//
where
    for<'a> TempRef<'a, T::IntoStaticValue>: StatelessRender<StatelessRenderStateKind = K>
{
}
// endregion
// region: Uncached
impl<T: UncachedNonReactiveValueWithKind> KnownReactiveValue for Uncached<T>
//
where
    T::UncachedNonReactiveValueKind: ValueKindStatelessRender,
{
}
// endregion
