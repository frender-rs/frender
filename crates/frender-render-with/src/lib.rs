use std::{any::Any, marker::PhantomData, pin::Pin};

use frender_html::{impl_unpinned_render_for_unpin, Element, RenderHtml, RenderState};
use frender_ssr::SsrElement;

pub trait AnyRenderState<PEH: ?Sized, Renderer: ?Sized>:
    RenderState<PEH, Renderer> + Unpin
{
    fn upcast_mut_dyn_any(&mut self) -> &mut (dyn 'static + Any);
}

impl<S, PEH: ?Sized, Renderer: ?Sized> AnyRenderState<PEH, Renderer> for S
where
    S: RenderState<PEH, Renderer> + Unpin + 'static,
{
    fn upcast_mut_dyn_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct PinBoxDynRenderState<PEH: ?Sized, Renderer: ?Sized> {
    render_state: Pin<Box<dyn 'static + AnyRenderState<PEH, Renderer>>>,
}

impl<PEH: ?Sized, Renderer: ?Sized> PinBoxDynRenderState<PEH, Renderer> {
    // fn as_pin_mut_render_state(&mut self) -> Pin<&mut dyn RenderState<PEH, Renderer>> {
    //     let render_state = self.render_state.as_mut();
    //     // render_state.upcast_pin_mut_dyn_render_state()
    // }

    fn pin_project(self: std::pin::Pin<&mut Self>) -> Pin<&mut dyn AnyRenderState<PEH, Renderer>> {
        // self.get_mut().as_pin_mut_render_state()
        self.get_mut().render_state.as_mut()
    }
}

impl<PEH: ?Sized, Renderer: ?Sized> Unpin for PinBoxDynRenderState<PEH, Renderer> {}

impl<PEH: ?Sized, R: ?Sized> RenderState<PEH, R> for PinBoxDynRenderState<PEH, R> {
    fn unmount(self: std::pin::Pin<&mut Self>, parent_elements_handle: &mut PEH, renderer: &mut R) {
        self.pin_project().unmount(parent_elements_handle, renderer)
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        self.pin_project().state_unmount()
    }

    fn poll_render(
        self: std::pin::Pin<&mut Self>,
        parent_elements_handle: &mut PEH,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        self.pin_project()
            .poll_render(parent_elements_handle, renderer, cx)
    }
}

pub struct CsrRenderContext<'a, PEH: ?Sized, Renderer: ?Sized, S: ?Sized = dyn Any> {
    parent_elements_handle: &'a mut PEH,
    renderer: &'a mut Renderer,
    render_state: &'a mut S,
    force_reposition: bool,
}

impl<'a, PEH: ?Sized, Renderer: ?Sized + RenderHtml> CsrRenderContext<'a, PEH, Renderer> {
    pub fn render<E: Element>(
        self,
        element: E,
    ) -> Rendered<'a, <E as Element>::UnpinnedRenderState<PEH, Renderer>>
    where
        <E as Element>::UnpinnedRenderState<PEH, Renderer>: 'static,
    {
        let render_state = self
            .render_state
            .downcast_mut::<<E as Element>::UnpinnedRenderState<PEH, Renderer>>()
            .expect("Element State type mismatch");

        element.unpinned_render_update_maybe_reposition(
            self.parent_elements_handle,
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
pub struct Rendered<'a, S: 'static>(PhantomData<&'a mut ()>, PhantomData<S>);

pub struct RenderWith<F>(pub F);

impl<F> SsrElement for RenderWith<F> {
    type HtmlChildren = async_str_iter::empty::Empty;

    fn into_html_children(self) -> Self::HtmlChildren {
        async_str_iter::empty::Empty
    }
}

pub trait DefaultAnyRenderState<PEH: ?Sized, Renderer: ?Sized>:
    'static + Default + AnyRenderState<PEH, Renderer>
{
}

impl<PEH: ?Sized, Renderer: ?Sized, S> DefaultAnyRenderState<PEH, Renderer> for S where
    S: 'static + Default + AnyRenderState<PEH, Renderer>
{
}

/// This might be just
/// `for<'r, PEH: ?Sized, Renderer: ?Sized + RenderHtml> FnMut(CsrRenderContext<'r, PEH, Renderer, dyn Any>) -> Rendered<'r, impl Any>`
/// in the future.
pub trait FnOnceRenderWithContext {
    fn call_once_render_with_context<'r, PEH: ?Sized, Renderer: ?Sized + RenderHtml>(
        self,
        ctx: CsrRenderContext<'r, PEH, Renderer>,
    ) -> Rendered<'r, impl DefaultAnyRenderState<PEH, Renderer>>;
}

impl<F: FnOnceRenderWithContext> Element for RenderWith<F> {
    // TODO: State should be statically typed and stacked allocated without Pin<Box<dyn __>> with [impl Trait in type aliases](https://github.com/rust-lang/rust/issues/63063)
    type RenderState<PEH: ?Sized, R: frender_html::RenderHtml + ?Sized> =
        Option<PinBoxDynRenderState<PEH, R>>;

    fn render_update_maybe_reposition<PEH: ?Sized, Renderer: frender_html::RenderHtml + ?Sized>(
        //
        self,
        parent_elements_handle: &mut PEH,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
        force_reposition: bool,
    ) {
        let state = render_state.get_mut();
        // state.get_or_insert_with(
        //     DefaultDynRenderState::<PEH, Renderer>::default_pin_box_dyn_render_state,
        // );
        let mut phantom_state = PhantomData;

        fn default_pin_box_dyn_render_state_with_phantom_hint<
            PEH: ?Sized,
            Renderer: ?Sized,
            T: DefaultAnyRenderState<PEH, Renderer>,
        >(
            _: PhantomData<T>,
        ) -> Pin<Box<dyn 'static + AnyRenderState<PEH, Renderer>>> {
            Box::pin(T::default())
        }

        let state = state.get_or_insert_with(|| PinBoxDynRenderState {
            render_state: default_pin_box_dyn_render_state_with_phantom_hint::<PEH, Renderer, _>(
                phantom_state,
            ),
        });

        let render_state = state.render_state.as_mut().get_mut();
        let render_state = render_state.upcast_mut_dyn_any();

        phantom_state = self
            .0
            .call_once_render_with_context(CsrRenderContext {
                parent_elements_handle,
                renderer,
                render_state,
                force_reposition,
            })
            .1;

        _ = phantom_state;
    }

    impl_unpinned_render_for_unpin! {}
}
