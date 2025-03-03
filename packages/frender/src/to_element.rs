pub use self::with::{RefToElementWithFn, ToElementWithFn};

use frender_html::csr::CsrElement;
use frender_ssr::SsrElement;

pub trait ToElement {
    type ToElement<'a>
    where
        Self: 'a;
    fn to_element(&self) -> Self::ToElement<'_>;
}

impl<E: ?Sized + ToElement> ToElement for &E {
    type ToElement<'a>
        = E::ToElement<'a>
    where
        Self: 'a;

    fn to_element(&self) -> Self::ToElement<'_> {
        E::to_element(self)
    }
}

pub trait ToCsrElement: for<'a> ToElement<ToElement<'a>: CsrElement> {}
impl<E: ?Sized + for<'a> ToElement<ToElement<'a>: CsrElement>> ToCsrElement for E {}

pub trait ToSsrElement: for<'a> ToElement<ToElement<'a>: SsrElement> {}
impl<E: ?Sized + for<'a> ToElement<ToElement<'a>: SsrElement>> ToSsrElement for E {}

pub mod with {
    use crate::fn_traits::{Fn1, FnOnce1};

    use super::ToElement;

    #[derive(Debug, Clone, Copy)]
    pub struct ToElementWithFn<E, F: for<'e> Fn1<&'e E>>(pub E, pub F);

    impl<E, F: for<'e> Fn1<&'e E>> ToElement for ToElementWithFn<E, F> {
        type ToElement<'a>
            = <F as FnOnce1<&'a E>>::Output_
        where
            Self: 'a;

        fn to_element(&self) -> Self::ToElement<'_> {
            (self.1)(&self.0)
        }
    }

    #[derive(Debug)]
    pub struct RefToElementWithFn<'e, E: ?Sized, F: ?Sized + Fn1<&'e E>>(pub &'e E, pub F);

    impl<'e, E, F: Copy + Fn1<&'e E>> Copy for RefToElementWithFn<'e, E, F> {}

    impl<'e, E, F: Clone + Fn1<&'e E>> Clone for RefToElementWithFn<'e, E, F> {
        fn clone(&self) -> Self {
            Self(self.0, self.1.clone())
        }
    }

    impl<'e, E: ?Sized, F: ?Sized + Fn1<&'e E>> ToElement for RefToElementWithFn<'e, E, F> {
        type ToElement<'a>
            = <F as FnOnce1<&'e E>>::Output_
        where
            Self: 'a;

        fn to_element(&self) -> Self::ToElement<'_> {
            (self.1)(self.0)
        }
    }
}

mod imps {
    use frender_reactive_value::{
        non_reactive::Uncached, temp_into_static::TempIntoStatic,
    };

    use super::ToElement;

    // scalar
    frender_common::impl_many!(
        impl<__> ToElement
            for each_of![
                i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64, //
                char,
            ]
        {
            type ToElement<'a>
                = Self
            where
                Self: 'a;
            fn to_element(&self) -> Self {
                *self
            }
        }
    );

    // acts like `Uncached<TempIntoStatic<&'a str>>`
    frender_common::impl_many!(
        impl<__> ToElement
            for each_of![
                //
                str,
                String,
                std::borrow::Cow<'_, str>,
            ]
        {
            type ToElement<'a>
                = Uncached<TempIntoStatic<&'a str>>
            where
                Self: 'a;
            fn to_element(&self) -> Self::ToElement<'_> {
                Uncached(TempIntoStatic(self))
            }
        }
    );

    // CheapClone
    frender_common::impl_many!(
        impl<__> ToElement
            for each_of![
                std::rc::Rc<str>, //
                std::sync::Arc<str>,
            ]
        {
            type ToElement<'a>
                = Uncached<TempIntoStatic<&'a Self>>
            where
                Self: 'a;
            fn to_element(&self) -> Self::ToElement<'_> {
                Uncached(TempIntoStatic(self))
            }
        }
    );
}

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, rc::Rc, sync::Arc};

    use super::ToElement;
    use frender_element::Element;
    use frender_html::csr::CsrElement;
    use frender_reactive_value::{
        non_reactive::Uncached, temp_into_static::TempIntoStatic,
    };
    use frender_ssr::SsrElement;

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
}
