pub use self::with::{RefToElementWithFn, ToElementWithFn};

use frender_html::CsrElement;
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
    use crate::TempStr;

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

    // acts like `TempStr<&'static str>`
    impl ToElement for str {
        type ToElement<'a>
            = TempStr<&'a str>
        where
            Self: 'a;

        fn to_element(&self) -> Self::ToElement<'_> {
            TempStr(self)
        }
    }

    // acts like `TempStr<&'static String>`
    frender_common::impl_many!(
        impl<__> ToElement
            for each_of![
                //
                String,
                std::borrow::Cow<'_, str>,
            ]
        {
            type ToElement<'a>
                = TempStr<&'a Self>
            where
                Self: 'a;
            fn to_element(&self) -> TempStr<&Self> {
                TempStr(self)
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
                = Self
            where
                Self: 'a;
            fn to_element(&self) -> Self {
                self.clone()
            }
        }
    );
}

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, rc::Rc, sync::Arc};

    use super::ToElement;
    use frender_common::TempStr;
    use frender_element::Element;
    use frender_html::CsrElement;
    use frender_ssr::SsrElement;

    const fn type_assert()
    where
        str: for<'a> ToElement<
            ToElement<'a>: Element<
                HtmlChildren = <TempStr<&'static str> as SsrElement>::HtmlChildren,
                RenderStateKind = <TempStr<&'static str> as CsrElement>::RenderStateKind,
            >,
        >,
        String: for<'a> ToElement<
            ToElement<'a>: Element<
                HtmlChildren = <TempStr<&'static String> as SsrElement>::HtmlChildren,
                RenderStateKind = <TempStr<&'static String> as CsrElement>::RenderStateKind,
            >,
        >,
        Cow<'static, str>: for<'a> ToElement<
            ToElement<'a>: Element<
                HtmlChildren = <TempStr<&'static String> as SsrElement>::HtmlChildren,
                RenderStateKind = <TempStr<&'static String> as CsrElement>::RenderStateKind,
            >,
        >,
        for<'c> Cow<'c, str>: TypeAssertCow,
        Rc<str>: for<'a> ToElement<
            ToElement<'a>: Element<
                HtmlChildren = <Rc<str> as SsrElement>::HtmlChildren,
                RenderStateKind = <Rc<str> as CsrElement>::RenderStateKind,
            >,
        >, //
        Arc<str>: for<'a> ToElement<
            ToElement<'a>: Element<
                HtmlChildren = <Arc<str> as SsrElement>::HtmlChildren,
                RenderStateKind = <Arc<str> as CsrElement>::RenderStateKind,
            >,
        >,
    {
    }

    trait TypeAssertCow {
        type Expected<'a>: ToElement<
            ToElement<'a>: Element<
                HtmlChildren = <TempStr<&'static String> as SsrElement>::HtmlChildren,
                RenderStateKind = <TempStr<&'static String> as CsrElement>::RenderStateKind,
            >,
        >
        where
            Self: 'a;
    }

    impl<'c> TypeAssertCow for Cow<'c, str> {
        type Expected<'a>
            = Cow<'c, str>
        where
            'c: 'a;
    }

    const _: () = type_assert();

    // use the above code
    #[test]
    const fn compile_only() {
        type_assert()
    }
}
