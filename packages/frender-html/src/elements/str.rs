use std::borrow::Cow;

use frender_common::ToAsRefStr;
use frender_dom::render::{RenderContext, RenderTextFromKnown, RenderWithContext};
use frender_dom::string_element::StringElement;

use crate::{dom::behaviors::Node, RenderHtml};

use crate::{Element, HtmlRenderContext, RenderState};

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
}

pub struct State<Cache, Text> {
    text_node: TextNode<Text>,
    cache: Cache,
}

impl<Cache, Text> State<Cache, Text> {
    pub fn init_with<S: RenderAsTextWithCache<Cache = Cache>, Ctx: ?Sized + RenderContext>(
        //
        data: S,
        render_context: &mut Ctx,
    ) -> Self
    where
        Ctx::Renderer: RenderHtml<Text = Text>,
        Text: Node<Ctx::Renderer>,
    {
        let (text, cache) = data.render_as_text_with_cache(render_context.renderer_mut());
        State {
            text_node: {
                let text_node = TextNode::mount(render_context, text);
                text_node
            },
            cache,
        }
    }

    pub fn update_maybe_reposition_with<S: RenderAsTextWithCache<Cache = Cache>, Ctx: ?Sized + RenderContext>(
        //
        &mut self,
        data: S,
        render_context: &mut Ctx,
        force_reposition: bool,
    ) where
        Ctx::Renderer: RenderHtml<Text = Text>,
        Text: Node<Ctx::Renderer>,
    {
        if S::not_match_cache(&data, &self.cache) {
            data.render_update_as_text_with_cache(
                //
                render_context.renderer_mut(),
                &mut self.text_node.node,
                &mut self.cache,
            )
        }

        render_context.map_mut_render_context(|render_context| self.text_node.readd_self(render_context, force_reposition))
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

pub fn unpinned_render_update_maybe_reposition<E: RenderAsTextWithCache, Ctx: ?Sized + HtmlRenderContext>(
    el: E,
    render_context: &mut Ctx,
    render_state: &mut Option<State<E::Cache, <Ctx::Renderer as RenderHtml>::Text>>,
    force_reposition: bool,
) {
    match render_state {
        Some(render_state) => render_state.update_maybe_reposition_with::<E, Ctx>(el, render_context, force_reposition),
        render_state @ None => *render_state = Some(State::init_with::<E, Ctx>(el, render_context)),
    }
}
pub struct Kind<Cache: 'static>(super::Kind<Cache>);

impl<Cache: 'static> crate::RenderStateKindPinned for Kind<Cache> {
    type RenderState<R: RenderHtml + ?Sized> = Option<State<Cache, R::Text>>;
}
impl<Cache: 'static> crate::RenderStateKindUnpinned for Kind<Cache> {
    type UnpinnedRenderState<R: RenderHtml + ?Sized> = Option<State<Cache, R::Text>>;
}

// impl<T: known RenderAsTextWithSelfAsCache> Element for T {}
frender_common::impl_many!(
    impl<__> Element
        for each_of![
            // known static strings: impl KnownStaticStr -> impl RenderAsTextWithSelfAsCache
            Cow<'static, str>,
            &'static str,
            String,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
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
        type RenderStateKind = Kind<Self>;

        fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: &mut crate::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
            force_reposition: bool,
        ) {
            unpinned_render_update_maybe_reposition(
                //
                self,
                render_context,
                render_state,
                force_reposition,
            )
        }

        crate::impl_render_for_unpin! {}
    }
);

/// <code>where TempStr\<S>: [CsrStr](frender_common::strings::CsrStr)</code>
///
/// `impl CsrStr` -> `impl RenderAsTextWithCache` -> `impl Element`
impl<S> Element for frender_common::TempStr<S>
where
    S: frender_common::IntoStaticStrCache,
{
    type RenderStateKind = Kind<S::StaticStrCache>;

    fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: &mut crate::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
        force_reposition: bool,
    ) {
        unpinned_render_update_maybe_reposition(self, render_context, render_state, force_reposition)
    }

    crate::impl_render_for_unpin! {}
}

pub trait RenderAsTextWithCache {
    type Cache: 'static;

    fn not_match_cache(&self, cache: &Self::Cache) -> bool;

    /// This method will only be called on initialization.
    fn render_as_text_with_cache<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
    ) -> (Renderer::Text, Self::Cache);

    /// This method will only be called on [cache mismatch](RenderAsTextWithCache::not_match_cache).
    /// The argument `cache` is stale and the implementation should update it.
    fn render_update_as_text_with_cache<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        text_handle: &mut Renderer::Text,
        cache: &mut Self::Cache,
    );
}

impl<T: RenderAsTextWithSelfAsCache> RenderAsTextWithCache for T {
    type Cache = T;

    fn not_match_cache(&self, cache: &Self::Cache) -> bool {
        self != cache
    }

    fn render_as_text_with_cache<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
    ) -> (Renderer::Text, Self::Cache) {
        (self.render_as_text(renderer), self)
    }

    fn render_update_as_text_with_cache<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        text_handle: &mut Renderer::Text,
        cache: &mut Self::Cache,
    ) {
        self.render_update_as_text(renderer, text_handle);
        *cache = self;
    }
}

/// <code>where TempStr\<S>: [CsrStr](frender_common::strings::CsrStr)</code>
impl<S> RenderAsTextWithCache for frender_common::TempStr<S>
where
    S: frender_common::IntoStaticStrCache,
{
    type Cache = S::StaticStrCache;

    fn not_match_cache(&self, cache: &Self::Cache) -> bool {
        *cache != self.0
    }

    fn render_as_text_with_cache<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
    ) -> (Renderer::Text, Self::Cache) {
        let cache = self.0.into_static_str_cache();

        let text = {
            let s = cache.to_as_ref_str();
            renderer.render_text_from(s.as_ref())
        };

        (text, cache)
    }

    fn render_update_as_text_with_cache<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        text_handle: &mut Renderer::Text,
        cache: &mut Self::Cache,
    ) {
        self.0.update_into_static_str_cache(cache);
        let s = cache.to_as_ref_str();
        renderer.update_text_from(text_handle, s.as_ref());
    }
}

trait RenderAsTextWithSelfAsCache: 'static + PartialEq {
    fn render_as_text<Text, R: ?Sized + RenderTextFromKnown<Text>>(&self, renderer: &mut R) -> Text;
    fn render_update_as_text<Text, R: ?Sized + RenderTextFromKnown<Text>>(&self, renderer: &mut R, text_handle: &mut Text);
}

impl<S: KnownStaticStr> RenderAsTextWithSelfAsCache for S {
    fn render_as_text<Text, R: ?Sized + RenderTextFromKnown<Text>>(&self, renderer: &mut R) -> Text {
        renderer.render_text_from(self.as_ref())
    }

    fn render_update_as_text<Text, R: ?Sized + RenderTextFromKnown<Text>>(&self, renderer: &mut R, text_handle: &mut Text) {
        renderer.update_text_from(text_handle, self.as_ref())
    }
}

frender_common::impl_many!(
    impl<__> RenderAsTextWithSelfAsCache
        for each_of![
            StringElement,
            // scalar types
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
        fn render_as_text<Text, R: ?Sized + RenderTextFromKnown<Text>>(&self, renderer: &mut R) -> Text {
            renderer.render_text_from(self)
        }

        fn render_update_as_text<Text, R: ?Sized + RenderTextFromKnown<Text>>(&self, renderer: &mut R, text_handle: &mut Text) {
            renderer.update_text_from(text_handle, self);
        }
    }
);

trait KnownStaticStr: 'static + AsRef<str> + PartialEq {}

frender_common::impl_many!(
    impl<__> KnownStaticStr
        for each_of![
            //
            &'static str,
            String,
            Cow<'static, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
    }
);
