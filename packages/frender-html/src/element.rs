use std::pin::Pin;

use frender_dom::render::RenderContext;

use crate::{RenderHtml, RenderState};

pub trait RenderStateKindPinned {
    type RenderState<R: RenderHtml + ?Sized>: RenderState<R> + Default;
}

pub trait RenderStateKindUnpinned {
    type UnpinnedRenderState<R: RenderHtml + ?Sized>: RenderState<R> + Default + Unpin;
}

pub trait RenderStateKind: RenderStateKindPinned + RenderStateKindUnpinned {}

impl<K: ?Sized + RenderStateKindPinned + RenderStateKindUnpinned> RenderStateKind for K {}

pub trait HtmlRenderContext: RenderContext<Renderer = Self::HtmlRenderer> {
    type HtmlRenderer: RenderHtml + ?Sized;
}

impl<Ctx: ?Sized + RenderContext> HtmlRenderContext for Ctx
where
    Ctx::Renderer: RenderHtml,
{
    type HtmlRenderer = Ctx::Renderer;
}

pub type RenderStateOfContext<K, Ctx> = <K as RenderStateKindPinned>::RenderState<<Ctx as RenderContext>::Renderer>;
pub type UnpinnedRenderStateOfContext<K, Ctx> = <K as RenderStateKindUnpinned>::UnpinnedRenderState<<Ctx as RenderContext>::Renderer>;

pub trait CsrElement {
    type RenderStateKind: RenderStateKind;

    fn render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>,
    ) where
        Self: Sized,
    {
        self.render_update_maybe_reposition(render_context, render_state, false)
    }

    /// The element needs to be repositioned (re-add to the ctx)
    fn render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>,
    ) where
        Self: Sized,
    {
        self.render_update_maybe_reposition(render_context, render_state, true)
    }

    fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>,
        force_reposition: bool,
    );

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
    ) where
        Self: Sized,
    {
        self.unpinned_render_update_maybe_reposition(render_context, render_state, false)
    }

    /// The element needs to be repositioned (re-add to the ctx)
    fn unpinned_render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
    ) where
        Self: Sized,
    {
        self.unpinned_render_update_maybe_reposition(render_context, render_state, true)
    }

    fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
        force_reposition: bool,
    );
}

#[macro_export]
macro_rules! impl_unpinned_render_for_unpin {
    () => {
        fn unpinned_render_update<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: &mut $crate::RenderStateOfContext<Self::RenderStateKind, Ctx>,
        ) where
            Self: Sized,
        {
            self.render_update(render_context, ::core::pin::Pin::new(render_state))
        }

        /// The element needs to be repositioned (re-add to the ctx)
        fn unpinned_render_update_force_reposition<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: &mut $crate::RenderStateOfContext<Self::RenderStateKind, Ctx>,
        ) where
            Self: Sized,
        {
            self.render_update_force_reposition(render_context, ::core::pin::Pin::new(render_state))
        }

        fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: &mut $crate::RenderStateOfContext<Self::RenderStateKind, Ctx>,
            force_reposition: ::core::primitive::bool,
        ) {
            self.render_update_maybe_reposition(render_context, ::core::pin::Pin::new(render_state), force_reposition)
        }
    };
}

#[macro_export]
macro_rules! impl_render_for_unpin {
    () => {
        fn render_update<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: ::core::pin::Pin<&mut $crate::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
        ) where
            Self: Sized,
        {
            self.unpinned_render_update(render_context, render_state.get_mut())
        }

        fn render_update_force_reposition<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: ::core::pin::Pin<&mut $crate::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
        ) where
            Self: Sized,
        {
            self.unpinned_render_update_force_reposition(render_context, render_state.get_mut())
        }

        fn render_update_maybe_reposition<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: ::core::pin::Pin<&mut $crate::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
            force_reposition: ::core::primitive::bool,
        ) {
            self.unpinned_render_update_maybe_reposition(render_context, render_state.get_mut(), force_reposition)
        }
    };
}

#[macro_export]
macro_rules! proxy_csr_element {
    (|$this:pat_param| $expr:expr) => {
        fn unpinned_render_update<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: &mut $crate::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
        ) where
            Self: Sized,
        {
            let $this = self;
            $expr.unpinned_render_update(render_context, render_state);
        }

        /// The element needs to be repositioned (re-add to the ctx)
        fn unpinned_render_update_force_reposition<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: &mut $crate::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
        ) where
            Self: Sized,
        {
            let $this = self;
            $expr.unpinned_render_update_force_reposition(render_context, render_state);
        }

        fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: &mut $crate::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
            force_reposition: ::core::primitive::bool,
        ) {
            let $this = self;
            $expr.unpinned_render_update_maybe_reposition(render_context, render_state, force_reposition);
        }

        fn render_update<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: ::core::pin::Pin<&mut $crate::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
        ) where
            Self: Sized,
        {
            let $this = self;
            $expr.render_update(render_context, render_state);
        }

        fn render_update_force_reposition<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: ::core::pin::Pin<&mut $crate::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
        ) where
            Self: Sized,
        {
            let $this = self;
            $expr.render_update_force_reposition(render_context, render_state);
        }

        fn render_update_maybe_reposition<Ctx: ?Sized + $crate::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: ::core::pin::Pin<&mut $crate::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
            force_reposition: ::core::primitive::bool,
        ) {
            let $this = self;
            $expr.render_update_maybe_reposition(render_context, render_state, force_reposition);
        }
    };
}

#[cfg(any(test, doctest))]
mod tests {
    /// ```compile_fail
    /// # use frender_element::Element;
    /// # fn __(v: (impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,)) -> impl Element {
    /// #   v
    /// # }
    /// # _ = __((1,2,3,4,5,6,7,8,9,10,11,12,13));
    /// ```
    enum _TupleMaxElements {}

    #[test]
    fn tuple_max_elements() {
        use crate::Element;
        fn __(
            v: (
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
            ),
        ) -> impl Element {
            v
        }
        _ = __((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));
    }
}
