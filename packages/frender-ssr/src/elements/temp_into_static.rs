use std::borrow::Borrow;

use frender_common::impl_many;
use frender_reactive_value::temp_into_static::{IntoStaticWithKind, TempIntoStatic};

use crate::SsrElement;

impl<T: IntoStaticWithKind> SsrElement for TempIntoStatic<T>
where
    T::IntoStaticValue: LendSsrElement,
{
    type HtmlChildren =
        HtmlChildrenOf<<T::IntoStaticValue as LendSsrElement>::LendSsrElement<T::IntoStatic>>;

    fn into_html_children(self) -> Self::HtmlChildren {
        T::IntoStaticValue::lend_ssr_element(self.0.into_static()).into_html_children()
    }
}

impl<T: IntoStaticWithKind> KnownCopySsrElement for TempIntoStatic<T>
where
    T::IntoStaticValue: LendSsrElement,
    T: Copy,
{
}

type HtmlChildrenOf<T> = <T as SsrElement>::HtmlChildren;

pub trait LendSsrElement: 'static {
    type LendSsrElement<B: 'static + Borrow<Self>>: SsrElement;
    fn lend_ssr_element<B: 'static + Borrow<Self>>(value: B) -> Self::LendSsrElement<B>;
}

pub(crate) trait KnownCopySsrElement: Copy + SsrElement {}

impl<T: KnownCopySsrElement + 'static> LendSsrElement for T {
    type LendSsrElement<B: 'static + Borrow<Self>> = T;
    fn lend_ssr_element<B: 'static + Borrow<Self>>(value: B) -> Self::LendSsrElement<B> {
        *value.borrow()
    }
}

impl LendSsrElement for str {
    type LendSsrElement<B: 'static + Borrow<Self>> = BorrowStrIntoSsrElement<B>;

    fn lend_ssr_element<B: 'static + Borrow<Self>>(value: B) -> Self::LendSsrElement<B> {
        BorrowStrIntoSsrElement(value)
    }
}

// CheapClone
impl_many!(
    impl<__> LendSsrElement
        for each_of![
            //
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type LendSsrElement<B: 'static + Borrow<Self>> = Self;
        fn lend_ssr_element<B: 'static + Borrow<Self>>(value: B) -> Self {
            Self::clone(value.borrow())
        }
    }
);

pub struct BorrowStrIntoSsrElement<T: Borrow<str>>(T);

impl<T: Borrow<str>> SsrElement for BorrowStrIntoSsrElement<T> {
    type HtmlChildren = frender_ssr_html::encode::Encode<
        frender_ssr_html::escape_safe::Safe,
        async_str_iter::borrow_str::IterBorrowStr<T>,
    >;

    fn into_html_children(self) -> Self::HtmlChildren {
        Self::HtmlChildren::new(
            frender_ssr_html::escape_safe::Safe,
            async_str_iter::borrow_str::IterBorrowStr::new(self.0),
        )
    }
}
