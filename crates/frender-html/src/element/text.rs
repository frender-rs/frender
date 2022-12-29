use std::borrow::Cow;

use futures_io::AsyncWrite;

use crate::render::{Dom, SsrContext, UpdateRenderState};

pub mod dom {
    use std::borrow::Cow;

    use crate::{render::RenderState, utils::map_or_insert_with_ctx};

    pub struct State {
        mounted: bool,
        node: Option<web_sys::Text>,
        cache: Option<Cow<'static, str>>,
    }

    impl State {
        pub(super) fn update_with_owned(
            &mut self,
            data: Cow<'static, str>,
            dom_ctx: &mut crate::render::Dom,
        ) {
            map_or_insert_with_ctx(
                &mut self.node,
                (dom_ctx, &mut self.mounted),
                |node, (dom_ctx, mounted)| {
                    if self.cache.as_ref() != Some(&data) {
                        node.set_data(&data);
                    }
                    if *mounted {
                        dom_ctx
                            .next_node_position
                            .set_as_insert_after(node.clone().into())
                    } else {
                        *mounted = true;
                        dom_ctx.next_node_position.add_node(node.clone().into());
                    }
                },
                |(dom_ctx, mounted)| {
                    let text = dom_ctx.document.create_text_node(&data);
                    dom_ctx.next_node_position.add_node(text.clone().into());
                    *mounted = true;
                    text
                },
            );

            self.cache = Some(data);
        }

        pub(super) fn update_with_borrowed(
            &mut self,
            data: &str,
            dom_ctx: &mut crate::render::Dom,
        ) {
            map_or_insert_with_ctx(
                &mut self.node,
                (dom_ctx, &mut self.mounted),
                |node, (dom_ctx, mounted)| {
                    if self.cache.as_deref() != Some(data) {
                        node.set_data(data);
                    }
                    if *mounted {
                        dom_ctx
                            .next_node_position
                            .set_as_insert_after(node.clone().into())
                    } else {
                        *mounted = true;
                        dom_ctx.next_node_position.add_node(node.clone().into());
                    }
                },
                |(dom_ctx, mounted)| {
                    let text = dom_ctx.document.create_text_node(data);
                    dom_ctx.next_node_position.add_node(text.clone().into());
                    *mounted = true;
                    text
                },
            );

            self.cache = None;
        }
    }

    impl Unpin for State {}

    impl RenderState for State {
        fn new_uninitialized() -> Self {
            Self {
                mounted: false,
                node: None,
                cache: None,
            }
        }

        fn unmount(self: std::pin::Pin<&mut Self>) {
            if !self.mounted || self.node.is_none() {
                return;
            }
            let this = self.get_mut();
            if let Some(node) = this.node.take() {
                node.remove()
            }
        }
    }
}

pub mod ssr {
    use std::{borrow::Cow, pin::Pin, task::Poll};

    use futures_io::AsyncWrite;

    use crate::render::{RenderState, SsrWriter};

    struct StateInner<W: AsyncWrite + Unpin> {
        writer: SsrWriter<W>,
        owned_buf: Cow<'static, [u8]>,
        written: usize,
    }

    pub struct State<W: AsyncWrite + Unpin>(Option<StateInner<W>>);

    impl<W: AsyncWrite + Unpin> State<W> {
        #[inline]
        pub(super) fn update_render_state_with_str(
            &mut self,
            buf: impl Into<Cow<'static, str>>,
            ctx: &mut crate::render::SsrContext<W>,
        ) {
            self.0 = ctx.writer.take().map(|writer| {
                let buf: Cow<str> = buf.into();
                let escaped = html_escape::encode_safe(&buf);

                let buf = match escaped {
                    Cow::Borrowed(_) => buf,
                    Cow::Owned(escaped) => Cow::Owned(escaped),
                };

                let owned_buf = match buf {
                    Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
                    Cow::Owned(s) => Cow::Owned(s.into_bytes()),
                };

                StateInner {
                    writer,
                    owned_buf,
                    written: 0,
                }
            });
        }
    }

    fn poll_write_all<W: AsyncWrite>(
        mut writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
        written: &mut usize,
    ) -> Poll<std::io::Result<()>> {
        while *written < buf.len() {
            let buf = &buf[*written..];
            let n = futures_lite::ready!(writer.as_mut().poll_write(cx, buf))?;

            if n == 0 {
                return Poll::Ready(Err(std::io::ErrorKind::WriteZero.into()));
            }

            *written += n;
        }

        Poll::Ready(Ok(()))
    }

    impl<W: AsyncWrite + Unpin> RenderState for State<W> {
        fn new_uninitialized() -> Self {
            Self(None)
        }

        fn unmount(self: std::pin::Pin<&mut Self>) {
            self.get_mut().0 = None;
        }

        /// The implementation is from [`futures_lite::io::WriteAllFuture`].
        fn poll_reactive(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<bool> {
            let this = &mut self.get_mut().0;
            if let Some(StateInner {
                writer,
                owned_buf,
                written,
            }) = this
            {
                if writer.error.is_none() {
                    if let Err(err) = futures_lite::ready!(poll_write_all(
                        Pin::new(&mut writer.writer),
                        cx,
                        owned_buf,
                        written
                    )) {
                        writer.error = Some(err)
                    }
                }
            }

            Poll::Ready(false)
        }
    }
}

impl UpdateRenderState<Dom> for Cow<'_, str> {
    type State = dom::State;

    fn update_render_state(self, ctx: &mut Dom, state: std::pin::Pin<&mut Self::State>) {
        match self {
            Cow::Borrowed(data) => data.update_render_state(ctx, state),
            Cow::Owned(data) => data.update_render_state(ctx, state),
        }
    }
}

impl UpdateRenderState<Dom> for &str {
    type State = dom::State;

    #[inline]
    fn update_render_state(self, ctx: &mut Dom, state: std::pin::Pin<&mut Self::State>) {
        state.get_mut().update_with_borrowed(self, ctx)
    }
}

impl UpdateRenderState<Dom> for String {
    type State = dom::State;

    #[inline]
    fn update_render_state(self, ctx: &mut Dom, state: std::pin::Pin<&mut Self::State>) {
        state.get_mut().update_with_owned(self.into(), ctx)
    }
}

pub struct StaticText<S: Into<Cow<'static, str>>>(S);

impl<S: Into<Cow<'static, str>>> UpdateRenderState<Dom> for StaticText<S> {
    type State = dom::State;

    #[inline]
    fn update_render_state(self, ctx: &mut Dom, state: std::pin::Pin<&mut Self::State>) {
        state.get_mut().update_with_owned(self.0.into(), ctx)
    }
}

impl<W: AsyncWrite + Unpin> UpdateRenderState<SsrContext<W>> for Cow<'_, str> {
    type State = ssr::State<W>;

    fn update_render_state(self, ctx: &mut SsrContext<W>, state: std::pin::Pin<&mut Self::State>) {
        match self {
            Cow::Borrowed(data) => data.update_render_state(ctx, state),
            Cow::Owned(data) => data.update_render_state(ctx, state),
        }
    }
}

impl<W: AsyncWrite + Unpin> UpdateRenderState<SsrContext<W>> for &str {
    type State = ssr::State<W>;

    #[inline]
    fn update_render_state(self, ctx: &mut SsrContext<W>, state: std::pin::Pin<&mut Self::State>) {
        state
            .get_mut()
            .update_render_state_with_str(self.to_owned(), ctx)
    }
}

impl<W: AsyncWrite + Unpin> UpdateRenderState<SsrContext<W>> for String {
    type State = ssr::State<W>;

    #[inline]
    fn update_render_state(self, ctx: &mut SsrContext<W>, state: std::pin::Pin<&mut Self::State>) {
        state.get_mut().update_render_state_with_str(self, ctx)
    }
}

impl<S: Into<Cow<'static, str>>, W: AsyncWrite + Unpin> UpdateRenderState<SsrContext<W>>
    for StaticText<S>
{
    type State = ssr::State<W>;

    #[inline]
    fn update_render_state(self, ctx: &mut SsrContext<W>, state: std::pin::Pin<&mut Self::State>) {
        state.get_mut().update_render_state_with_str(self.0, ctx);
    }
}

crate::impl_render_dom_or_ssr! {
    [S: Into<Cow<'static, str>>] for StaticText<S>
}
