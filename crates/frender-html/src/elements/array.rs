use frender_common::utils::pin_project_map_array;

use crate::{render_state::array::ArrayRenderState, Element, HtmlRenderContext, RenderHtml, RenderStateKind, RenderStateOfContext, UnpinnedRenderStateOfContext};

enum Never {}
pub struct Kind<K: RenderStateKind, const N: usize>(super::Kind<K>);

impl<K: RenderStateKind, const N: usize> crate::RenderStateKindPinned for Kind<K, N> {
    type RenderState<R: RenderHtml + ?Sized> = ArrayRenderState<K::RenderState<R>, N>;
}

impl<K: RenderStateKind, const N: usize> crate::RenderStateKindUnpinned for Kind<K, N> {
    type UnpinnedRenderState<R: RenderHtml + ?Sized> = ArrayRenderState<K::UnpinnedRenderState<R>, N>;
}

impl<E: Element, const N: usize> Element for [E; N] {
    type RenderStateKind = Kind<E::RenderStateKind, N>;

    fn render_update<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: std::pin::Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>) {
        let render_state = render_state.project_inner();
        let mut this = self.into_iter();
        pin_project_map_array(render_state, |state| this.next().unwrap().render_update(render_context, state));
        debug_assert!(this.next().is_none());
    }

    fn render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: std::pin::Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>) {
        let render_state = render_state.project_inner();
        let mut this = self.into_iter();
        pin_project_map_array(render_state, |state| this.next().unwrap().render_update_force_reposition(render_context, state));
        debug_assert!(this.next().is_none());
    }

    fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: std::pin::Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>, force_reposition: bool) {
        let render_state = render_state.project_inner();
        let mut this = self.into_iter();
        pin_project_map_array(render_state, |state| this.next().unwrap().render_update_maybe_reposition(render_context, state, force_reposition));
        debug_assert!(this.next().is_none());
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>) {
        let render_state = &mut render_state.0;

        let mut this = self.into_iter();
        let mut i = 0;

        while i < N {
            this.next().unwrap().unpinned_render_update(render_context, &mut render_state[i]);
            i += 1;
        }

        debug_assert!(this.next().is_none());
    }

    fn unpinned_render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>) {
        let render_state = &mut render_state.0;

        let mut this = self.into_iter();
        let mut i = 0;

        while i < N {
            this.next().unwrap().unpinned_render_update_force_reposition(render_context, &mut render_state[i]);
            i += 1;
        }

        debug_assert!(this.next().is_none());
    }

    fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>, force_reposition: bool) {
        let render_state = &mut render_state.0;

        let mut this = self.into_iter();
        let mut i = 0;

        while i < N {
            this.next().unwrap().unpinned_render_update_maybe_reposition(render_context, &mut render_state[i], force_reposition);
            i += 1;
        }

        debug_assert!(this.next().is_none());
    }
}
