use futures_io::AsyncWrite;

#[non_exhaustive]
pub struct SsrWriter<W> {
    pub writer: W,
    pub error: Option<std::io::Error>,
}

pub enum WriterOrError<W> {
    Writer(W),
    Error(std::io::Error),
}

pub struct SsrContext<W: AsyncWrite + Unpin> {
    pub writer_or_error: Option<WriterOrError<W>>,
}

impl<W: AsyncWrite + Unpin> Default for SsrContext<W> {
    fn default() -> Self {
        Self {
            writer_or_error: None,
        }
    }
}

pub type AnySsrContext = SsrContext<std::pin::Pin<Box<dyn AsyncWrite>>>;
