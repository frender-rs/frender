use std::pin::Pin;

use frender_csr::{
    experimental::{self, kinds::KindUnpinned, HtmlRenderContext, RenderHtml},
    CsrElement,
};

use crate::{Keyed, KeyedElements};

pub trait KeyedElementsAlgorithm<K, E> {
    type KeyedElementsRenderStateKind: experimental::UnpinnedRenderStateKindPollRender;

    fn dummy_state<Renderer: ?Sized + RenderHtml>(
    ) -> experimental::UnpinnedStateOfKind<Renderer, Self::KeyedElementsRenderStateKind>;

    fn keyed_elements_render_init<
        I: IntoIterator<Item = Keyed<K, E>>,
        Ctx: ?Sized + HtmlRenderContext,
    >(
        self,
        elements: I,
        render_context: &mut Ctx,
    ) -> (
        experimental::UnpinnedStateOfKind<Ctx::Renderer, Self::KeyedElementsRenderStateKind>,
        experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::KeyedElementsRenderStateKind>,
    );

    fn keyed_elements_render_init_by_reusing<
        I: IntoIterator<Item = Keyed<K, E>>,
        Ctx: ?Sized + HtmlRenderContext,
    >(
        self,
        elements: I,
        render_context: &mut Ctx,
        reused_state: &mut experimental::UnpinnedStateOfKind<
            Ctx::Renderer,
            Self::KeyedElementsRenderStateKind,
        >,
        unmounted_ui_handle: experimental::UnpinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::KeyedElementsRenderStateKind,
        >,
    ) -> experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::KeyedElementsRenderStateKind>;

    fn keyed_elements_render_update<
        I: IntoIterator<Item = Keyed<K, E>>,
        Renderer: ?Sized + RenderHtml,
    >(
        self,
        elements: I,
        renderer: &mut Renderer,
        state: &mut experimental::UnpinnedStateOfKind<Renderer, Self::KeyedElementsRenderStateKind>,
        ui_handle: &mut experimental::UnpinnedUiHandleOfKind<
            Renderer,
            Self::KeyedElementsRenderStateKind,
        >,
    );
}

pub mod default;

pub struct RenderInit<I: IntoIterator, A>(KeyedElements<I, A>);

impl<
        K,
        E,
        I: IntoIterator<Item = Keyed<K, E>>,
        A: KeyedElementsAlgorithm<K, E>,
        Ctx: ?Sized + HtmlRenderContext,
    >
    experimental::RenderInitPinned<
        &mut Ctx,
        experimental::UnpinnedStateOfKind<Ctx::Renderer, A::KeyedElementsRenderStateKind>,
    > for RenderInit<I, A>
{
    type Output =
        experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, A::KeyedElementsRenderStateKind>;

    fn render_init_pinned(
        self,
        render_context: &mut Ctx,
        state: Pin<
            &mut experimental::UnpinnedStateOfKind<Ctx::Renderer, A::KeyedElementsRenderStateKind>,
        >,
    ) -> Self::Output {
        let ui_handle;
        (*state.get_mut(), ui_handle) = self.0.unpinned_render_init(render_context);
        ui_handle
    }
}

impl<I, A, E, K> CsrElement for KeyedElements<I, A>
where
    I: IntoIterator<Item = Keyed<K, E>>,
    A: KeyedElementsAlgorithm<K, E>,
{
    // TODO: refactor with impl_pinned_with_unpinned
    type RenderStateKind = KindUnpinned<A::KeyedElementsRenderStateKind>;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = RenderInit<I, A>;

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        _: &mut Renderer,
    ) -> (
        //
        experimental::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
    ) {
        (A::dummy_state::<Renderer>(), RenderInit(self))
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: Pin<
            &mut experimental::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        >,
        unmounted_ui_handle: experimental::PinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::RenderStateKind,
        >,
    ) -> experimental::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        self.unpinned_render_init_by_reusing(
            render_context,
            reused_state.get_mut(),
            unmounted_ui_handle,
        )
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut experimental::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut experimental::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        self.unpinned_render_update(renderer, state.get_mut(), ui_handle)
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> (
        //
        experimental::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) {
        self.algorithm
            .keyed_elements_render_init(self.iter, render_context)
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: &mut experimental::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        unmounted_ui_handle: experimental::UnpinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::RenderStateKind,
        >,
    ) -> experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        self.algorithm.keyed_elements_render_init_by_reusing(
            self.iter,
            render_context,
            reused_state,
            unmounted_ui_handle,
        )
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: &mut experimental::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut experimental::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        self.algorithm
            .keyed_elements_render_update(self.iter, renderer, state, ui_handle)
    }
}
