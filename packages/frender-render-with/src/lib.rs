#![cfg_attr(feature = "nightly", feature(impl_trait_in_assoc_type))]

use std::{any::Any, marker::PhantomData, pin::Pin};

use frender_html::{
    dom::{
        render::RenderWithContext,
        ui_handle::{UiHandle, UnmountedUiHandle},
    },
    experimental::{UnpinnedRenderStateKind, UnpinnedRenderStatesOfKind},
    CsrElement, HtmlRenderContext, RenderHtml,
};
use frender_ssr::SsrElement;

/// This struct is a wrapper for render context so that it can only be used with
/// [`ctx.render(element)`](CsrRenderContext::render).
pub struct CsrRenderContext<'a, Ctx: ?Sized + HtmlRenderContext> {
    render_context: &'a mut Ctx,
}

impl<'a, Ctx: ?Sized + HtmlRenderContext> CsrRenderContext<'a, Ctx> {
    pub fn render<E: CsrElement>(self, element: E) -> Rendered<'a, E::RenderStateKind, Ctx> {
        Rendered(
            element.unpinned_render_init(self.render_context),
            PhantomData,
        )
    }
}

/// A marker returned by [`CsrRenderContext::render`].
pub struct Rendered<'a, K: UnpinnedRenderStateKind, Ctx: ?Sized + HtmlRenderContext>(
    UnpinnedRenderStatesOfKind<K, Ctx::Renderer>,
    PhantomData<&'a mut ()>,
);

// #[cfg(not(feature = "nightly"))]
// impl<S> Rendered<'_, S> {
//     fn type_check(self, _: PhantomData<S>) {}
// }

pub struct RenderWith<F>(pub F);

// TODO: implement ssr with csr
impl<F> SsrElement for RenderWith<F> {
    type HtmlChildren = async_str_iter::empty::Empty;

    fn into_html_children(self) -> Self::HtmlChildren {
        async_str_iter::empty::Empty
    }
}

/// This might be just
/// `for<'r,  Renderer: ?Sized + RenderHtml> FnMut(CsrRenderContext<'r,  Renderer, dyn Any>) -> Rendered<'r, impl Any>`
/// in the future.
pub trait IntoFnOnceRenderWithContext {
    fn into_fn_once_render_with_context<Ctx: ?Sized + HtmlRenderContext>(
        self,
    ) -> impl FnOnceRenderWithContext<Ctx>;
    // TODO: `impl DefaultAnyRenderState<Renderer>` should not capture `'r`. See https://github.com/rust-lang/rust/issues/117210#issuecomment-2180030657
}

// trait Helper<Renderer: ?Sized + RenderHtml>: FnOnce(T) -> Self::FnOnceRenderWithContext {
//     type FnOnceRenderWithContext: FnOnceRenderWithContext<Renderer>;
// }

// TODO: RenderWith(Test::default())

pub trait FnOnceRenderWithContext<Ctx: ?Sized + HtmlRenderContext>:
    for<'r> FnOnce(CsrRenderContext<'r, Ctx>) -> Rendered<'r, Self::OutputRenderStateKind, Ctx>
{
    type OutputRenderStateKind: UnpinnedRenderStateKind + 'static;
}

impl<F, Ctx: ?Sized + HtmlRenderContext, K> FnOnceRenderWithContext<Ctx> for F
where
    F: for<'r> FnOnce(CsrRenderContext<'r, Ctx>) -> Rendered<'r, K, Ctx>,
    K: UnpinnedRenderStateKind + 'static,
{
    type OutputRenderStateKind = K;
}

#[cfg(feature = "nightly")]
mod nightly_impl {
    use frender_html::RenderStateKindPinned;

    use super::*;

    pub trait NamedIntoFnOnceRenderWithContext: IntoFnOnceRenderWithContext {
        // TODO: should not be generic over R
        type RenderedRenderStateKind<R: frender_html::RenderHtml + ?Sized>: RenderStateKind;

        fn _named_into_fn_once_render_with_context_helper<R: frender_html::RenderHtml + ?Sized>(
            self,
        ) -> PhantomData<Self::RenderedRenderStateKind<R>>;
    }

    impl<F: IntoFnOnceRenderWithContext> NamedIntoFnOnceRenderWithContext for F {
        type RenderedRenderStateKind<R: frender_html::RenderHtml + ?Sized> =
            impl 'static + RenderStateKind;

        fn _named_into_fn_once_render_with_context_helper<R: frender_html::RenderHtml + ?Sized>(
            self,
        ) -> PhantomData<Self::RenderedRenderStateKind<R>> {
            fn test<'a, Renderer: ?Sized + RenderHtml, K>(
                _: impl FnOnceRenderWithContext<Renderer::RenderContext<'a>, OutputRenderStateKind = K>,
            ) -> PhantomData<K> {
                PhantomData
            }
            test::<R, _>(self.into_fn_once_render_with_context::<R::RenderContext<'_>>())
        }
    }

    enum Never {}
    pub struct Kind<F: NamedIntoFnOnceRenderWithContext>(Never, PhantomData<F>);

    impl<F: NamedIntoFnOnceRenderWithContext> RenderStateKindPinned for Kind<F> {
        type RenderState<R: RenderHtml + ?Sized> =
            <F::RenderedRenderStateKind<R> as RenderStateKindUnpinned>::UnpinnedRenderState<R>;
    }

    impl<F: NamedIntoFnOnceRenderWithContext> RenderStateKindUnpinned for Kind<F> {
        type UnpinnedRenderState<R: RenderHtml + ?Sized> =
            <F::RenderedRenderStateKind<R> as RenderStateKindUnpinned>::UnpinnedRenderState<R>;
    }

    impl<F: IntoFnOnceRenderWithContext> Element for RenderWith<F> {
        type RenderStateKind = Kind<F>;

        fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: Pin<&mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
            force_reposition: bool,
        ) {
            let render_state = render_state.get_mut();

            let f = self.0.into_fn_once_render_with_context();
            f(CsrRenderContext {
                render_context,
                render_state: render_state as &mut (dyn 'static + Any),
                force_reposition,
            });
        }

        impl_unpinned_render_for_unpin! {}
    }
}

#[cfg(not(feature = "nightly"))]
mod not_nightly_impl;
