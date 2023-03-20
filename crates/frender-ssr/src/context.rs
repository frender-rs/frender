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
    writer_or_error: Option<WriterOrError<W>>,
}

impl<W: AsyncWrite + Unpin> SsrContext<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer_or_error: Some(WriterOrError::Writer(writer)),
        }
    }
}

impl<W: AsyncWrite + Unpin> SsrContext<W> {
    pub fn expect_to_take_writer(&mut self) -> WriterOrError<W> {
        self.writer_or_error
            .take()
            .expect("writer_or_error is available")
    }
}

impl<W: AsyncWrite + Unpin> Default for SsrContext<W> {
    fn default() -> Self {
        Self {
            writer_or_error: None,
        }
    }
}

pub type AnySsrContext<'a> = SsrContext<std::pin::Pin<&'a mut dyn AsyncWrite>>;
