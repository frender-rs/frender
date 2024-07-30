#![cfg_attr(feature = "nightly", feature(impl_trait_in_assoc_type))]

use std::{any::Any, marker::PhantomData, pin::Pin};

use frender_html::{
    impl_unpinned_render_for_unpin, CsrElement as Element, HtmlRenderContext, RenderHtml,
    RenderState, RenderStateKind, RenderStateKindUnpinned,
};
use frender_ssr::SsrElement;

pub trait AnyRenderState<Renderer: ?Sized>: RenderState<Renderer> + Unpin {
    fn upcast_mut_dyn_any(&mut self) -> &mut (dyn 'static + Any);
}

impl<S, Renderer: ?Sized> AnyRenderState<Renderer> for S
where
    S: RenderState<Renderer> + Unpin + 'static,
{
    fn upcast_mut_dyn_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct PinBoxDynRenderState<Renderer: ?Sized> {
    render_state: Pin<Box<dyn 'static + AnyRenderState<Renderer>>>,
}

impl<Renderer: ?Sized> PinBoxDynRenderState<Renderer> {
    // fn as_pin_mut_render_state(&mut self) -> Pin<&mut dyn RenderState<Renderer>> {
    //     let render_state = self.render_state.as_mut();
    //     // render_state.upcast_pin_mut_dyn_render_state()
    // }

    fn pin_project(self: std::pin::Pin<&mut Self>) -> Pin<&mut dyn AnyRenderState<Renderer>> {
        // self.get_mut().as_pin_mut_render_state()
        self.get_mut().render_state.as_mut()
    }
}

impl<Renderer: ?Sized> Unpin for PinBoxDynRenderState<Renderer> {}

impl<R: ?Sized> RenderState<R> for PinBoxDynRenderState<R> {
    fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
        self.pin_project().unmount(renderer)
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        self.pin_project().state_unmount()
    }

    fn poll_render(
        self: std::pin::Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        self.pin_project().poll_render(renderer, cx)
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_html::dom::render::RenderWithContext,
    {
        self.render_state.check_and_move_cursor(render_context)
    }
}

pub struct CsrRenderContext<'a, Ctx: ?Sized + HtmlRenderContext, S: ?Sized = dyn Any> {
    render_context: &'a mut Ctx,
    render_state: &'a mut S,
    force_reposition: bool,
}

impl<'a, Ctx: ?Sized + HtmlRenderContext> CsrRenderContext<'a, Ctx> {
    pub fn render<E: Element>(self, element: E) -> Rendered<'a, E::RenderStateKind>
    where
        E::RenderStateKind: 'static,
        // <E::RenderStateKind as RenderStateKindUnpinned>::UnpinnedRenderState<Ctx::Renderer>:
        //     'static,
    {
        let render_state = self
            .render_state
            .downcast_mut::<<E::RenderStateKind as RenderStateKindUnpinned>::UnpinnedRenderState<Ctx::Renderer>>()
            .expect("Element State type mismatch");

        element.unpinned_render_update_maybe_reposition(
            self.render_context,
            render_state,
            self.force_reposition,
        );
        Rendered(PhantomData, PhantomData)
    }
}

/// A marker returned by [`CsrRenderContext::render`].
///
/// This struct is `'static` if and only if `'a: 'static`.
///
/// ```
/// # use frender::{TempStr, elements::render_with::Rendered};
/// fn test<'a>(a: Rendered<'static, TempStr<&'a str>>) -> impl 'static + Sized {
///     a
/// }
/// ```
///
/// ```compile_fail
/// # use frender::{TempStr, elements::render_with::Rendered};
/// fn test<'a>(a: Rendered<'a, ()>) -> impl 'static + Sized {
///     a
/// }
/// ```
pub struct Rendered<'a, S>(PhantomData<&'a mut ()>, PhantomData<S>);

#[cfg(not(feature = "nightly"))]
impl<S> Rendered<'_, S> {
    fn type_check(self, _: PhantomData<S>) {}
}

pub struct RenderWith<F>(pub F);

// TODO: implement ssr with csr
impl<F> SsrElement for RenderWith<F> {
    type HtmlChildren = async_str_iter::empty::Empty;

    fn into_html_children(self) -> Self::HtmlChildren {
        async_str_iter::empty::Empty
    }
}

pub trait DefaultAnyRenderState<Renderer: ?Sized>:
    'static + Default + AnyRenderState<Renderer>
{
}

impl<Renderer: ?Sized, S> DefaultAnyRenderState<Renderer> for S where
    S: 'static + Default + AnyRenderState<Renderer>
{
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
    for<'r> FnOnce(CsrRenderContext<'r, Ctx>) -> Rendered<'r, Self::OutputRenderStateKind>
{
    type OutputRenderStateKind: RenderStateKind + 'static;
}

impl<F, Ctx: ?Sized + HtmlRenderContext, K> FnOnceRenderWithContext<Ctx> for F
where
    F: for<'r> FnOnce(CsrRenderContext<'r, Ctx>) -> Rendered<'r, K>,
    K: RenderStateKind + 'static,
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
mod not_nightly_impl {
    use frender_html::RenderStateKindPinned;

    use super::*;

    enum Never {}
    pub struct Kind(Never);

    impl RenderStateKindPinned for Kind {
        // TODO: State should be statically typed and stacked allocated without Pin<Box<dyn __>> with [impl Trait in type aliases](https://github.com/rust-lang/rust/issues/63063)
        type RenderState<R: frender_html::RenderHtml + ?Sized> = Option<PinBoxDynRenderState<R>>;
    }
    impl RenderStateKindUnpinned for Kind {
        type UnpinnedRenderState<R: RenderHtml + ?Sized> = Option<PinBoxDynRenderState<R>>;
    }

    impl<F: IntoFnOnceRenderWithContext> Element for RenderWith<F> {
        type RenderStateKind = Kind;

        fn render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: Pin<&mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
            force_reposition: bool,
        ) {
            let state = render_state.get_mut();

            let phantom_state = PhantomData;

            fn default_pin_box_dyn_render_state_with_phantom_hint<
                Renderer: ?Sized + RenderHtml,
                K: RenderStateKindUnpinned + 'static,
            >(
                _: PhantomData<K>,
            ) -> Pin<Box<dyn 'static + AnyRenderState<Renderer>>> {
                Box::pin(<K::UnpinnedRenderState<Renderer> as Default>::default())
            }

            let state =
                state.get_or_insert_with(|| PinBoxDynRenderState {
                    render_state: default_pin_box_dyn_render_state_with_phantom_hint::<
                        Ctx::Renderer,
                        _,
                    >(phantom_state),
                });

            let render_state = state.render_state.as_mut().get_mut();
            let render_state = render_state.upcast_mut_dyn_any();

            let f = self.0.into_fn_once_render_with_context::<Ctx>();
            let rendered: Rendered<_> = f(CsrRenderContext {
                render_context,
                render_state,
                force_reposition,
            });
            rendered.type_check(phantom_state);
        }

        impl_unpinned_render_for_unpin! {}
    }
}
