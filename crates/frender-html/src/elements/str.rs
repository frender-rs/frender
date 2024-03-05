use std::borrow::{Borrow, Cow};
use std::ops::Deref;

// use wasm_bindgen::{JsCast, JsValue};

#[cfg(feature = "StaticText")]
use frender_core::{StaticStr, StaticText};

use crate::dom::render::RenderTextFrom;
use crate::{dom::behaviors::Node, RenderHtml};

use crate::{Element, RenderState};

pub struct State<Cache, Text> {
    node: Text,
    cache: Cache,
    unmounted: bool,
}

pub trait RenderingStr: Deref<Target = str> {
    type Cache;

    fn create_cache(value: Self) -> Self::Cache;
    fn not_match_cache(this: &Self, cache: &Self::Cache) -> bool;
    fn update_cache(cache: &mut Self::Cache, value: Self);
}

frender_common::impl_many!(
    impl<__> RenderingStr for each_of![&'static str, String, Cow<'static, str>,] {
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
    fn add_self_to_dom<R: ?Sized>(&mut self, renderer: &mut R, force_reposition: bool)
    where
        Text: Node<R>,
    {
        self.node.readd_self(renderer, force_reposition || self.unmounted);
    }

    fn update_with_str_maybe_reposition<R: RenderHtml<Text = Text> + RenderTextFrom<Text, V> + ?Sized, S: Borrow<V>, V: ?Sized>(
        &mut self,
        data: S,
        renderer: &mut R,
        force_reposition: bool,
        not_match_cache: impl FnOnce(&S, &Cache) -> bool,
        update_cache: impl FnOnce(&mut Cache, S),
    ) where
        Text: Node<R>,
    {
        if not_match_cache(&data, &self.cache) {
            renderer.update_text_from(&mut self.node, data.borrow());

            update_cache(&mut self.cache, data);
        }

        self.add_self_to_dom(renderer, force_reposition)
    }

    pub fn initialize_with_str<R: RenderHtml<Text = Text> + RenderTextFrom<Text, V> + ?Sized, S: Borrow<V>, V: ?Sized>(data: S, renderer: &mut R, create_cache: impl FnOnce(S) -> Cache) -> Self
    where
        Text: Node<R>,
    {
        let mut text = renderer.render_text_from(data.borrow());
        text.readd_self(renderer, true);
        State {
            node: text,
            cache: create_cache(data),
            unmounted: false,
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

impl<Cache, Text: Node<R>, PEH: ?Sized, R: ?Sized> RenderState<PEH, R> for State<Cache, Text> {
    fn unmount(self: std::pin::Pin<&mut Self>, _: &mut PEH, renderer: &mut R) {
        let this = self.get_mut();
        this.unmounted = true;
        this.node.remove_self(renderer);
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {}

    fn poll_render(self: std::pin::Pin<&mut Self>, _: &mut PEH, _: &mut R, _: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        std::task::Poll::Ready(())
    }
}

frender_common::impl_many!(
    impl<__> Element
        for each_of![
            //
            Cow<'static, str>,
            &'static str,
            String,
        ]
    {
        type RenderState<PEH: ?Sized, Renderer: RenderHtml + ?Sized> = Option<State<<Self as RenderingStr>::Cache, Renderer::Text>>;

        #[cfg(feature = "render_into")]
        fn render_into<'s, Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: PinMutMaybeUninit<'s, Self::RenderState<PEH, Renderer>>) -> std::pin::Pin<&'s mut Self::RenderState<PEH, Renderer>> {
            render_state.write(Self::RenderState::<Renderer>::initialize_with_str(self, renderer))
        }

        fn render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
            self,
            _: &mut PEH,
            renderer: &mut Renderer,
            render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
            force_reposition: bool,
        ) {
            match render_state.get_mut() {
                Some(render_state) => render_state.update_with_str_maybe_reposition::<_, _, str>(self, renderer, force_reposition, RenderingStr::not_match_cache, RenderingStr::update_cache),
                render_state @ None => *render_state = Some(State::initialize_with_str::<_, _, str>(self, renderer, RenderingStr::create_cache)),
            }
        }

        crate::impl_unpinned_render_for_unpin! {}
    }
);

impl<S: std::borrow::Borrow<str> + frender_common::IntoStaticStr> Element for frender_common::TempStr<S> {
    type RenderState<PEH: ?Sized, Renderer: RenderHtml + ?Sized> = Option<State<<S as frender_common::IntoStaticStr>::IntoStaticStr, Renderer::Text>>;

    fn render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, _: &mut PEH, renderer: &mut Renderer, render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>, force_reposition: bool) {
        match render_state.get_mut() {
            Some(render_state) => render_state.update_with_str_maybe_reposition::<_, _, str>(
                self.0,
                renderer,
                force_reposition,
                |s, cache| *s.borrow() == *cache.borrow(),
                |cache, s| frender_common::IntoStaticStr::update_into_static_str(s, cache),
            ),
            render_state @ None => *render_state = Some(State::initialize_with_str::<_, _, str>(self.0, renderer, frender_common::IntoStaticStr::into_static_str)),
        }
    }

    crate::impl_unpinned_render_for_unpin! {}
}

pub(crate) fn render_update_maybe_reposition<V: ?Sized, S: Borrow<V>, Cache, Renderer: RenderHtml + RenderTextFrom<Renderer::Text, V> + ?Sized>(
    data: S,
    renderer: &mut Renderer,
    render_state: std::pin::Pin<&mut Option<State<Cache, Renderer::Text>>>,
    force_reposition: bool,
    not_match_cache: impl FnOnce(&S, &Cache) -> bool,
    update_cache: impl FnOnce(&mut Cache, S),
    create_cache: impl FnOnce(S) -> Cache,
) {
    match render_state.get_mut() {
        Some(render_state) => render_state.update_with_str_maybe_reposition::<Renderer, S, V>(data, renderer, force_reposition, not_match_cache, update_cache),
        render_state @ None => *render_state = Some(State::initialize_with_str::<Renderer, S, V>(data, renderer, create_cache)),
    }
}
