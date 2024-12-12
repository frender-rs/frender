use std::borrow::Cow;

use frender_common::ToAsRefStr;
use frender_dom::string_element::StringElement;
use frender_dom::ui_handle::{UiHandle, UnmountedUiHandle};

use crate::element::{PinnedRenderStateKind, PinnedRenderStateKindPollRender, RenderStates, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender};
use crate::kinds::UiHandleWithNonReactiveState;
use crate::{dom::behaviors::Node, RenderHtml};

use crate::{CsrElement, HtmlRenderContext};

// region: kind

pub struct Kind<Cache: 'static>(super::Kind<Cache>);

impl<Cache: 'static> PinnedRenderStateKind for Kind<Cache> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = UiHandleWithNonReactiveState<R::Text, Cache>;
    type PinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
    type PinnedReactiveState = ();
}

impl<Cache: 'static> PinnedRenderStateKindPollRender for Kind<Cache> {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        states: crate::element::PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        std::task::Poll::Ready(())
    }
}

impl<Cache: 'static> UnpinnedRenderStateKind for Kind<Cache> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = R::Text;
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = Cache;
    type UnpinnedReactiveState = ();
}

impl<Cache: 'static> UnpinnedRenderStateKindPollRender for Kind<Cache> {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        states: crate::element::UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        std::task::Poll::Ready(())
    }
}

// endregion

// impl<T: known RenderAsTextWithSelfAsCache> Element for T {}
frender_common::impl_many!(
    impl<__> CsrElement
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

        fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            _: crate::element::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
        ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
            self.render_init_as_text_with_cache(render_context)
        }

        fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            states: crate::element::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
        ) {
            let UiHandleWithNonReactiveState { ui_handle, non_reactive_state } = states.ui_handle;
            self.render_update_as_text_with_cache(render_context.renderer_mut(), ui_handle, non_reactive_state);
        }

        fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
        ) -> crate::element::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
            let UiHandleWithNonReactiveState { ui_handle, non_reactive_state } = self.render_init_as_text_with_cache(render_context);
            RenderStates {
                ui_handle,
                non_reactive_state,
                reactive_state: (),
            }
        }

        fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            states: crate::element::UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
        ) {
            let RenderStates {
                ui_handle,
                non_reactive_state,
                reactive_state: (),
            } = states;
            self.render_update_as_text_with_cache(render_context.renderer_mut(), ui_handle, non_reactive_state);
        }
    }
);

/// <code>where TempStr\<S>: [CsrStr](frender_common::strings::CsrStr)</code>
///
/// `impl CsrStr` -> `impl RenderAsTextWithCache` -> `impl Element`
impl<S> CsrElement for frender_common::TempStr<S>
where
    S: frender_common::IntoStaticStrCache,
{
    type RenderStateKind = Kind<S::StaticStrCache>;

    fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        _: crate::element::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        self.render_init_as_text_with_cache(render_context)
    }

    fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        let UiHandleWithNonReactiveState { ui_handle, non_reactive_state } = states.ui_handle;
        self.render_update_as_text_with_cache(render_context.renderer_mut(), ui_handle, non_reactive_state);
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> crate::element::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        let UiHandleWithNonReactiveState { ui_handle, non_reactive_state } = self.render_init_as_text_with_cache(render_context);
        RenderStates {
            ui_handle,
            non_reactive_state,
            reactive_state: (),
        }
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        let RenderStates {
            ui_handle,
            non_reactive_state,
            reactive_state: (),
        } = states;
        self.render_update_as_text_with_cache(render_context.renderer_mut(), ui_handle, non_reactive_state);
    }
}

pub trait RenderAsTextWithCache {
    type Cache: 'static;

    fn not_match_cache(&self, cache: &Self::Cache) -> bool;

    /// This method will only be called on initialization.
    fn render_as_text_with_cache<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
    ) -> (<Renderer::Text as UiHandle<Renderer>>::Unmounted, Self::Cache);

    /// This method will only be called on [cache mismatch](RenderAsTextWithCache::not_match_cache).
    /// The argument `cache` is stale and the implementation should update it.
    fn render_update_as_text_with_cache<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        text_handle: &mut Renderer::Text,
        cache: &mut Self::Cache,
    );

    fn render_init_as_text_with_cache<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
    ) -> UiHandleWithNonReactiveState<
        //
        <Ctx::Renderer as RenderHtml>::Text,
        Self::Cache,
    >
    where
        Self: Sized,
    {
        let (text, cache) = self.render_as_text_with_cache(render_context.renderer_mut());

        let text = render_context.map_mut_render_context(|render_context| text.mount(render_context));

        UiHandleWithNonReactiveState {
            ui_handle: text,
            non_reactive_state: cache,
        }
    }
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
    ) -> (<Renderer::Text as UiHandle<Renderer>>::Unmounted, Self::Cache) {
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
    ) -> (<Renderer::Text as UiHandle<Renderer>>::Unmounted, Self::Cache) {
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
    fn render_as_text<R: ?Sized + RenderHtml>(&self, renderer: &mut R) -> <R::Text as UiHandle<R>>::Unmounted;
    fn render_update_as_text<R: ?Sized + RenderHtml>(&self, renderer: &mut R, text_handle: &mut R::Text);
}

impl<S: KnownStaticStr> RenderAsTextWithSelfAsCache for S {
    fn render_as_text<R: ?Sized + RenderHtml>(&self, renderer: &mut R) -> <R::Text as UiHandle<R>>::Unmounted {
        renderer.render_text_from(self.as_ref())
    }

    fn render_update_as_text<R: ?Sized + RenderHtml>(&self, renderer: &mut R, text_handle: &mut R::Text) {
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
        fn render_as_text<R: ?Sized + RenderHtml>(&self, renderer: &mut R) -> <R::Text as UiHandle<R>>::Unmounted {
            renderer.render_text_from(self)
        }

        fn render_update_as_text<R: ?Sized + RenderHtml>(&self, renderer: &mut R, text_handle: &mut R::Text) {
            renderer.update_text_from(text_handle, self)
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
