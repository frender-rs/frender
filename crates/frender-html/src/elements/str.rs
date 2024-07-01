use std::borrow::{Borrow, Cow};
use std::ops::Deref;

// use wasm_bindgen::{JsCast, JsValue};

use frender_dom::render::{RenderContext, RenderWithContext};

use crate::dom::render::RenderTextFrom;
use crate::{dom::behaviors::Node, RenderHtml};

use crate::{Element, HtmlRenderContext, RenderState, RenderStateOfContext};

/// `Text` node with a field recording whether it is unmounted.
pub struct TextNode<Text> {
    pub node: Text,
    pub unmounted: bool,
}

impl<Text> TextNode<Text> {
    pub fn readd_self<R: ?Sized + RenderWithContext>(&mut self, render_context: &mut R::RenderContext<'_>, force_reposition: bool)
    where
        Text: Node<R>,
    {
        self.node.readd_self(render_context, force_reposition || self.unmounted);
    }

    pub fn unmount<R: ?Sized>(&mut self, renderer: &mut R)
    where
        Text: Node<R>,
    {
        self.unmounted = true;
        self.node.remove_self(renderer);
    }

    pub fn mount<Ctx: ?Sized + RenderContext>(render_context: &mut Ctx, mut node: Text) -> Self
    where
        Text: Node<Ctx::Renderer>,
    {
        render_context.map_mut_render_context(|render_context| node.readd_self(render_context, true));
        Self { node, unmounted: false }
    }

    fn mount_from<Ctx: ?Sized + RenderContext, V: ?Sized>(render_context: &mut Ctx, v: &V) -> Self
    where
        Ctx::Renderer: RenderTextFrom<Text, V>,
        Text: Node<Ctx::Renderer>,
    {
        let node = render_context.renderer_mut().render_text_from(v);
        Self::mount(render_context, node)
    }
}

pub struct State<Cache, Text> {
    text_node: TextNode<Text>,
    cache: Cache,
}

trait RenderingStr: Deref<Target = str> {
    type Cache;

    fn create_cache(value: Self) -> Self::Cache;
    fn not_match_cache(this: &Self, cache: &Self::Cache) -> bool;
    fn update_cache(cache: &mut Self::Cache, value: Self);
}

frender_common::impl_many!(
    impl<__> RenderingStr
        for each_of![
            //
            &'static str,
            String,
            Cow<'static, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type Cache = Self;

        #[inline]
        fn create_cache(value: Self) -> Self::Cache {
            value
        }

        #[inline]
        fn not_match_cache(this: &Self, cache: &Self::Cache) -> bool {
            cache != this
        }

        #[inline]
        fn update_cache(cache: &mut Self::Cache, value: Self) {
            *cache = value
        }
    }
);

#[cfg(remove)]
mod js {
    use js_sys::JsString;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type Text;

        #[wasm_bindgen(method, setter)]
        pub fn set_data(this: &Text, val: JsString);

        #[wasm_bindgen(js_name = Document)]
        pub type Document;

        #[wasm_bindgen(method, structural, js_class = "Document", js_name = createTextNode)]
        pub fn create_text_node(this: &Document, data: JsString) -> web_sys::Text;

        /// Calls `String(value)`
        #[wasm_bindgen(js_name = String)]
        pub fn js_string(value: JsValue) -> JsString;
    }
}

impl<Cache, Text> State<Cache, Text> {
    fn update_with_str_maybe_reposition<Ctx: ?Sized + RenderContext, S: Borrow<V>, V: ?Sized>(
        &mut self,
        data: S,
        render_context: &mut Ctx,
        force_reposition: bool,
        not_match_cache: impl FnOnce(&S, &Cache) -> bool,
        update_cache: impl FnOnce(&mut Cache, S),
    ) where
        Ctx::Renderer: RenderHtml<Text = Text> + RenderTextFrom<Text, V>,
        Text: Node<Ctx::Renderer>,
    {
        if not_match_cache(&data, &self.cache) {
            render_context.renderer_mut().update_text_from(&mut self.text_node.node, data.borrow());

            update_cache(&mut self.cache, data);
        }

        render_context.map_mut_render_context(|render_context| self.text_node.readd_self(render_context, force_reposition))
    }

    pub fn initialize_with_str<Ctx: ?Sized + RenderContext, S: Borrow<V>, V: ?Sized>(data: S, render_context: &mut Ctx, create_cache: impl FnOnce(S) -> Cache) -> Self
    where
        Ctx::Renderer: RenderHtml<Text = Text> + RenderTextFrom<Text, V>,
        Text: Node<Ctx::Renderer>,
    {
        State {
            text_node: TextNode::mount_from(render_context, data.borrow()),
            cache: create_cache(data),
        }
    }

    #[cfg(remove)]
    /// The js value returned by `to_js` will be called with `String(value)`
    /// and then set as data of `Text` node.
    pub(crate) fn update_with_js_value_maybe_reposition(&mut self, data: Cache, renderer: &mut R, to_js: impl FnOnce(&Cache) -> JsValue, force_reposition: bool)
    where
        Cache: PartialEq<Cache>,
    {
        self.update_with_js_string_maybe_reposition(data, renderer, move |v| js::js_string(to_js(v)), force_reposition)
    }

    #[cfg(remove)]
    /// The js value returned by `to_js` will be called with `String(value)`
    /// and then set as data of `Text` node.
    #[inline]
    pub fn initialize_with_js_value(data: Cache, dom_ctx: &mut CsrContext, to_js: impl FnOnce(&Cache) -> JsValue) -> Self
    where
        Cache: PartialEq<Cache>,
    {
        Self::initialize_with_js_string(data, dom_ctx, move |v| js::js_string(to_js(v)))
    }

    #[cfg(remove)]
    pub(crate) fn update_with_js_string_maybe_reposition(&mut self, data: Cache, dom_ctx: &mut R, to_js: impl FnOnce(&Cache) -> JsString, force_reposition: bool)
    where
        Cache: PartialEq<Cache>,
    {
        if self.cache != data {
            let s = to_js(&data);
            self.node.unchecked_ref::<js::Text>().set_data(s);
            self.cache = data;
        }

        self.add_self_to_dom(dom_ctx, force_reposition)
    }

    #[cfg(remove)]
    pub fn initialize_with_js_string(data: Cache, dom_ctx: &mut R, to_js: impl FnOnce(&Cache) -> JsString) -> Self {
        let s = to_js(&data);
        let text = dom_ctx.document.unchecked_ref::<js::Document>().create_text_node(s);
        dom_ctx.next_node_position.add_node(Cow::Owned(text.clone().into()));
        Self {
            node: text,
            cache: data,
            unmounted: false,
        }
    }
}

impl<Cache, Text> Unpin for State<Cache, Text> {}

impl<Cache, Text: Node<R>, R: ?Sized> RenderState<R> for State<Cache, Text> {
    fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
        let this = self.get_mut();
        this.text_node.unmount(renderer);
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {}

    fn poll_render(self: std::pin::Pin<&mut Self>, _: &mut R, _: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        std::task::Poll::Ready(())
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        match &self.text_node {
            TextNode { node, unmounted: false } => node.check_and_move_cursor_after_self(render_context),
            _ => {}
        }
    }
}

pub struct Kind<Cache: 'static>(super::Kind<Cache>);

impl<Cache: 'static> crate::RenderStateKindPinned for Kind<Cache> {
    type RenderState<R: RenderHtml + ?Sized> = Option<State<Cache, R::Text>>;
}
impl<Cache: 'static> crate::RenderStateKindUnpinned for Kind<Cache> {
    type UnpinnedRenderState<R: RenderHtml + ?Sized> = Option<State<Cache, R::Text>>;
}

frender_common::impl_many!(
    impl<__> Element
        for each_of![
            //
            Cow<'static, str>,
            &'static str,
            String,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type RenderStateKind = Kind<Self>;

        fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: std::pin::Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>, force_reposition: bool) {
            match render_state.get_mut() {
                Some(render_state) => render_state.update_with_str_maybe_reposition::<_, _, str>(self, render_context, force_reposition, RenderingStr::not_match_cache, RenderingStr::update_cache),
                render_state @ None => *render_state = Some(State::initialize_with_str::<_, _, str>(self, render_context, RenderingStr::create_cache)),
            }
        }

        crate::impl_unpinned_render_for_unpin! {}
    }
);

impl<S: std::borrow::Borrow<str> + frender_common::IntoStaticStr> Element for frender_common::TempStr<S> {
    type RenderStateKind = Kind<<S as frender_common::IntoStaticStr>::IntoStaticStr>;

    fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: std::pin::Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>, force_reposition: bool) {
        match render_state.get_mut() {
            Some(render_state) => render_state.update_with_str_maybe_reposition::<_, _, str>(
                self.0,
                render_context,
                force_reposition,
                |s, cache| *s.borrow() != *cache.borrow(),
                |cache, s| frender_common::IntoStaticStr::update_into_static_str(s, cache),
            ),
            render_state @ None => *render_state = Some(State::initialize_with_str::<_, _, str>(self.0, render_context, frender_common::IntoStaticStr::into_static_str)),
        }
    }

    crate::impl_unpinned_render_for_unpin! {}
}

pub(crate) fn render_update_maybe_reposition<V: ?Sized, S: Borrow<V>, Cache, Ctx: ?Sized + HtmlRenderContext>(
    data: S,
    render_context: &mut Ctx,
    render_state: std::pin::Pin<&mut Option<State<Cache, <Ctx::Renderer as RenderHtml>::Text>>>,
    force_reposition: bool,
    not_match_cache: impl FnOnce(&S, &Cache) -> bool,
    update_cache: impl FnOnce(&mut Cache, S),
    create_cache: impl FnOnce(S) -> Cache,
) where
    Ctx::Renderer: RenderTextFrom<<Ctx::Renderer as RenderHtml>::Text, V>,
{
    match render_state.get_mut() {
        Some(render_state) => render_state.update_with_str_maybe_reposition::<Ctx, S, V>(data, render_context, force_reposition, not_match_cache, update_cache),
        render_state @ None => *render_state = Some(State::initialize_with_str::<Ctx, S, V>(data, render_context, create_cache)),
    }
}
