use std::borrow::{Borrow, Cow};

// use wasm_bindgen::{JsCast, JsValue};

use frender_dom::render::{RenderContext, RenderWithContext};
use frender_dom::string_element::StringElement;

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

trait BorrowText: Borrow<Self::Text> {
    type Text: ?Sized;
}

frender_common::impl_many!(
    impl<__> BorrowText
        for each_of![
            //
            &'static str,
            String,
            Cow<'static, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type Text = str;
    }
);

frender_common::impl_many!(
    impl<__> BorrowText
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
        type Text = Self;
    }
);

trait FnOnceOutputBorrow<Arg, V: ?Sized>: FnOnce(Arg) -> Self::OutputBorrow {
    type OutputBorrow: Borrow<V>;
}

impl<Arg, V: ?Sized, F: FnOnce(Arg) -> R, R: Borrow<V>> FnOnceOutputBorrow<Arg, V> for F {
    type OutputBorrow = R;
}

impl<Cache, Text> State<Cache, Text> {
    fn update_with_str_maybe_reposition<Ctx: ?Sized + RenderContext, S: Borrow<V>, V: ?Sized>(
        &mut self,
        data: S,
        render_context: &mut Ctx,
        force_reposition: bool,
        not_match_cache: impl FnOnce(&S, &Cache) -> bool,
        update_cache: impl FnOnce(S, &mut Cache),
    ) where
        Ctx::Renderer: RenderHtml<Text = Text> + RenderTextFrom<Text, V>,
        Text: Node<Ctx::Renderer>,
    {
        self.update_maybe_reposition::<Ctx, S, V>(data, render_context, force_reposition, Borrow::borrow, not_match_cache, update_cache)
    }

    fn update_maybe_reposition<Ctx: ?Sized + RenderContext, S, V: ?Sized>(
        &mut self,
        data: S,
        render_context: &mut Ctx,
        force_reposition: bool,
        get_value: impl for<'a> FnOnceOutputBorrow<&'a S, V>,
        not_match_cache: impl FnOnce(&S, &Cache) -> bool,
        update_cache: impl FnOnce(S, &mut Cache),
    ) where
        Ctx::Renderer: RenderHtml<Text = Text> + RenderTextFrom<Text, V>,
        Text: Node<Ctx::Renderer>,
    {
        if not_match_cache(&data, &self.cache) {
            render_context.renderer_mut().update_text_from(&mut self.text_node.node, get_value(&data).borrow());

            update_cache(data, &mut self.cache);
        }

        render_context.map_mut_render_context(|render_context| self.text_node.readd_self(render_context, force_reposition))
    }

    pub fn initialize_with_str<Ctx: ?Sized + RenderContext, S: Borrow<V>, V: ?Sized>(data: S, render_context: &mut Ctx, create_cache: impl FnOnce(S) -> Cache) -> Self
    where
        Ctx::Renderer: RenderHtml<Text = Text> + RenderTextFrom<Text, V>,
        Text: Node<Ctx::Renderer>,
    {
        Self::init::<Ctx, S, V>(data, render_context, Borrow::borrow, create_cache)
    }

    fn init<Ctx: ?Sized + RenderContext, S, V: ?Sized>(
        //
        data: S,
        render_context: &mut Ctx,
        get_value: impl for<'a> FnOnceOutputBorrow<&'a S, V>,
        create_cache: impl FnOnce(S) -> Cache,
    ) -> Self
    where
        Ctx::Renderer: RenderHtml<Text = Text> + RenderTextFrom<Text, V>,
        Text: Node<Ctx::Renderer>,
    {
        State {
            text_node: {
                let text_node = TextNode::mount_from(render_context, get_value(&data).borrow());
                text_node
            },
            cache: create_cache(data),
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
            // static strings
            Cow<'static, str>,
            &'static str,
            String,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
            //
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
        type RenderStateKind = Kind<Self>;

        fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: std::pin::Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>, force_reposition: bool) {
            match render_state.get_mut() {
                Some(render_state) => render_state.update_with_str_maybe_reposition::<Ctx, Self, <Self as BorrowText>::Text>(self, render_context, force_reposition, PartialEq::ne, |this, cache| *cache = this),
                render_state @ None => *render_state = Some(State::initialize_with_str::<Ctx, Self, <Self as BorrowText>::Text>(self, render_context, std::convert::identity)),
            }
        }

        crate::impl_unpinned_render_for_unpin! {}
    }
);

/// <code>where TempStr<S>: [CsrStr](frender_common::strings::CsrStr)</code>
impl<S> Element for frender_common::TempStr<S>
where
    S: frender_common::IntoStaticStrCache,
{
    type RenderStateKind = Kind<S::StaticStrCache>;

    fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: std::pin::Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>, force_reposition: bool) {
        fn to_as_ref_str_and_borrow(v: &impl frender_common::ToAsRefStr) -> impl '_ + Borrow<str> {
            let v = frender_common::ToAsRefStr::to_as_ref_str(v);
            struct BorrowAsRef<S>(S);

            impl<S: AsRef<str>> Borrow<str> for BorrowAsRef<S> {
                fn borrow(&self) -> &str {
                    self.0.as_ref()
                }
            }

            BorrowAsRef(v)
        }

        match render_state.get_mut() {
            Some(render_state) => {
                if render_state.cache != self.0 {
                    use frender_common::ToAsRefStr as _;
                    self.0.update_into_static_str_cache(&mut render_state.cache);
                    render_context.renderer_mut().update_text_from(&mut render_state.text_node.node, render_state.cache.to_as_ref_str().as_ref());
                }

                render_context.map_mut_render_context(|render_context| render_state.text_node.readd_self(render_context, force_reposition))
            }
            render_state @ None => *render_state = Some(State::init(self.0.into_static_str_cache(), render_context, to_as_ref_str_and_borrow, std::convert::identity)),
        }
    }

    crate::impl_unpinned_render_for_unpin! {}
}
