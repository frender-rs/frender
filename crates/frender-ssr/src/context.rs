use std::task::Poll;

use futures_io::AsyncWrite;

#[derive(Clone, Copy)]
pub(crate) enum WritingState {
    ReadyToStart,
    Writing,
    Finished,
}

impl WritingState {
    #[must_use]
    pub(crate) fn is_ready_to_start(self) -> bool {
        matches!(self, Self::ReadyToStart)
    }

    #[must_use]
    pub(crate) fn is_finished(self) -> bool {
        matches!(self, Self::Finished)
    }

    // #[must_use]
    // pub(crate) fn is_writing(self) -> bool {
    //     matches!(self, Self::Writing)
    // }
}

impl Default for WritingState {
    fn default() -> Self {
        Self::ReadyToStart
    }
}

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
    pub(crate) writer_or_error: WriterOrError<W>,
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

    pub(crate) fn map(
        &mut self,
        writing: &mut WritingState,
        write: impl FnOnce(&mut Self, &mut std::task::Context<'_>) -> Poll<()>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        match writing {
            WritingState::ReadyToStart => {
                if self.busy {
                    // no need to use cx.waker()
                    // because a former RenderState must also be pending and have used the waker
                    return Poll::Pending;
                } else {
                    *writing = WritingState::Writing;
                    self.busy = true;
                }
            }
            WritingState::Writing => {
                #[cfg(debug_assertions)]
                assert!(self.busy);
            }
            WritingState::Finished => {
                return Poll::Ready(());
            }
        };

        write(self, cx).map(|()| {
            self.busy = false;
            *writing = WritingState::Finished;
        })
    }

    pub(crate) fn map_writer_or_error(
        &mut self,
        writing: &mut WritingState,
        write: impl FnOnce(&mut WriterOrError<W>, &mut std::task::Context<'_>) -> Poll<()>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        self.map(writing, |this, cx| write(&mut this.writer_or_error, cx), cx)
    }

    pub(crate) fn map_writer(
        &mut self,
        writing: &mut WritingState,
        write: impl FnOnce(&mut W, &mut std::task::Context<'_>) -> Poll<std::io::Result<()>>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        self.map_writer_or_error(
            writing,
            |woe, cx| {
                match woe {
                    WriterOrError::Writer(w) => match write(w, cx) {
                        Poll::Ready(Ok(())) => {}
                        Poll::Ready(Err(error)) => *woe = WriterOrError::Error(error),
                        Poll::Pending => return Poll::Pending,
                    },
                    WriterOrError::Error(_) => {}
                };

                Poll::Ready(())
            },
            cx,
        )
    }
}

pub type AnySsrContext<'a> = SsrContext<std::pin::Pin<&'a mut dyn AsyncWrite>>;
