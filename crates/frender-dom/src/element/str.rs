use std::borrow::Cow;
use std::ops::Deref;

use frender_core::{RenderState, StaticStr, StaticText, UpdateRenderState};
use js_sys::JsString;
use wasm_bindgen::{JsCast, JsValue};

use crate::Dom;

use crate::utils::option::map_or_insert_with_ctx;

pub struct StateInner<Cache> {
    node: web_sys::Text,
    cache: Cache,
}

pub struct State<Cache> {
    inner: Option<StateInner<Cache>>,
}

pub trait RenderingStr: Deref<Target = str> {
    type Cache;

    fn create_cache(value: Self) -> Self::Cache;
    fn not_match_cache(this: &Self, cache: &Self::Cache) -> bool;
    fn update_cache(cache: &mut Self::Cache, value: Self);
}

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
        cache.as_deref() != Some(&this)
    }

    #[inline]
    fn update_cache(cache: &mut Self::Cache, value: Self) {
        *cache = match value {
            Cow::Borrowed(_) => None,
            Cow::Owned(s) => Some(s),
        }
    }
}

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

impl<Cache> State<Cache> {
    pub fn update_with_str<S>(&mut self, data: S, dom_ctx: &mut Dom)
    where
        S: RenderingStr<Cache = Cache>,
    {
        map_or_insert_with_ctx(
            &mut self.inner,
            (dom_ctx, data),
            |this, (dom_ctx, data)| {
                if S::not_match_cache(&data, &this.cache) {
                    this.node.set_data(&data);
                    S::update_cache(&mut this.cache, data);
                }
                dom_ctx
                    .next_node_position
                    .set_as_insert_after(this.node.clone().into());
            },
            |(dom_ctx, data)| {
                let text = dom_ctx.document.create_text_node(&data);
                dom_ctx.next_node_position.add_node(text.clone().into());
                StateInner {
                    node: text,
                    cache: S::create_cache(data),
                }
            },
        );
    }

    /// The js value returned by `to_js` will be called with `String(value)`
    /// and then set as data of `Text` node.
    #[inline]
    pub fn update_with_js_value(
        &mut self,
        data: Cache,
        dom_ctx: &mut Dom,
        to_js: impl FnOnce(&Cache) -> JsValue,
    ) where
        Cache: PartialEq<Cache>,
    {
        self.update_with_js_string(data, dom_ctx, move |v| js::js_string(to_js(v)))
    }

    pub fn update_with_js_string(
        &mut self,
        data: Cache,
        dom_ctx: &mut Dom,
        to_js: impl FnOnce(&Cache) -> JsString,
    ) where
        Cache: PartialEq<Cache>,
    {
        map_or_insert_with_ctx(
            &mut self.inner,
            (dom_ctx, data, to_js),
            |this, (dom_ctx, data, to_js)| {
                if this.cache != data {
                    let s = to_js(&data);
                    this.node.unchecked_ref::<js::Text>().set_data(s);
                    this.cache = data;
                }
                dom_ctx
                    .next_node_position
                    .set_as_insert_after(this.node.clone().into());
            },
            |(dom_ctx, data, to_js)| {
                let s = to_js(&data);
                let text = dom_ctx
                    .document
                    .unchecked_ref::<js::Document>()
                    .create_text_node(s);
                dom_ctx.next_node_position.add_node(text.clone().into());
                StateInner {
                    node: text,
                    cache: data,
                }
            },
        );
    }
}

impl<Cache> Unpin for State<Cache> {}

impl<Cache> RenderState for State<Cache> {
    #[inline]
    fn new_uninitialized() -> Self {
        Self { inner: None }
    }

    fn unmount(self: std::pin::Pin<&mut Self>) {
        let this = self.get_mut();
        if let Some(inner) = this.inner.take() {
            inner.node.remove()
        }
    }
}

macro_rules! impl_render_str {
    ($(
        $(@[$($generics:tt)*])?
        $for_ty:ty
    ),* $(,)?) => {$(
        impl $(<$($generics)*>)? UpdateRenderState<Dom> for $for_ty {
            type State = State<<Self as RenderingStr>::Cache>;

            #[inline]
            fn update_render_state(self, ctx: &mut Dom, state: std::pin::Pin<&mut Self::State>) {
                state.get_mut().update_with_str(self, ctx)
            }
        }
    )*};
}

impl_render_str! {
    Cow<'_, str>,
    &str,
    String,
    @[S: StaticStr] StaticText<S>,
}
