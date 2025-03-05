use frender_fn_traits::FnMut2;

use super::bound::MapValueToElement;

/// `F: FnMut(&_, &Dep) -> _`
pub struct WithMemo<F, Dep>(pub F, pub Dep);

impl<F: for<'a, 'b> FnMut2<&'a V, &'b Dep>, Dep, V: ?Sized> MapValueToElement<V>
    for WithMemo<F, Dep>
{
}

pub struct MemoCallWithRef<F, Dep> {
    f: F,
    _dep: std::marker::PhantomData<Dep>,
}

impl<F, Dep> MemoCallWithRef<F, Dep> {
    pub const fn new(f: F) -> Self {
        Self {
            f,
            _dep: std::marker::PhantomData,
        }
    }
}

#[cfg(feature = "csr")]
mod csr {
    use frender_csr::{CsrElement, RenderStateKind};
    use frender_memo::{MemoAndProvideFirstArgument, csr_experimental::Kind};

    use frender_fn_traits::FnMut2;

    use super::{
        super::bound::csr::{AsMutCsrElementWithValue, MapValueToCsrElement},
        MemoCallWithRef, WithMemo,
    };

    use self::memo_phantom::MemoPhantomAndProvideFirstArgument;

    mod memo_phantom;

    impl<V, F, Dep, K> AsMutCsrElementWithValue<V> for MemoCallWithRef<F, Dep>
    where
        V: ?Sized,
        F: for<'a, 'b> FnMut2<&'a V, &'b Dep, Output: CsrElement<RenderStateKind = K>>,
        K: RenderStateKind,
    {
        type ElementWithValueRenderStateKind = Kind<K, Dep>;

        type ElementWithValue<'a>
            = MemoPhantomAndProvideFirstArgument<&'a mut F, &'a V, Dep>
        where
            Self: 'a,
            V: 'a;

        fn as_mut_csr_element_with_value<'a>(
            &'a mut self,
            value: &'a V,
        ) -> Self::ElementWithValue<'a> {
            MemoPhantomAndProvideFirstArgument::new(&mut self.f, value)
        }
    }

    impl<V, F, Dep, K> MapValueToCsrElement<V> for WithMemo<F, Dep>
    where
        V: ?Sized,
        F: for<'a, 'b> FnMut2<&'a V, &'b Dep, Output: CsrElement<RenderStateKind = K>>,
        Dep: PartialEq,
        K: RenderStateKind,
    {
        type OwnedPart = Dep;

        type MutPart = MemoCallWithRef<F, Dep>;

        fn into_csr_parts(self) -> (Self::MutPart, Self::OwnedPart) {
            let Self(f, dep) = self;
            (MemoCallWithRef::new(f), dep)
        }

        type OwnedPartIntoCsrElement<'a>
            = MemoAndProvideFirstArgument<&'a mut F, &'a V, Dep>
        where
            Self: 'a,
            V: 'a;

        fn owned_part_into_csr_element<'a>(
            mut_part: &'a mut Self::MutPart,
            value: &'a V,
            owned_part: Self::OwnedPart,
        ) -> Self::OwnedPartIntoCsrElement<'a> {
            MemoAndProvideFirstArgument(&mut mut_part.f, value, owned_part)
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use frender_fn_traits::FnMut2;
    use frender_ssr::{SsrElement, html::assert::HtmlChildren};

    use super::{super::bound::ssr::MapValueToSsrElement, WithMemo};

    impl<V, F, Dep, HC> MapValueToSsrElement<V> for WithMemo<F, Dep>
    where
        V: ?Sized,
        F: for<'a, 'b> FnMut2<&'a V, &'b Dep, Output: SsrElement<HtmlChildren = HC>>,
        Dep: PartialEq,
        HC: HtmlChildren,
    {
        type HtmlChildrenWithValue = HC;

        fn into_html_children_with_value(mut self, value: &V) -> Self::HtmlChildrenWithValue {
            use frender_ssr::SsrElement as _;
            (self.0)(value, &self.1).into_html_children()
        }
    }
}
