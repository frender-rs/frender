#![allow(non_camel_case_types)]

use bg::builder;

use frender_core::UpdateRenderState;
use frender_dom::Dom;
use frender_ssr::{AsyncWrite, SsrContext};

use crate::{
    props::{events, UpdateDomEventListener},
    utils::dom::insert_element_and_update,
};

builder! {
    pub struct ButtonProps {
        children[impl Sized],
        on_click[impl Sized],
    }
}

pub mod dom {
    use frender_core::RenderState;

    pin_project_lite::pin_project! {
        pub struct State<Children, OnClick> {
            pub mounted: bool,
            pub node: Option<web_sys::HtmlButtonElement>,
            #[pin]
            pub children: Children,
            pub on_click: OnClick,
        }
    }

    impl<Children, OnClick> RenderState for State<Children, OnClick>
    where
        Children: RenderState,
        OnClick: Default,
    {
        fn new_uninitialized() -> Self {
            Self {
                mounted: false,
                node: None,
                children: RenderState::new_uninitialized(),
                on_click: Default::default(),
            }
        }

        fn unmount(self: std::pin::Pin<&mut Self>) {
            if !self.mounted || self.node.is_none() {
                return;
            }

            let this = self.project();
            if let Some(node) = this.node {
                node.remove()
            }
        }
    }
}

pub mod ssr {
    use std::{borrow::Cow, pin::Pin, task::Poll};

    use frender_ssr::{AsyncWrite, SsrContext, SsrWriter};

    use frender_core::RenderState;

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
            ctx: &mut SsrContext<W>,
        ) {
            self.0 = ctx.writer.take().map(|writer| {
                let buf: Cow<str> = buf.into();

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

builder! {
    pub struct button(ButtonProps);

    pub use build as build_element;
}

impl<TypeDefs: ?Sized + button::Types> UpdateRenderState<Dom> for button::Data<TypeDefs>
where
    TypeDefs::children: UpdateRenderState<Dom>,
    TypeDefs::on_click: UpdateDomEventListener<events::OnClick>,
{
    type State = dom::State<
        <TypeDefs::children as UpdateRenderState<Dom>>::State,
        <TypeDefs::on_click as UpdateDomEventListener<events::OnClick>>::State,
    >;

    #[inline]
    fn update_render_state(self, ctx: &mut Dom, state: std::pin::Pin<&mut Self::State>) {
        let props = self.0;
        let state = state.project();

        insert_element_and_update(state.node, ctx, "button", |node, children_ctx| {
            props
                .children
                .update_render_state(children_ctx, state.children);

            props
                .on_click
                .update_dom_event_listener(node, state.on_click);
        })
    }
}

impl<W: AsyncWrite + Unpin, TypeDefs: ?Sized + button::Types> UpdateRenderState<SsrContext<W>>
    for button::Data<TypeDefs>
where
    TypeDefs::children: UpdateRenderState<SsrContext<W>>,
    TypeDefs::on_click: UpdateDomEventListener<events::OnClick>,
{
    type State = ssr::State<W>;

    #[inline]
    fn update_render_state(self, ctx: &mut SsrContext<W>, state: std::pin::Pin<&mut Self::State>) {
        // state.get_mut().update_render_state_with_str(self.0, ctx);
        // todo!()
    }
}

// crate::impl_render_dom_or_ssr! {
//     [TypeDefs: ?Sized + button::Types] for button::Data<TypeDefs>
//     where
//         TypeDefs::children: UpdateRenderState<Dom>,
//         TypeDefs::on_click: UpdateDomEventListener<events::OnClick>,
// }
