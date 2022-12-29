use futures_io::AsyncWrite;

#[non_exhaustive]
pub struct SsrWriter<W: AsyncWrite + Unpin> {
    pub writer: W,
    pub error: Option<std::io::Error>,
}

pub struct SsrContext<W: AsyncWrite + Unpin> {
    pub writer: Option<SsrWriter<W>>,
}

impl<W: AsyncWrite + Unpin> Default for SsrContext<W> {
    fn default() -> Self {
        Self { writer: None }
    }
}

pub type AnySsrContext = SsrContext<std::pin::Pin<Box<dyn AsyncWrite>>>;
