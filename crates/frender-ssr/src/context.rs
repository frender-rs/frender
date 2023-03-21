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
    writer_or_error: WriterOrError<W>,
    pub(crate) busy: bool,
}

impl<W: AsyncWrite + Unpin> SsrContext<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer_or_error: WriterOrError::Writer(writer),
            busy: false,
        }
    }
}

impl<W: AsyncWrite + Unpin> SsrContext<W> {
    pub fn finish(self) -> std::io::Result<()> {
        if self.busy {
            return Err(std::io::ErrorKind::Other.into()); // TODO: better error type
        }

        match self.writer_or_error {
            crate::WriterOrError::Writer(_) => Ok(()),
            crate::WriterOrError::Error(error) => Err(error),
        }
    }

    pub(crate) fn use_writer_and_mark_busy(
        &mut self,
        is_writing: &mut bool,
    ) -> Option<&mut WriterOrError<W>> {
        if self.busy && !*is_writing {
            None
        } else {
            self.busy = true;
            *is_writing = true;
            Some(&mut self.writer_or_error)
        }
    }
}

pub type AnySsrContext<'a> = SsrContext<std::pin::Pin<&'a mut dyn AsyncWrite>>;
