use std::borrow::Cow;
use std::ops::Deref;

use crate::{
    pin_mut_maybe_uninit::PinMutMaybeUninit,
    renderer::{self, RemoveNode},
    Element, RenderHtml, RenderState,
};
#[cfg(feature = "StaticText")]
use frender_core::{StaticStr, StaticText};

use wasm_bindgen::{JsCast, JsValue};

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

#[cfg(feature = "StaticText")]
impl<S: StaticStr> RenderingStr for StaticText<S> {
    type Cache = S;

    #[inline]
    fn create_cache(value: Self) -> Self::Cache {
        value.0
    }

    #[inline]
    fn not_match_cache(this: &Self, cache: &Self::Cache) -> bool {
        **cache != **this
    }

    #[inline]
    fn update_cache(cache: &mut Self::Cache, value: Self) {
        *cache = value.0
    }
}

impl RenderingStr for String {
    type Cache = String;

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

/// No cache
impl RenderingStr for &str {
    type Cache = ();

    #[inline]
    fn create_cache(_: Self) -> Self::Cache {}

    #[inline]
    fn not_match_cache(_: &Self, _: &Self::Cache) -> bool {
        true
    }

    #[inline]
    fn update_cache(_: &mut Self::Cache, _: Self) {}
}

/// Only cache when self is [`Cow::Owned`].
impl RenderingStr for Cow<'_, str> {
    type Cache = Option<String>;

    #[inline]
    fn create_cache(value: Self) -> Self::Cache {
        match value {
            Cow::Borrowed(_) => None,
            Cow::Owned(s) => Some(s),
        }
    }

    #[inline]
    fn not_match_cache(this: &Self, cache: &Self::Cache) -> bool {
        cache.as_deref() != Some(this)
    }

    #[inline]
    fn update_cache(cache: &mut Self::Cache, value: Self) {
        *cache = match value {
            Cow::Borrowed(_) => None,
            Cow::Owned(s) => Some(s),
        }
    }
}

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
    fn add_self_to_dom<R: RenderHtml<Text = Text>>(
        &mut self,
        renderer: &mut R,
        force_reposition: bool,
    ) {
        renderer.readd_node(&mut self.node, force_reposition || self.unmounted);
    }

    fn update_with_str_maybe_reposition<R: RenderHtml<Text = Text>, S>(
        &mut self,
        data: S,
        renderer: &mut R,
        force_reposition: bool,
    ) where
        S: RenderingStr<Cache = Cache>,
    {
        if S::not_match_cache(&data, &self.cache) {
            renderer.update_text_from(&mut self.node, &*data);

            S::update_cache(&mut self.cache, data);
        }

        self.add_self_to_dom(renderer, force_reposition)
    }

    pub fn initialize_with_str<R: RenderHtml<Text = Text>, S>(data: S, renderer: &mut R) -> Self
    where
        S: RenderingStr<Cache = Cache>,
    {
        let text = renderer.render_text_from(&*data);
        State {
            node: text,
            cache: S::create_cache(data),
            unmounted: false,
        }
    }

    #[cfg(remove)]
    /// The js value returned by `to_js` will be called with `String(value)`
    /// and then set as data of `Text` node.
    pub(crate) fn update_with_js_value_maybe_reposition(
        &mut self,
        data: Cache,
        renderer: &mut R,
        to_js: impl FnOnce(&Cache) -> JsValue,
        force_reposition: bool,
    ) where
        Cache: PartialEq<Cache>,
    {
        self.update_with_js_string_maybe_reposition(
            data,
            renderer,
            move |v| js::js_string(to_js(v)),
            force_reposition,
        )
    }

    #[cfg(remove)]
    /// The js value returned by `to_js` will be called with `String(value)`
    /// and then set as data of `Text` node.
    #[inline]
    pub fn initialize_with_js_value(
        data: Cache,
        dom_ctx: &mut CsrContext,
        to_js: impl FnOnce(&Cache) -> JsValue,
    ) -> Self
    where
        Cache: PartialEq<Cache>,
    {
        Self::initialize_with_js_string(data, dom_ctx, move |v| js::js_string(to_js(v)))
    }

    #[cfg(remove)]
    pub(crate) fn update_with_js_string_maybe_reposition(
        &mut self,
        data: Cache,
        dom_ctx: &mut R,
        to_js: impl FnOnce(&Cache) -> JsString,
        force_reposition: bool,
    ) where
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
    pub fn initialize_with_js_string(
        data: Cache,
        dom_ctx: &mut R,
        to_js: impl FnOnce(&Cache) -> JsString,
    ) -> Self {
        let s = to_js(&data);
        let text = dom_ctx
            .document
            .unchecked_ref::<js::Document>()
            .create_text_node(s);
        dom_ctx
            .next_node_position
            .add_node(Cow::Owned(text.clone().into()));
        Self {
            node: text,
            cache: data,
            unmounted: false,
        }
    }
}

impl<Cache, Text> Unpin for State<Cache, Text> {}

impl<Cache, Text, R: RemoveNode<Text>> RenderState<R> for State<Cache, Text> {
    fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
        let this = self.get_mut();
        this.unmounted = true;
        renderer.remove_node(&mut this.node)
    }

    fn state_unmount(mut self: std::pin::Pin<&mut Self>) {}

    fn poll_render(
        self: std::pin::Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        std::task::Poll::Ready(())
    }
}

macro_rules! impl_render_str {
    ($(
        $(@[$($generics:tt)*])?
        $for_ty:ty
    ),* $(,)?) => {$(
        impl $(<$($generics)*>)? Element for $for_ty {
            type RenderState<Renderer: RenderHtml> = Option<State<<Self as RenderingStr>::Cache, Renderer::Text>>;

            #[cfg(feature = "render_into")]
            fn render_into<'s, Renderer: RenderHtml>(
                self,
                renderer: &mut Renderer,
                render_state: PinMutMaybeUninit<'s, Self::RenderState<Renderer>>,
            ) -> std::pin::Pin<&'s mut Self::RenderState<Renderer>> {
                render_state.write(
                    Self::RenderState::<Renderer>::initialize_with_str(self, renderer)
                )
            }

            fn render_update_maybe_reposition<Renderer: RenderHtml>(
                self,
                renderer: &mut Renderer,
                render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
                force_reposition: bool,
            ) {
                match render_state.get_mut() {
                    Some(render_state) => render_state.update_with_str_maybe_reposition(self, renderer, force_reposition),
                    render_state @ None => *render_state = Some(State::initialize_with_str(self, renderer))
                }
            }
        }
    )*};
}

impl_render_str! {
    Cow<'_, str>,
    &str,
    String,
}

#[cfg(feature = "StaticText")]
impl_render_str! {
    @[S: StaticStr] StaticText<S>,
}
