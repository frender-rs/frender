use frender_csr::render_state::compound::CompoundState;
use frender_html::CsrElement;

use crate::{memoed::MemoAndProvideFirstArgument, FnMutMap2RefsToElement, Memo};

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

pub struct MemoCallWithRefAsMutElement<'a, F, Dep, V: ?Sized> {
    f: &'a mut F,
    value: &'a V,
    _dep: std::marker::PhantomData<Dep>,
}

impl<'a, F, Dep, V: ?Sized> CsrElement for MemoCallWithRefAsMutElement<'a, F, Dep, V>
where
    F: FnMutMap2RefsToElement<V, Dep>,
{
    type RenderStateKind = crate::memoed::Kind<F::Refs2ToElementRenderStateKind, Dep>;

    fn render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: std::pin::Pin<
            &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
        >,
    ) where
        Self: Sized,
    {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state.pin_project();

        (self.f)(self.value, dep.as_ref().unwrap()).render_update(render_context, render_state)
    }

    fn render_update_force_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: std::pin::Pin<
            &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
        >,
    ) where
        Self: Sized,
    {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state.pin_project();

        (self.f)(self.value, dep.as_ref().unwrap())
            .render_update_force_reposition(render_context, render_state)
    }

    fn render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: std::pin::Pin<
            &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
        >,
        force_reposition: bool,
    ) {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state.pin_project();

        (self.f)(self.value, dep.as_ref().unwrap()).render_update_maybe_reposition(
            render_context,
            render_state,
            force_reposition,
        )
    }

    fn unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
    ) where
        Self: Sized,
    {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state;

        (self.f)(self.value, dep.as_ref().unwrap())
            .unpinned_render_update(render_context, render_state)
    }

    fn unpinned_render_update_force_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
    ) where
        Self: Sized,
    {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state;

        (self.f)(self.value, dep.as_ref().unwrap())
            .unpinned_render_update_force_reposition(render_context, render_state)
    }

    fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
        force_reposition: bool,
    ) {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state;

        (self.f)(self.value, dep.as_ref().unwrap()).unpinned_render_update_maybe_reposition(
            render_context,
            render_state,
            force_reposition,
        )
    }
}

impl<V, F, Dep> AsMutCsrElementWithValue<V> for MemoCallWithRef<F, Dep>
where
    V: ?Sized,
    F: FnMutMap2RefsToElement<V, Dep>,
{
    type ElementWithValueRenderStateKind =
        crate::memoed::Kind<F::Refs2ToElementRenderStateKind, Dep>;

    type ElementWithValue<'a> = MemoCallWithRefAsMutElement<'a, F, Dep, V>
    where
        Self: 'a,
        V: 'a;

    fn as_mut_csr_element_with_value<'a>(&'a mut self, value: &'a V) -> Self::ElementWithValue<'a> {
        MemoCallWithRefAsMutElement {
            f: &mut self.f,
            value,
            _dep: self._dep,
        }
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
