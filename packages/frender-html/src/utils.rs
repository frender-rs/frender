use std::task::Poll;

pub(crate) fn poll_each(mut iter: impl Iterator<Item = Poll<()>>) -> Poll<()> {
    let mut out = Poll::Ready(());

    for p in iter.by_ref() {
        if let Poll::Pending = p {
            out = Poll::Pending
        }
    }

    out
}
