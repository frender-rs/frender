use std::{mem::MaybeUninit, pin::Pin, task::Poll};

use frender_html::RenderHtml;

pub trait RenderState<R> {
    fn unmount(self: Pin<&mut Self>, renderer: &mut R);

    fn state_unmount(self: Pin<&mut Self>);

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

pub trait FromUnpinned {
    type Unpinned;

    fn from_unpinned(this: &mut Self::Unpinned) -> Pin<&mut Self>;
}

// impl<T: Unpin> FromUnpinned for T {
//     type Unpinned = T;

//     fn from_unpinned(this: &mut Self::Unpinned) -> Pin<&mut Self> {
//         Pin::new(this)
//     }
// }

pub trait Element: frender_ssr::SsrElement {
    type RenderState<R: RenderHtml>: RenderState<R> + Default;

    #[cfg(feature = "render_into")]
    fn render_into<'s, Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: PinMutMaybeUninit<'s, Self::RenderState<Renderer>>,
    ) -> Pin<&'s mut Self::RenderState<Renderer>>;

    fn render_update<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
        self.render_update_maybe_reposition(renderer, render_state, false)
    }

    /// The element needs to be repositioned (re-add to the ctx)
    fn render_update_force_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
        self.render_update_maybe_reposition(renderer, render_state, true)
    }

    fn render_update_maybe_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
        force_reposition: bool,
    );

    type UnpinnedRenderState<R: RenderHtml>: RenderState<R> + Default + Unpin;

    fn unpinned_render_update<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
    ) where
        Self: Sized,
    {
        self.unpinned_render_update_maybe_reposition(renderer, render_state, false)
    }

    /// The element needs to be repositioned (re-add to the ctx)
    fn unpinned_render_update_force_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
    ) where
        Self: Sized,
    {
        self.unpinned_render_update_maybe_reposition(renderer, render_state, true)
    }

    fn unpinned_render_update_maybe_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
        force_reposition: bool,
    );
}

#[macro_export]
macro_rules! impl_unpinned_render_for_unpin {
    () => {
        type UnpinnedRenderState<R: $crate::__private::RenderHtml> = Self::RenderState<R>;

        fn unpinned_render_update<Renderer: $crate::__private::RenderHtml>(
            self,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
        ) where
            Self: Sized,
        {
            self.render_update(renderer, ::core::pin::Pin::new(render_state))
        }

        /// The element needs to be repositioned (re-add to the ctx)
        fn unpinned_render_update_force_reposition<Renderer: $crate::__private::RenderHtml>(
            self,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
        ) where
            Self: Sized,
        {
            self.render_update_force_reposition(renderer, ::core::pin::Pin::new(render_state))
        }

        fn unpinned_render_update_maybe_reposition<Renderer: $crate::__private::RenderHtml>(
            self,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
            force_reposition: bool,
        ) {
            self.render_update_maybe_reposition(
                renderer,
                ::core::pin::Pin::new(render_state),
                force_reposition,
            )
        }
    };
}
