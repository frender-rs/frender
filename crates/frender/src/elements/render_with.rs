use std::{any::Any, marker::PhantomData};

use frender_html::{impl_unpinned_render_for_unpin, Element, RenderHtml};
use frender_ssr::SsrElement;

/// This struct is always `'static`.
///
/// ```
/// # use frender::{TempStr, elements::render_with::ElementKind};
/// fn test<'a>(a: TempStr<&'a str>) -> impl 'static + Sized {
///     ElementKind::of_val(&a)
/// }
/// # let s = String::new();
/// # test(TempStr(&s));
///
/// trait IsStatic: 'static {}
/// impl<S: ?Sized + 'static> IsStatic for S {}
///
/// const _: &'static dyn IsStatic = &{
///     let mut s = [0u8; 5];
///     s = *b"hello";
///
///     let s = match std::str::from_utf8(&s) {
///         Ok(v) => v,
///         Err(_) => panic!(),
///     };
///     let element_kind = ElementKind::of_val(&TempStr(s));
///     element_kind
/// };
/// ```
pub struct ElementKind<E: Element + ?Sized>(PhantomData<fn(PhantomData<E>)>);

impl<E: Element + ?Sized> ElementKind<E> {
    pub const fn of() -> Self {
        Self(PhantomData)
    }
    pub const fn of_val(_: &E) -> Self {
        Self::of()
    }
}

pub struct State<F: FnMutRenderWithContext> {
    // state: S,
    f: F,
}

pin_project_lite::pin_project!(
    pub struct CsrRenderContext<'a, PEH: ?Sized, Renderer: ?Sized, S: ?Sized> {
        parent_elements_handle: &'a mut PEH,
        renderer: &'a mut Renderer,
        render_state: &'a mut S,
        force_reposition: bool,
    }
);

impl<'a, PEH: ?Sized, Renderer: ?Sized + RenderHtml> CsrRenderContext<'a, PEH, Renderer, dyn Any> {
    pub fn render<E: Element>(self, element: E) -> Rendered<'a, E>
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
        Rendered(PhantomData, ElementKind::of())
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
pub struct Rendered<'a, E: Element + ?Sized>(PhantomData<&'a mut ()>, ElementKind<E>);

pub struct RenderWith<F>(F);

impl<F> SsrElement for RenderWith<F> {
    type HtmlChildren = async_str_iter::empty::Empty;

    fn into_html_children(self) -> Self::HtmlChildren {
        async_str_iter::empty::Empty
    }
}

/// This might be just
/// `for<'r, PEH: ?Sized, Renderer: ?Sized + RenderHtml> FnMut(CsrRenderContext<'r, PEH, Renderer, dyn Any>) -> Rendered<'r, impl Element>`
/// in the future.
pub trait FnMutRenderWithContext {
    fn call_mut_render_with_context<'r, PEH: ?Sized, Renderer: ?Sized + RenderHtml>(
        &mut self,
        ctx: CsrRenderContext<'r, PEH, Renderer, dyn Any>,
    ) -> Rendered<'r, impl Element>;
}

impl FnMutRenderWithContext for &str {
    fn call_mut_render_with_context<'r, PEH: ?Sized, Renderer: ?Sized + RenderHtml>(
        &mut self,
        ctx: CsrRenderContext<'r, PEH, Renderer, dyn Any>,
    ) -> Rendered<'r, impl Element> {
        use crate::prelude::*;
        use crate::TempStr;
        ctx.render(cs::button.children(TempStr(*self)).on_click(|_: &_| {}))
    }
}

impl<F: FnMutRenderWithContext> Element for RenderWith<F> {
    type RenderState<PEH: ?Sized, R: frender_html::RenderHtml + ?Sized> = ();

    fn render_update_maybe_reposition<PEH: ?Sized, Renderer: frender_html::RenderHtml + ?Sized>(
        //
        self,
        parent_elements_handle: &mut PEH,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
        force_reposition: bool,
    ) {
        todo!()
    }

    impl_unpinned_render_for_unpin! {}
}
