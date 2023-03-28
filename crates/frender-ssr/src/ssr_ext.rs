use std::{future::Future, io, pin::Pin, task::Poll};

use futures_io::AsyncWrite;

use crate::{Element, RenderState};

pin_project_lite::pin_project!(
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct RenderToWriter<W: AsyncWrite, S: RenderState> {
        #[pin]
        writer: W,
        #[pin]
        state: S,
    }
);

impl<W: AsyncWrite, S: RenderState> Future for RenderToWriter<W, S> {
    type Output = io::Result<()>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        this.state.poll_render(this.writer, cx)
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
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct RenderAs<M: MapWriteResult<W>, W: AsyncWrite, S>
    where
        W: Unpin,
    {
        map_and_writer: Option<(M, W)>,
        #[pin]
        state: S,
    }
);

impl<M: MapWriteResult<W>, W: AsyncWrite + Unpin, S: RenderState> RenderAs<M, W, S> {
    fn from_element<E>(writer: W, element: E, map: M) -> Self
    where
        E: Element<SsrState = S>,
    {
        let state = element.into_ssr_state();

        Self {
            map_and_writer: Some((map, writer)),
            state,
        }
    }
}

impl<M: MapWriteResult<W>, W: AsyncWrite + Unpin, S: RenderState> Future for RenderAs<M, W, S> {
    type Output = M::Out;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        if let Some((_, writer)) = this.map_and_writer {
            match this.state.poll_render(Pin::new(writer), cx) {
                Poll::Ready(res) => {
                    let (map, writer) = this.map_and_writer.take().unwrap();
                    Poll::Ready(map.map_write_result(res.and(Ok(writer))))
                }
                Poll::Pending => Poll::Pending,
            }
        } else {
            panic!("RenderAs cannot be re-polled after returned ready")
        }
    }
}

pub trait ElementExt: Sized + Element {
    fn render_to_writer<W: AsyncWrite + Unpin>(
        self,
        writer: &mut W,
    ) -> RenderToWriter<&mut W, Self::SsrState> {
        let state = self.into_ssr_state();

        RenderToWriter { writer, state }
    }

    fn render_as_bytes(self) -> RenderAs<Identity, Vec<u8>, Self::SsrState> {
        RenderAs::from_element(vec![], self, Identity)
    }

    fn render_as_string(self) -> RenderAs<BytesIntoString, Vec<u8>, Self::SsrState> {
        RenderAs::from_element(vec![], self, BytesIntoString)
    }
}

impl<E: Element> ElementExt for E {}
