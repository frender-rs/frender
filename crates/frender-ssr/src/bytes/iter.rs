use super::{AsyncWritableByteChunks, AsyncWritableBytes};

pub struct IterByteChunks<I: Iterator> {
    current_chunk: Option<I::Item>,
    iterator: I,
}

impl<I: Iterator> IterByteChunks<I> {
    pub fn new(mut iterator: I) -> Self {
        Self {
            current_chunk: iterator.next(),
            iterator,
        }
    }
}

impl<I: Iterator> AsyncWritableByteChunks for IterByteChunks<I>
where
    <I as Iterator>::Item: AsyncWritableBytes,
{
    type Chunk = I::Item;

    fn as_mut_current_chunk(&mut self) -> Option<&mut Self::Chunk> {
        self.current_chunk.as_mut()
    }

    fn go_to_next_chunk(&mut self) {
        self.current_chunk = self.iterator.next();
    }
}
