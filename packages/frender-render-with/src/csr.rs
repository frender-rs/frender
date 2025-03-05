use std::{any::Any, marker::PhantomData};

use frender_csr::{
    experimental::{self, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    CsrElement,
};

pub trait RenderHtml: experimental::RenderHtml {}
impl<T: ?Sized + experimental::RenderHtml> RenderHtml for T {}

#[cfg(feature = "feature_impl_trait_in_assoc_type")]
mod nightly_impl;

#[cfg(not(feature = "feature_impl_trait_in_assoc_type"))]
mod not_nightly_impl;

/// This struct is a wrapper for render context so that it can only be used with
/// [`ctx.render(element)`](CsrRenderContext::render).
pub struct CsrRenderContext<'a, 'ctx, R: ?Sized + RenderHtml>(CsrRenderContextInner<'a, 'ctx, R>);

enum CsrRenderContextInner<'a, 'ctx, R: ?Sized + RenderHtml> {
    Update {
        renderer: &'a mut R,
        state: &'a mut (dyn 'static + Any),
        ui_handle: &'a mut (dyn 'static + Any),
    },
    Init(&'a mut R::RenderContext<'ctx>),
}

impl<'a, 'ctx, R: ?Sized + RenderHtml> CsrRenderContext<'a, 'ctx, R> {
    pub fn render<E: CsrElement>(self, element: E) -> Rendered<'a, 'ctx, E::RenderStateKind, R>
    where
        E::RenderStateKind: 'static,
    {
        Rendered(
            PhantomData,
            match self.0 {
                CsrRenderContextInner::Update {
                    renderer,
                    state,
                    ui_handle,
                } => {
                    element.unpinned_render_update(
                        renderer,
                        state.downcast_mut().unwrap(),
                        ui_handle.downcast_mut().unwrap(),
                    );

                    RenderedInner::Update
                }
                CsrRenderContextInner::Init(render_context) => RenderedInner::Init({
                    //
                    element.unpinned_render_init(render_context)
                }),
            },
        )
    }
}

/// A marker returned by [`CsrRenderContext::render`].
pub struct Rendered<'a, 'ctx, K: UnpinnedRenderStateKind, R: ?Sized + RenderHtml>(
    PhantomData<CsrRenderContext<'a, 'ctx, R>>,
    RenderedInner<K, R>,
);

enum RenderedInner<K: UnpinnedRenderStateKind, R: ?Sized + RenderHtml> {
    Update,
    Init(
        (
            experimental::UnpinnedStateOfKind<R, K>,
            experimental::UnpinnedUiHandleOfKind<R, K>,
        ),
    ),
}

/// This might be just
/// `for<'r,  Renderer: ?Sized + RenderHtml> FnMut(CsrRenderContext<'r,  Renderer, dyn Any>) -> Rendered<'r, impl Any>`
/// in the future.
pub trait IntoFnOnceRenderWithContext {
    fn into_fn_once_render_with_context<Renderer: ?Sized + RenderHtml>(
        self,
    ) -> impl FnOnceRenderWithContext<Renderer>;
    // TODO: `impl DefaultAnyRenderState<Renderer>` should not capture `'r`. See https://github.com/rust-lang/rust/issues/117210#issuecomment-2180030657
}

pub trait FnOnceRenderWithContext<Renderer: ?Sized + RenderHtml>:
    for<'r, 'ctx> FnOnce(
    CsrRenderContext<'r, 'ctx, Renderer>,
) -> Rendered<'r, 'ctx, Self::OutputRenderStateKind, Renderer>
{
    type OutputRenderStateKind: UnpinnedRenderStateKindPollRender + 'static;
}

impl<F, R: ?Sized + RenderHtml, K> FnOnceRenderWithContext<R> for F
where
    F: for<'r, 'ctx> FnOnce(CsrRenderContext<'r, 'ctx, R>) -> Rendered<'r, 'ctx, K, R>,
    K: UnpinnedRenderStateKindPollRender + 'static,
{
    type OutputRenderStateKind = K;
}
