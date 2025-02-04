use std::marker::PhantomData;

use frender_common::{impl_many, TempStr};
use frender_dom::{
    render::{RenderIntoTextKnown, RenderTextFrom},
    string_element::StringElement,
};

use crate::{HtmlRenderContext, RenderHtml};

use super::{StatelessRender, StatelessRenderStateKind, StatelessUiHandleOfKind};

enum Never {}
pub struct Kind<V: 'static + RenderIntoTextKnown>(Never, PhantomData<V>);

impl<V: 'static + RenderIntoTextKnown> StatelessRenderStateKind for Kind<V> {
    type UiHandle<R: ?Sized + RenderHtml> = <V::RenderTextFromSelf<R> as RenderTextFrom<V>>::Text;
}

// known text ('static)
impl_many!(
    impl<__> StatelessRender
        for each_of![
            // string
            &'static str,
            // - known special text elements
            StringElement,
            // - known scalar types
            i8,
            u8,
            i16,
            u16,
            i32,
            u32,
            i64,
            u64,
            i128,
            u128,
            isize,
            usize,
            f32,
            f64,
            char,
        ]
    {
        type StatelessRenderStateKind = Kind<Self>;

        fn stateless_render_init<Ctx: ?Sized + HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
        ) -> StatelessUiHandleOfKind<Ctx::Renderer, Self::StatelessRenderStateKind> {
            use frender_dom::render::RenderContextRenderTextFrom as _;
            render_context.map_mut_render_context(|render_context| render_context.render_text_from(self))
        }

        fn stateless_render_update<R: ?Sized + RenderHtml>(
            //
            self,
            renderer: &mut R,
            ui_handle: &mut StatelessUiHandleOfKind<R, Self::StatelessRenderStateKind>,
        ) {
            renderer.update_text_from(ui_handle, self);
        }
    }
);

impl StatelessRender for &StringElement {
    type StatelessRenderStateKind = Kind<&'static StringElement>;
    fn stateless_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> StatelessUiHandleOfKind<Ctx::Renderer, Self::StatelessRenderStateKind> {
    }
    fn stateless_render_update<R: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut R,
        ui_handle: &mut StatelessUiHandleOfKind<R, Self::StatelessRenderStateKind>,
    ) {
        renderer.update_text_from(ui_handle, &self);
    }
}

impl StatelessRender for TempStr<&str> {
    type StatelessRenderStateKind = Kind<TempStr<&'static str>>;
    fn stateless_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> StatelessUiHandleOfKind<Ctx::Renderer, Self::StatelessRenderStateKind> {
    }
    fn stateless_render_update<R: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut R,
        ui_handle: &mut StatelessUiHandleOfKind<R, Self::StatelessRenderStateKind>,
    ) {
        renderer.update_text_from(ui_handle, &self);
    }
}
