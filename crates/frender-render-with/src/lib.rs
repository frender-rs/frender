#![cfg_attr(feature = "nightly", feature(impl_trait_in_assoc_type))]

use std::{any::Any, marker::PhantomData, pin::Pin};

use frender_html::{impl_unpinned_render_for_unpin, Element, RenderHtml, RenderState};
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
}

pub struct CsrRenderContext<'a, Renderer: ?Sized, S: ?Sized = dyn Any> {
    renderer: &'a mut Renderer,
    render_state: &'a mut S,
    force_reposition: bool,
}

impl<'a, Renderer: ?Sized + RenderHtml> CsrRenderContext<'a, Renderer> {
    pub fn render<E: Element>(
        self,
        element: E,
    ) -> Rendered<'a, <E as Element>::UnpinnedRenderState<Renderer>>
    where
        <E as Element>::UnpinnedRenderState<Renderer>: 'static,
    {
        let render_state = self
            .render_state
            .downcast_mut::<<E as Element>::UnpinnedRenderState<Renderer>>()
            .expect("Element State type mismatch");

        element.unpinned_render_update_maybe_reposition(
            self.renderer,
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
    fn into_fn_once_render_with_context<Renderer: ?Sized + RenderHtml>(
        self,
    ) -> impl FnOnceRenderWithContext<Renderer>;
    // TODO: `impl DefaultAnyRenderState<Renderer>` should not capture `'r`. See https://github.com/rust-lang/rust/issues/117210#issuecomment-2180030657
}

// trait Helper<Renderer: ?Sized + RenderHtml>: FnOnce(T) -> Self::FnOnceRenderWithContext {
//     type FnOnceRenderWithContext: FnOnceRenderWithContext<Renderer>;
// }

// TODO: RenderWith(Test::default())

pub trait FnOnceRenderWithContext<Renderer: ?Sized + RenderHtml>:
    for<'r> FnOnce(CsrRenderContext<'r, Renderer>) -> Rendered<'r, Self::OutputRenderedState>
{
    type OutputRenderedState: DefaultAnyRenderState<Renderer>;
}

impl<F, Renderer: ?Sized + RenderHtml, S> FnOnceRenderWithContext<Renderer> for F
where
    S: DefaultAnyRenderState<Renderer>,
    F: for<'r> FnOnce(CsrRenderContext<'r, Renderer>) -> Rendered<'r, S>,
{
    type OutputRenderedState = S;
}

pub trait RenderContext<Renderer: ?Sized + RenderHtml> {
    fn render<E: Element>(self, element: E) -> Rendered<'static, E::UnpinnedRenderState<Renderer>>;
}

#[cfg(feature = "nightly")]
mod nightly_impl {
    use super::*;

    pub trait NamedIntoFnOnceRenderWithContext: IntoFnOnceRenderWithContext {
        type RenderedRenderState<R: frender_html::RenderHtml + ?Sized>: DefaultAnyRenderState<R>;

        fn _named_into_fn_once_render_with_context_helper<R: frender_html::RenderHtml + ?Sized>(
            self,
        ) -> PhantomData<Self::RenderedRenderState<R>>;
    }

    impl<F: IntoFnOnceRenderWithContext> NamedIntoFnOnceRenderWithContext for F {
        type RenderedRenderState<R: frender_html::RenderHtml + ?Sized> =
            impl DefaultAnyRenderState<R>;

        fn _named_into_fn_once_render_with_context_helper<R: frender_html::RenderHtml + ?Sized>(
            self,
        ) -> PhantomData<Self::RenderedRenderState<R>> {
            fn test<Renderer: ?Sized + RenderHtml, S>(
                _: impl FnOnceRenderWithContext<Renderer, OutputRenderedState = S>,
            ) -> PhantomData<S> {
                PhantomData
            }
            test(self.into_fn_once_render_with_context())
        }
    }

    impl<F: IntoFnOnceRenderWithContext> Element for RenderWith<F> {
        type RenderState<R: frender_html::RenderHtml + ?Sized> =
            <F as NamedIntoFnOnceRenderWithContext>::RenderedRenderState<R>;

        fn render_update_maybe_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
            //
            self,
            renderer: &mut Renderer,
            render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
            force_reposition: bool,
        ) {
            let render_state = render_state.get_mut();

            let f = self.0.into_fn_once_render_with_context();
            f(CsrRenderContext {
                renderer,
                render_state: render_state as &mut (dyn 'static + Any),
                force_reposition,
            });
        }

        impl_unpinned_render_for_unpin! {}
    }
}

#[cfg(not(feature = "nightly"))]
impl<F: IntoFnOnceRenderWithContext> Element for RenderWith<F> {
    // TODO: State should be statically typed and stacked allocated without Pin<Box<dyn __>> with [impl Trait in type aliases](https://github.com/rust-lang/rust/issues/63063)
    type RenderState<R: frender_html::RenderHtml + ?Sized> = Option<PinBoxDynRenderState<R>>;

    fn render_update_maybe_reposition<Renderer: frender_html::RenderHtml + ?Sized>(
        //
        self,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
        force_reposition: bool,
    ) {
        let state = render_state.get_mut();

        let phantom_state = PhantomData;

        fn default_pin_box_dyn_render_state_with_phantom_hint<
            Renderer: ?Sized,
            T: DefaultAnyRenderState<Renderer>,
        >(
            _: PhantomData<T>,
        ) -> Pin<Box<dyn 'static + AnyRenderState<Renderer>>> {
            Box::pin(T::default())
        }

        let state = state.get_or_insert_with(|| PinBoxDynRenderState {
            render_state: default_pin_box_dyn_render_state_with_phantom_hint::<Renderer, _>(
                phantom_state,
            ),
        });

        let render_state = state.render_state.as_mut().get_mut();
        let render_state = render_state.upcast_mut_dyn_any();

        let f = self.0.into_fn_once_render_with_context();
        f(CsrRenderContext {
            renderer,
            render_state,
            force_reposition,
        })
        .type_check(phantom_state);
    }

    impl_unpinned_render_for_unpin! {}
}
