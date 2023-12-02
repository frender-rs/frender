use async_str_iter::{AsyncStrIterator, IntoAsyncStrIterator};

use crate::{assert, encode::Encode};

pub trait SsrElementScriptContent {
    type ScriptContent: assert::ScriptContent;
    fn into_script_content(self) -> Self::ScriptContent;
}

impl SsrElementScriptContent for () {
    type ScriptContent = async_str_iter::empty::Empty;

    fn into_script_content(self) -> Self::ScriptContent {
        async_str_iter::empty::Empty
    }
}

impl<T: SsrElementScriptContent> SsrElementScriptContent for Option<T> {
    type ScriptContent = async_str_iter::option::IterOption<T::ScriptContent>;

    fn into_script_content(self) -> Self::ScriptContent {
        self.map(T::into_script_content).into_async_str_iterator()
    }
}

#[cfg(feature = "either")]
impl<L: SsrElementScriptContent, R: SsrElementScriptContent> SsrElementScriptContent
    for either::Either<L, R>
{
    type ScriptContent = async_str_iter::either::IterEither<L::ScriptContent, R::ScriptContent>;

    fn into_script_content(self) -> Self::ScriptContent {
        use async_str_iter::either::IterEither;
        match self {
            either::Either::Left(this) => IterEither::Left(this.into_script_content()),
            either::Either::Right(this) => IterEither::Right(this.into_script_content()),
        }
    }
}

pin_project_lite::pin_project!(
    /// Currently this might not work as expected due to the following reasons:
    ///
    ///  - [`html_escape::encode_script`] is not perfect, see [`crate::escape_safe::Script`].
    ///  - `S: AsyncStrIterator` is not encoded if it yields more than one string chunks
    ///    and `</script>` is split into multiple chunks.
    pub struct IterScriptInnerTextWronglyEncoded<S: AsyncStrIterator> {
        #[pin]
        encode: Encode<crate::escape_safe::Script, S>,
    }
);

impl<S: AsyncStrIterator> AsyncStrIterator for IterScriptInnerTextWronglyEncoded<S> {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        self.project().encode.poll_next_str(cx)
    }
}

impl<S: AsyncStrIterator> IterScriptInnerTextWronglyEncoded<S> {
    pub fn new(s: S) -> Self {
        Self {
            encode: Encode::new(crate::escape_safe::Script, s),
        }
    }
}
