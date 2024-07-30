use frender_csr::render_state::compound::CompoundState;
use frender_html::CsrElement;

use crate::{FnMutMap2RefsToElement, FnOnce2OutputElement, Memo};

use super::{IntoMutElementWithValue, MutCsrElementWithValue};

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

impl<V, F, Dep> MutCsrElementWithValue<V> for MemoCallWithRef<F, Dep>
where
    V: ?Sized,
    F: FnMutMap2RefsToElement<V, Dep>,
{
    type RenderStateKind = crate::memoed::Kind<F::Refs2ToElementRenderStateKind, Dep>;

    fn mut_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        &mut self,
        value: &V,
        render_context: &mut Ctx,
        render_state: std::pin::Pin<
            &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
        >,
    ) {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state.pin_project();

        (self.f)(value, dep.as_ref().unwrap()).render_update(render_context, render_state)
    }

    fn mut_render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        &mut self,
        value: &V,
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

        (self.f)(value, dep.as_ref().unwrap()).render_update_maybe_reposition(
            render_context,
            render_state,
            force_reposition,
        )
    }

    fn mut_unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        &mut self,
        value: &V,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
    ) {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state;

        (self.f)(value, dep.as_ref().unwrap()).unpinned_render_update(render_context, render_state)
    }

    fn mut_unpinned_render_update_maybe_reposition<
        Ctx: ?Sized + frender_html::HtmlRenderContext,
    >(
        &mut self,
        value: &V,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
        force_reposition: bool,
    ) {
        let CompoundState {
            reactive: render_state,
            non_reactive: dep,
        } = render_state;

        (self.f)(value, dep.as_ref().unwrap()).unpinned_render_update_maybe_reposition(
            render_context,
            render_state,
            force_reposition,
        )
    }
}

#[inline(always)]
fn closure_2_to_1<'a, Value: ?Sized, Dep: ?Sized, F: FnMutMap2RefsToElement<Value, Dep>>(
    mut f: F,
    value: &'a Value,
) -> impl for<'d> FnOnce(&'d Dep) -> <F as FnOnce2OutputElement<&'a Value, &'d Dep>>::OutputElement
{
    move |dep: &_| f(value, dep)
}

impl<V, F, Dep> IntoMutElementWithValue<V> for Memo<F, Dep>
where
    V: ?Sized,
    F: FnMutMap2RefsToElement<V, Dep>,
    Dep: PartialEq,
{
    type HtmlChildren = F::Refs2ToElementHtmlChildren;

    fn into_html_children_with_value(mut self, value: &V) -> Self::HtmlChildren {
        use crate::SsrElement;
        (self.0)(value, &self.1).into_html_children()
    }

    type MutCsrElementWithValue = MemoCallWithRef<F, Dep>;

    fn render_update_maybe_reposition_with_value_and_into<
        Ctx: ?Sized + frender_html::HtmlRenderContext,
    >(
        self,
        value: &V,
        render_context: &mut Ctx,
        render_state: std::pin::Pin<
            &mut frender_html::RenderStateOfContext<
                <Self::MutCsrElementWithValue as super::MutCsrElementWithValue<V>>::RenderStateKind,
                Ctx,
            >,
        >,
        force_reposition: bool,
    ) -> Self::MutCsrElementWithValue {
        let Self(mut f, dep) = self;
        Memo(closure_2_to_1(&mut f, value), dep).render_update_maybe_reposition(
            render_context,
            render_state,
            force_reposition,
        );
        MemoCallWithRef {
            f,
            _dep: std::marker::PhantomData,
        }
    }

    fn unpinned_render_update_maybe_reposition_with_value_and_into<
        Ctx: ?Sized + frender_html::HtmlRenderContext,
    >(
        self,
        value: &V,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<
            <Self::MutCsrElementWithValue as super::MutCsrElementWithValue<V>>::RenderStateKind,
            Ctx,
        >,
        force_reposition: bool,
    ) -> Self::MutCsrElementWithValue {
        let Self(mut f, dep) = self;
        Memo(closure_2_to_1(&mut f, value), dep).unpinned_render_update_maybe_reposition(
            render_context,
            render_state,
            force_reposition,
        );
        MemoCallWithRef {
            f,
            _dep: std::marker::PhantomData,
        }
    }
}
