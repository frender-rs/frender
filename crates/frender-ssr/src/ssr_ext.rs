use std::{future::Future, io, pin::Pin, task::Poll};

use frender_core::{RenderState, UpdateRenderState};
use futures_io::AsyncWrite;

use crate::{AnySsrContext, SsrContext};

#[cfg(prototyping)]
pin_project_lite::pin_project!(
    pub struct RenderIntoWriter<S: RenderState> {
        #[pin]
        state: S,
    }
);

#[cfg(prototyping)]
impl<S: RenderState> Future for RenderIntoWriter<S> {
    type Output = Poll<()>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

// pub trait SsrExt<'a>: Sized + UpdateRenderState<AnySsrContext<'a>> {
//     fn render_into_writer<W: AsyncWrite + Unpin>(
//         self,
//         writer: &mut W,
//     ) -> RenderIntoWriter<<Self as UpdateRenderState<AnySsrContext<'a>>>::State> {
//         let writer = Pin::new(writer);
//         let mut ctx: AnySsrContext = SsrContext::new(writer);
//         let state = self.initialize_render_state(&mut ctx);

//         RenderIntoWriter { state }
//     }
// }

// impl<E: for<'a> UpdateRenderState<AnySsrContext<'a>>> SsrExt for E {}

pub async fn render_to_writer<
    'a,
    W: AsyncWrite + Unpin,
    E: UpdateRenderState<AnySsrContext<'a>>,
>(
    element: E,
    writer: &'a mut W,
) -> io::Result<()> {
    let writer = Pin::new(writer);
    let mut ctx: AnySsrContext = SsrContext::new(writer);
    let state = element.initialize_render_state(&mut ctx);

    let mut state = std::pin::pin!(state);

    let _: () = std::future::poll_fn(|cx| state.as_mut().poll_reactive(&mut ctx, cx)).await;

    ctx.finish()
}

pub async fn render_to_string<E: for<'a> UpdateRenderState<AnySsrContext<'a>>>(
    element: E,
) -> io::Result<String> {
    let mut buf = Vec::<u8>::new();

    render_to_writer(element, &mut buf).await?;

    // Writing is implemented by this crate and should write utf8.
    // Thus, just unwrap.
    Ok(String::from_utf8(buf).unwrap())
}
