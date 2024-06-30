pub use self::with::*;

use frender_html::{Element, RenderStateKind};
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

    use crate::{FnMapRefToElement, FnOutputElement, ToElement};

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
        type ToElement<'a> = <F as FnOutputElement<&'a E>>::OutputElement
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

        type ToElement<'a> = <F as FnOutputElement<&'e E>>::OutputElement
        where
            Self: 'a;

        fn to_element(&self) -> Self::ToElement<'_> {
            (self.1)(self.0)
        }
    }
}
