// pub struct Chain<A, B>(A, B);

use crate::element::html::HtmlAttrPair;

pub trait IntoIteratorAttrs<'a>: Sized {
    type IntoIterAttrs: Iterator<Item = HtmlAttrPair<'a>>;
    fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs;
}

impl<'a, const N: usize> IntoIteratorAttrs<'a> for [HtmlAttrPair<'a>; N] {
    type IntoIterAttrs = std::array::IntoIter<HtmlAttrPair<'a>, N>;

    fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
        this.into_iter()
    }
}

impl<'a> IntoIteratorAttrs<'a> for () {
    type IntoIterAttrs = std::iter::Empty<HtmlAttrPair<'a>>;

    #[inline]
    fn into_iter_attrs(_: Self) -> Self::IntoIterAttrs {
        std::iter::empty()
    }
}

impl<'a, A: IntoIteratorAttrs<'a>, B: IntoIteratorAttrs<'a>> IntoIteratorAttrs<'a> for (A, B) {
    type IntoIterAttrs = std::iter::Chain<A::IntoIterAttrs, B::IntoIterAttrs>;

    fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
        A::into_iter_attrs(this.0).chain(B::into_iter_attrs(this.1))
    }
}
