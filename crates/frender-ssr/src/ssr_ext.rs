use std::{future::Future, io, pin::Pin, task::Poll};

use frender_core::{RenderState, UpdateRenderState};
use futures_io::AsyncWrite;

use crate::{AnySsrContext, SsrContext};

pin_project_lite::pin_project!(
    pub struct RenderIntoWriter<S: RenderState> {
        #[pin]
        state: S,
    }
);

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
) {
    let writer = Pin::new(writer);
    let mut ctx: AnySsrContext = SsrContext::new(writer);
    let state = element.initialize_render_state(&mut ctx);

    let mut state = std::pin::pin!(state);

    let _: () = std::future::poll_fn(|cx| state.as_mut().poll_reactive(cx)).await;

    // TODO: this function should return io::Result<()>.
    // However, the WriterOrError is taken into state, and we cannot take the error back.
    // There are 3 ways to fix this:
    // 1. Make RenderState::poll_reactive fallible (it output Poll<Result<(), Ctx::Error>> instead of Poll(())).
    //    This also requires Ctx to have associated Error Type
    // 2. Make RenderState generic over Ctx: RenderState<Ctx> and poll_reactive(ctx)
    //    so that RenderState don't need to take ctx.
    //    This would also allow RenderIntoWriter to be implemented
    // 3. Add a method like `take_context` to RenderState, which returns the taken context.
}

pub async fn render_to_string<E: for<'a> UpdateRenderState<AnySsrContext<'a>>>(
    element: E,
) -> io::Result<String> {
    let mut buf = Vec::<u8>::new();

    render_to_writer(element, &mut buf).await;
    // TODO: process error

    // Writing is implemented by this crate and should write utf8.
    // Thus, just unwrap.
    Ok(String::from_utf8(buf).unwrap())
}
