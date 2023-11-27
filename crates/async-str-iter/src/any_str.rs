use crate::{AsyncStrIterator, IntoAsyncStrIterator};

pub struct IterAnyStr<S: AsRef<str>> {
    s: S,
    taken: bool,
}

impl<S: AsRef<str>> Unpin for IterAnyStr<S> {}

impl<S: AsRef<str>> AsyncStrIterator for IterAnyStr<S> {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        let this = self.get_mut();
        std::task::Poll::Ready(if this.taken {
            None
        } else {
            this.taken = true;
            Some(this.s.as_ref())
        })
    }
}

pub struct AnyStr<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> IntoAsyncStrIterator for AnyStr<S> {
    type IntoAsyncStrIterator = IterAnyStr<S>;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        IterAnyStr {
            s: self.0,
            taken: false,
        }
    }
}
