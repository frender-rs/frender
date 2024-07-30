pub use self::with::*;

use frender_element::Element;
use frender_html::RenderStateKind;
use frender_ssr::html::assert::HtmlChildren;

pub trait ToElement {
    type ToElementHtmlChildren: HtmlChildren;
    type ToElementRenderStateKind: RenderStateKind;
    type ToElement<'a>: Element<
        HtmlChildren = Self::ToElementHtmlChildren,
        RenderStateKind = Self::ToElementRenderStateKind,
    >
    where
        Self: 'a;
    fn to_element(&self) -> Self::ToElement<'_>;
}

impl<E: ?Sized + ToElement> ToElement for &E {
    type ToElementHtmlChildren = E::ToElementHtmlChildren;

    type ToElementRenderStateKind = E::ToElementRenderStateKind;

    type ToElement<'a>= E::ToElement<'a>
    where
        Self: 'a;

    fn to_element(&self) -> Self::ToElement<'_> {
        E::to_element(self)
    }
}

pub mod with {
    use frender_html::RenderStateKind;
    use frender_ssr::html::assert::HtmlChildren;

    use crate::{FnMapRefToElement, FnOnceOutputElement, FnOutputElement, ToElement};

    #[derive(Debug, Clone, Copy)]
    pub struct ToElementWithFn<E, F>(pub E, pub F);

    #[derive(Debug)]
    pub struct RefToElementWithFn<'a, E: ?Sized, F>(pub &'a E, pub F);

    impl<'a, E, F: Copy> Copy for RefToElementWithFn<'a, E, F> {}

    impl<'a, E, F: Clone> Clone for RefToElementWithFn<'a, E, F> {
        fn clone(&self) -> Self {
            Self(self.0, self.1.clone())
        }
    }

    impl<E, F> ToElement for ToElementWithFn<E, F>
    where
        F: FnMapRefToElement<E>,
    {
        type ToElement<'a> = <F as FnOnceOutputElement<&'a E>>::OutputElement
        where
            Self: 'a;

        type ToElementHtmlChildren = F::RefToElementHtmlChildren;
        type ToElementRenderStateKind = F::RefToElementRenderStateKind;

        fn to_element(&self) -> Self::ToElement<'_> {
            (self.1)(&self.0)
        }
    }

    impl<'e, E, F, C, K> ToElement for RefToElementWithFn<'e, E, F>
    where
        E: ?Sized,
        C: HtmlChildren,
        K: RenderStateKind,
        F: FnOutputElement<&'e E, OutputElementHtmlChildren = C, OutputElementRenderStateKind = K>,
    {
        type ToElementHtmlChildren = C;
        type ToElementRenderStateKind = K;

        type ToElement<'a> = <F as FnOnceOutputElement<&'e E>>::OutputElement
        where
            Self: 'a;

        fn to_element(&self) -> Self::ToElement<'_> {
            (self.1)(self.0)
        }
    }
}

mod imps {
    use crate::{CsrElement, SsrElement, TempStr};

    use super::ToElement;

    // scalar
    frender_common::impl_many!(
        impl<__> ToElement
            for each_of![
                i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64, //
                char,
            ]
        {
            type ToElementHtmlChildren = <Self as SsrElement>::HtmlChildren;
            type ToElementRenderStateKind = <Self as CsrElement>::RenderStateKind;
            type ToElement<'a> = Self
            where
                Self: 'a;
            fn to_element(&self) -> Self {
                *self
            }
        }
    );

    // acts like `TempStr<&'static str>`
    frender_common::impl_many!(
        impl<__> ToElement
            for each_of![
                str, //
                String,
                std::borrow::Cow<'_, str>,
            ]
        {
            type ToElementHtmlChildren = <TempStr<&'static str> as SsrElement>::HtmlChildren;
            type ToElementRenderStateKind = <TempStr<&'static str> as CsrElement>::RenderStateKind;
            type ToElement<'a> = TempStr<&'a Self>
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
            type ToElementHtmlChildren = <Self as SsrElement>::HtmlChildren;
            type ToElementRenderStateKind = <Self as CsrElement>::RenderStateKind;
            type ToElement<'a> = Self
            where
                Self: 'a;
            fn to_element(&self) -> Self {
                self.clone()
            }
        }
    );
}
