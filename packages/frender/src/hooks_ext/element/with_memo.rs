use crate::{
    memoed::{MemoAndProvideFirstArgument, MemoPhantomAndProvideFirstArgument},
    FnMutMap2RefsToElement, Memo,
};

use super::{AsMutCsrElementWithValue, IntoAsMutCsrElementWithValue, IntoHtmlChildrenWithValue};

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

impl<V, F, Dep> AsMutCsrElementWithValue<V> for MemoCallWithRef<F, Dep>
where
    V: ?Sized,
    F: FnMutMap2RefsToElement<V, Dep>,
{
    type ElementWithValueRenderStateKind =
        crate::memoed::Kind<F::Refs2ToElementRenderStateKind, Dep>;

    type ElementWithValue<'a> = MemoPhantomAndProvideFirstArgument<&'a mut F, &'a V, Dep, true>
    where
        Self: 'a,
        V: 'a;

    fn as_mut_csr_element_with_value<'a>(&'a mut self, value: &'a V) -> Self::ElementWithValue<'a> {
        MemoPhantomAndProvideFirstArgument::new(&mut self.f, value)
    }
}

impl<V, F, Dep> IntoHtmlChildrenWithValue<V> for Memo<F, Dep>
where
    V: ?Sized,
    F: FnMutMap2RefsToElement<V, Dep>,
    Dep: PartialEq,
{
    type HtmlChildrenWithValue = F::Refs2ToElementHtmlChildren;

    fn into_html_children_with_value(mut self, value: &V) -> Self::HtmlChildrenWithValue {
        use crate::SsrElement as _;
        (self.0)(value, &self.1).into_html_children()
    }
}

impl<V, F, Dep> IntoAsMutCsrElementWithValue<V> for Memo<F, Dep>
where
    V: ?Sized,
    F: FnMutMap2RefsToElement<V, Dep>,
    Dep: PartialEq,
{
    type OwnedPart = Dep;

    type MutPart = MemoCallWithRef<F, Dep>;

    fn into_csr_parts(self) -> (Self::MutPart, Self::OwnedPart) {
        let Self(f, dep) = self;
        (MemoCallWithRef::new(f), dep)
    }

    type OwnedPartIntoCsrElement<'a> = MemoAndProvideFirstArgument<&'a mut F, &'a V, Dep>
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
