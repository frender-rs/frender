use std::{future::Future, io, pin::Pin, task::Poll};

use frender_core::{RenderState, UpdateRenderState};
use futures_io::AsyncWrite;

use crate::{AnySsr, AnySsrContext, SsrContext};

pin_project_lite::pin_project!(
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct RenderToWriter<W: AsyncWrite, S: RenderState<AnySsr>>
    where
        W: Unpin,
    {
        context: SsrContext<W>,
        #[pin]
        state: S,
    }
);

impl<W: AsyncWrite + Unpin, S: RenderState<AnySsr>> Future for RenderToWriter<W, S> {
    type Output = io::Result<()>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        this.context
            .write_as_any(|ctx| match this.state.poll_reactive(ctx, cx) {
                Poll::Ready(()) => Poll::Ready(match &mut ctx.writer_or_error {
                    crate::WriterOrError::Writer(_) => Ok(()),
                    crate::WriterOrError::Error(error) => {
                        Err(std::mem::replace(error, io::ErrorKind::Other.into()))
                    }
                }),
                Poll::Pending => Poll::Pending,
            })
    }
}

pub trait MapWriteResult<W> {
    type Out;

    fn map_write_result(self, write_result: io::Result<W>) -> Self::Out;
}

impl<W, F, Out> MapWriteResult<W> for F
where
    F: FnOnce(io::Result<W>) -> Out,
{
    type Out = Out;

    fn map_write_result(self, write_result: io::Result<W>) -> Self::Out {
        self(write_result)
    }
}

pub struct Identity;

impl<W> MapWriteResult<W> for Identity {
    type Out = io::Result<W>;

    fn map_write_result(self, write_result: io::Result<W>) -> Self::Out {
        write_result
    }
}

pub struct BytesIntoString;

impl MapWriteResult<Vec<u8>> for BytesIntoString {
    type Out = io::Result<String>;

    fn map_write_result(self, write_result: io::Result<Vec<u8>>) -> Self::Out {
        write_result.and_then(|bytes| {
            String::from_utf8(bytes).map_err(|error| io::Error::new(io::ErrorKind::Other, error))
        })
    }
}

pin_project_lite::pin_project!(
    pub struct RenderAs<M: MapWriteResult<W>, W: AsyncWrite, S: RenderState<AnySsr>>
    where
        W: Unpin,
    {
        map: Option<M>,
        context: Option<SsrContext<W>>,
        #[pin]
        state: S,
    }
);

impl<M: MapWriteResult<W>, W: AsyncWrite + Unpin, S: RenderState<AnySsr>> RenderAs<M, W, S> {
    fn from_element<E>(writer: W, element: E, map: M) -> Self
    where
        E: UpdateRenderState<AnySsr, State = S>,
    {
        let mut context = SsrContext::new(writer);

        let state = context.write_as_any(|ctx| E::initialize_render_state(element, ctx));

        Self {
            map: Some(map),
            context: Some(context),
            state,
        }
    }
}

impl<M: MapWriteResult<W>, W: AsyncWrite + Unpin, S: RenderState<AnySsr>> Future
    for RenderAs<M, W, S>
{
    type Output = M::Out;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        if let Some(context) = this.context {
            match context.write_as_any(|ctx| this.state.poll_reactive(ctx, cx)) {
                Poll::Ready(()) => {
                    let context = this.context.take().unwrap();
                    Poll::Ready(
                        this.map
                            .take()
                            .unwrap()
                            .map_write_result(context.writer_or_error.into()),
                    )
                }
                Poll::Pending => Poll::Pending,
            }
        } else {
            panic!("RenderAs cannot be re-polled after returned ready")
        }
    }
}

pub trait SsrExt: Sized + UpdateRenderState<AnySsr> {
    fn render_to_writer<W: AsyncWrite + Unpin>(
        self,
        writer: &mut W,
    ) -> RenderToWriter<Pin<&mut dyn AsyncWrite>, <Self as UpdateRenderState<AnySsr>>::State> {
        let writer = Pin::new(writer);
        let mut context: AnySsrContext = SsrContext::new(writer);
        let state = self.initialize_render_state(&mut context);

        RenderToWriter { context, state }
    }

    fn render_as_bytes(
        self,
    ) -> RenderAs<Identity, Vec<u8>, <Self as UpdateRenderState<AnySsr>>::State> {
        RenderAs::from_element(vec![], self, Identity)
    }

    fn render_as_string(
        self,
    ) -> RenderAs<BytesIntoString, Vec<u8>, <Self as UpdateRenderState<AnySsr>>::State> {
        RenderAs::from_element(vec![], self, BytesIntoString)
    }
}

impl<E: UpdateRenderState<AnySsr>> SsrExt for E {}

pub async fn render_to_writer<'a, W: AsyncWrite + Unpin, E: UpdateRenderState<AnySsr>>(
    element: E,
    writer: &'a mut W,
) -> io::Result<()> {
    element.render_to_writer(writer).await
}

pub async fn render_to_string<E: for<'a> UpdateRenderState<AnySsr>>(
    element: E,
) -> io::Result<String> {
    let mut buf = Vec::<u8>::new();

    render_to_writer(element, &mut buf).await?;

    // Writing is implemented by this crate and should write utf8.
    // Thus, just unwrap.
    Ok(String::from_utf8(buf).unwrap())
}
