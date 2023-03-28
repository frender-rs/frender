use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

pub trait AsyncWritableBytes: AsRef<[u8]> {
    fn truncate_start_at(&mut self, n: usize);

    fn poll_write_bytes<W: crate::AsyncWrite + ?Sized>(
        &mut self,
        mut writer: Pin<&mut W>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        while !self.as_ref().is_empty() {
            let n = match writer.as_mut().poll_write(cx, self.as_ref()) {
                Poll::Ready(n) => n?,
                Poll::Pending => return Poll::Pending,
            };

            self.truncate_start_at(n);

            if n == 0 {
                return Poll::Ready(Err(io::ErrorKind::WriteZero.into()));
            }
        }

        Poll::Ready(Ok(()))
    }
}

impl AsyncWritableBytes for &[u8] {
    fn truncate_start_at(&mut self, n: usize) {
        let buf = std::mem::take(self);
        let (_, rest) = buf.split_at(n);
        *self = rest;
    }
}

pub trait AsyncWritableByteChunks {
    type Chunk: AsyncWritableBytes;
    fn as_mut_current_chunk(&mut self) -> Option<&mut Self::Chunk>;
    fn go_to_next_chunk(&mut self);

    fn poll_write_byte_chunks<W: crate::AsyncWrite + ?Sized>(
        &mut self,
        mut writer: Pin<&mut W>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        while let Some(current_chunk) = self.as_mut_current_chunk() {
            match current_chunk.poll_write_bytes(writer.as_mut(), cx) {
                Poll::Ready(Ok(_)) => self.go_to_next_chunk(),
                res => return res,
            }
        }

        Poll::Ready(Ok(()))
    }
}
