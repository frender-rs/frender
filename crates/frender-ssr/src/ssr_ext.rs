use std::{future::Future, io, pin::Pin, task::Poll};

use frender_core::{RenderState, UpdateRenderState};
use futures_io::AsyncWrite;

use crate::{AnySsrContext, SsrContext};

pin_project_lite::pin_project!(
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct RenderToWriter<W: AsyncWrite, S: RenderState<SsrContext<W>>>
    where
        W: Unpin,
    {
        context: SsrContext<W>,
        #[pin]
        state: S,
    }
);

impl<W: AsyncWrite + Unpin, S: RenderState<SsrContext<W>>> Future for RenderToWriter<W, S> {
    type Output = io::Result<()>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        match this.state.poll_reactive(this.context, cx) {
            Poll::Ready(()) => Poll::Ready(match &mut this.context.writer_or_error {
                crate::WriterOrError::Writer(_) => Ok(()),
                crate::WriterOrError::Error(error) => {
                    Err(std::mem::replace(error, io::ErrorKind::Other.into()))
                }
            }),
            Poll::Pending => Poll::Pending,
        }
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
    pub struct RenderAs<M: MapWriteResult<W>, W: AsyncWrite, S>
    where
        W: Unpin,
    {
        map: Option<M>,
        context: Option<SsrContext<W>>,
        #[pin]
        state: S,
    }
);

impl<M: MapWriteResult<W>, W: AsyncWrite + Unpin, S: for<'c> RenderState<AnySsrContext<'c>>>
    RenderAs<M, W, S>
{
    fn from_element<E>(writer: W, element: E, map: M) -> Self
    where
        E: for<'c> UpdateRenderState<AnySsrContext<'c>, State = S>,
    {
        let mut context = SsrContext::new(writer);

        let state = context.write_as_any(|ctx| element.initialize_render_state(ctx));

        Self {
            map: Some(map),
            context: Some(context),
            state,
        }
    }
}

impl<M: MapWriteResult<W>, W: AsyncWrite + Unpin, S: for<'c> RenderState<AnySsrContext<'c>>> Future
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

pub trait SsrExt<'ctx>: Sized + UpdateRenderState<AnySsrContext<'ctx>> {
    fn render_to_writer<W: AsyncWrite + Unpin>(
        self,
        writer: &'ctx mut W,
    ) -> RenderToWriter<
        Pin<&'ctx mut dyn AsyncWrite>,
        <Self as UpdateRenderState<AnySsrContext<'ctx>>>::State,
    > {
        let writer = Pin::new(writer);
        let mut context: AnySsrContext = SsrContext::new(writer);
        let state = self.initialize_render_state(&mut context);

        RenderToWriter { context, state }
    }
}

pub trait AnySsrExt: for<'ctx> SsrExt<'ctx>
where
    for<'ctx> Self: UpdateRenderState<AnySsrContext<'ctx>, State = Self::SsrState>,
{
    type SsrState: for<'ctx> RenderState<AnySsrContext<'ctx>>;

    fn render_as_bytes(self) -> RenderAs<Identity, Vec<u8>, Self::SsrState> {
        RenderAs::from_element(vec![], self, Identity)
    }

    fn render_as_string(self) -> RenderAs<BytesIntoString, Vec<u8>, Self::SsrState> {
        RenderAs::from_element(vec![], self, BytesIntoString)
    }
}

impl<'ctx, E: UpdateRenderState<AnySsrContext<'ctx>>> SsrExt<'ctx> for E {}
impl<S, E> AnySsrExt for E
where
    E: for<'ctx> UpdateRenderState<AnySsrContext<'ctx>, State = S>,
    S: for<'ctx> RenderState<AnySsrContext<'ctx>>,
{
    type SsrState = S;
}

pub async fn render_to_writer<
    'a,
    W: AsyncWrite + Unpin,
    E: UpdateRenderState<AnySsrContext<'a>>,
>(
    element: E,
    writer: &'a mut W,
) -> io::Result<()> {
    element.render_to_writer(writer).await
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
