use crate::AsyncStrIterator;

pub enum Never {}

impl AsyncStrIterator for Never {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        match *self {}
    }
}
