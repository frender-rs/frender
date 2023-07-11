use std::task::Poll;

pub use state::{StrEncoded, StrWriteState, StrWritingState};

mod state {
    use std::{borrow::Cow, ops::RangeFrom};

    /// `None` means the str is unencoded.
    #[derive(Default)]
    pub struct StrWriteState(Option<StrWritingState>);

    impl StrWriteState {
        pub fn new() -> Self {
            Self(None)
        }

        /// `encode` should make sure:
        /// if `Cow::Borrowed(s)` is returned, then `s` must be exactly
        /// the same pointer as the input `&str`.
        pub fn encode_if_not(
            &mut self,
            encode: impl FnOnce(&str) -> Cow<str>,
            unencoded: &str,
        ) -> &mut StrWritingState {
            self.0.get_or_insert_with(|| match encode(unencoded) {
                Cow::Borrowed(s) => {
                    assert!(
                        std::ptr::eq(unencoded, s),
                        "encode fn returned borrowed str but it is not the original str"
                    );
                    StrWritingState::new(StrEncoded::UnChanged)
                }
                Cow::Owned(v) => StrWritingState::new(StrEncoded::Changed(v.into_bytes())),
            })
        }
    }

    pub struct StrWritingState {
        pub encoded: StrEncoded,
        pub unwritten: RangeFrom<usize>,
    }

    impl StrWritingState {
        fn new(encoded: StrEncoded) -> Self {
            Self {
                encoded,
                unwritten: 0..,
            }
        }
    }

    pub enum StrEncoded {
        UnChanged,
        Changed(Vec<u8>),
    }

    impl StrEncoded {
        pub fn as_bytes<'a, S: ?Sized + AsRef<[u8]>>(&'a self, original: &'a S) -> &'a [u8] {
            match self {
                StrEncoded::UnChanged => original.as_ref(),
                StrEncoded::Changed(s) => s,
            }
        }
    }
}

pub trait AsyncWriteStr {
    fn poll_write_str(
        &mut self,
        cx: &mut std::task::Context,
        s: &str,
        write_state: &mut StrWriteState,
    ) -> Poll<std::io::Result<()>>;
}

pub trait AsyncWritableStr {
    fn poll_write_str_into<W: AsyncWriteStr>(
        &mut self,
        cx: &mut std::task::Context,
        write: &mut W,
    ) -> Poll<std::io::Result<()>>;
}

pub struct StrWriting<S: std::borrow::Borrow<str>> {
    s: S,
    write_state: StrWriteState,
}

impl<S: std::borrow::Borrow<str>> AsyncWritableStr for StrWriting<S> {
    fn poll_write_str_into<W: AsyncWriteStr>(
        &mut self,
        cx: &mut std::task::Context,
        write: &mut W,
    ) -> Poll<std::io::Result<()>> {
        write.poll_write_str(cx, self.s.borrow(), &mut self.write_state)
    }
}

impl<S: std::borrow::Borrow<str>> StrWriting<S> {
    pub fn new(s: S) -> Self {
        Self {
            s,
            write_state: StrWriteState::new(),
        }
    }
}

pub trait IntoAsyncWritableStr {
    type AsyncWritableStr: AsyncWritableStr;

    fn into_async_writable_str(this: Self) -> Self::AsyncWritableStr;
}

// TODO: remove
#[cfg(aaa)]
impl<S: AsyncWritableStr> AsyncWritableStr for &mut S {
    fn poll_write_str_into<W: AsyncWriteStr>(
        &mut self,
        cx: &mut std::task::Context,
        write: &mut W,
    ) -> Poll<std::io::Result<()>> {
        S::poll_write_str_into(self, cx, write)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Empty;

impl AsyncWritableStr for Empty {
    fn poll_write_str_into<W: AsyncWriteStr>(
        &mut self,
        _: &mut std::task::Context,
        _: &mut W,
    ) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}
