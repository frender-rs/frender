use frender_html::{
    experimental::{self, RenderStates},
    kinds::UiHandleWithNonReactiveState,
    RenderStateKind,
};

use crate::{
    fn_traits::{FnOnce1, FnOnce2},
    hooks_ext::element::CsrElementRenderUpdate,
};

use super::{
    super::{MemoPhantom, MemoPhantomAndProvideFirstArgument},
    Kind,
};

impl<F, Dep, K: RenderStateKind> CsrElementRenderUpdate for MemoPhantom<F, Dep>
where
    F: for<'a> FnOnce1<&'a Dep, Output: CsrElementRenderUpdate<RenderStateKind = K>>,
{
    type RenderStateKind = Kind<K, Dep>;

    fn pinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        RenderStates {
            ui_handle:
                UiHandleWithNonReactiveState {
                    ui_handle,
                    non_reactive_state: dep,
                },
            non_reactive_state,
            reactive_state,
        }: experimental::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        (self.f)(dep).pinned_render_update(
            render_context,
            RenderStates {
                ui_handle,
                non_reactive_state,
                reactive_state,
            },
        )
    }

    fn unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        RenderStates {
            ui_handle:
                UiHandleWithNonReactiveState {
                    ui_handle,
                    non_reactive_state: dep,
                },
            non_reactive_state,
            reactive_state,
        }: experimental::UnpinnedMutRenderStatesOfKind<
            Self::RenderStateKind,
            Ctx::Renderer,
        >,
    ) {
        (self.f)(dep).unpinned_render_update(
            render_context,
            RenderStates {
                ui_handle,
                non_reactive_state,
                reactive_state,
            },
        )
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
