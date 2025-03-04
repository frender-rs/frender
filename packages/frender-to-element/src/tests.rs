use std::{borrow::Cow, rc::Rc, sync::Arc};

use super::ToElement;
use frender_element::{CsrElement, Element, SsrElement};
use frender_reactive_value::{non_reactive::Uncached, temp_into_static::TempIntoStatic};

trait ToElementLikeUncachedTempIntoStatic<V: ?Sized + 'static>:
    for<'a> ToElement<
    ToElement<'a>: Element<
        HtmlChildren = <Uncached<TempIntoStatic<&'static V>> as SsrElement>::HtmlChildren,
        RenderStateKind = <Uncached<TempIntoStatic<&'static V>> as CsrElement>::RenderStateKind,
    >,
>
where
    TempIntoStatic<&'static V>: SsrElement,
    Uncached<TempIntoStatic<&'static V>>: CsrElement,
{
}

impl<T: ?Sized, V: ?Sized + 'static> ToElementLikeUncachedTempIntoStatic<V> for T
where
    T: for<'a> ToElement<
        ToElement<'a>: Element<
            HtmlChildren = <Uncached<TempIntoStatic<&'static V>> as SsrElement>::HtmlChildren,
            RenderStateKind = <Uncached<TempIntoStatic<&'static V>> as CsrElement>::RenderStateKind,
        >,
    >,
    TempIntoStatic<&'static V>: SsrElement,
    Uncached<TempIntoStatic<&'static V>>: CsrElement,
{
}

const fn type_assert()
where
    str: ToElementLikeUncachedTempIntoStatic<str>,
    String: ToElementLikeUncachedTempIntoStatic<str>,
    Cow<'static, str>: ToElementLikeUncachedTempIntoStatic<str>,
    (): TypeAssertCowStr,
    Rc<str>: ToElementLikeUncachedTempIntoStatic<Rc<str>>,
    Arc<str>: ToElementLikeUncachedTempIntoStatic<Arc<str>>,
{
}

const _: () = type_assert();

trait TypeAssertCowStr {
    type Assert<'a>: ToElement<
        ToElement<'a>: Element<
            HtmlChildren = <Uncached<TempIntoStatic<&'static str>> as SsrElement>::HtmlChildren,
            RenderStateKind = <Uncached<TempIntoStatic<&'static str>> as CsrElement>::RenderStateKind,
        >,
    >
    where
        Self: 'a;
}

impl TypeAssertCowStr for () {
    type Assert<'a>
        = Cow<'a, str>
    where
        Self: 'a;
}

// use the above code
#[test]
const fn compile_only() {
    type_assert()
}
