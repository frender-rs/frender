use frender_csr::render_state::compound::CompoundState;
use frender_html::{experimental, RenderStateKind};

use crate::fn_traits::{FnOnce1, FnOnce2};
use crate::memo::csr::Kind;

use super::super::CsrElementRenderUpdate;

/// As [`CsrElementRenderUpdate`], this type has the same
/// [`RenderStateKind`] as [`Memo<F, Dep>`].
///
/// It will always try to update render state with the element returned by `F` without the memoed dep updated.
pub struct MemoPhantom<F: for<'a> FnOnce1<&'a Dep>, Dep> {
    pub f: F,
    _dep: std::marker::PhantomData<Dep>,
}

impl<F: for<'a> FnOnce1<&'a Dep>, Dep> MemoPhantom<F, Dep> {
    pub const fn new(f: F) -> Self {
        Self {
            f,
            _dep: std::marker::PhantomData,
        }
    }
}

/// Works like [`MemoPhantom`]
pub struct MemoPhantomAndProvideFirstArgument<F: for<'a> FnOnce2<A, &'a Dep>, A, Dep> {
    pub f: F,
    pub first_argument: A,
    _dep: std::marker::PhantomData<Dep>,
}

impl<F: for<'a> FnOnce2<V, &'a Dep>, V, Dep> MemoPhantomAndProvideFirstArgument<F, V, Dep> {
    pub const fn new(f: F, first_argument: V) -> Self {
        Self {
            f,
            first_argument,
            _dep: std::marker::PhantomData,
        }
    }

    fn into_f(self) -> impl FnOnce(&Dep) -> <F as FnOnce2<V, &Dep>>::Output_ {
        move |dep: &_| (self.f)(self.first_argument, dep)
    }

    pub fn into_memo_phantom(
        self,
    ) -> MemoPhantom<impl FnOnce(&Dep) -> <F as FnOnce2<V, &Dep>>::Output_, Dep> {
        MemoPhantom::new(self.into_f())
    }
}

impl<F, Dep, K: RenderStateKind> CsrElementRenderUpdate for MemoPhantom<F, Dep>
where
    F: for<'a> FnOnce1<&'a Dep, Output: CsrElementRenderUpdate<RenderStateKind = K>>,
{
    type RenderStateKind = Kind<K, Dep>;

    fn pinned_render_update<Renderer: ?Sized + frender_html::RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: std::pin::Pin<&mut experimental::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut experimental::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        let CompoundState {
            reactive: state,
            non_reactive: dep,
        } = state.project_state();
        (self.f)(dep).pinned_render_update(renderer, state, ui_handle)
    }

    fn unpinned_render_update<Renderer: ?Sized + frender_html::RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        CompoundState {
            reactive: state,
            non_reactive: dep,
        }: &mut experimental::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut experimental::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        (self.f)(dep).unpinned_render_update(renderer, state, ui_handle)
    }
}

impl<F, V, Dep, K: RenderStateKind> CsrElementRenderUpdate
    for MemoPhantomAndProvideFirstArgument<F, V, Dep>
where
    F: for<'a> FnOnce2<V, &'a Dep, Output: CsrElementRenderUpdate<RenderStateKind = K>>,
{
    type RenderStateKind = Kind<K, Dep>;

    frender_html::proxy_csr_element_render_update!(|this| this.into_memo_phantom());
}
