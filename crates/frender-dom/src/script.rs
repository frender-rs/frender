use async_str_iter::IntoAsyncStrIterator;
use frender_html_common::maybe_str::MaybeStr;
use frender_ssr::html::assert;

pub trait SsrElementScriptContent {
    type ScriptContent: assert::ScriptContent;
    fn into_script_content(this: Self) -> Self::ScriptContent;

    type MaybeStr: MaybeStr;

    fn into_maybe_str(this: Self) -> Self::MaybeStr;
}

impl SsrElementScriptContent for () {
    type ScriptContent = async_str_iter::empty::Empty;

    fn into_script_content((): Self) -> Self::ScriptContent {
        async_str_iter::empty::Empty
    }

    type MaybeStr = Self;

    fn into_maybe_str(this: Self) -> Self::MaybeStr {
        this
    }
}

impl<T: SsrElementScriptContent> SsrElementScriptContent for Option<T> {
    type ScriptContent = async_str_iter::option::IterOption<T::ScriptContent>;

    fn into_script_content(this: Self) -> Self::ScriptContent {
        this.map(T::into_script_content).into_async_str_iterator()
    }

    type MaybeStr = Option<T::MaybeStr>;

    fn into_maybe_str(this: Self) -> Self::MaybeStr {
        this.map(T::into_maybe_str)
    }
}

#[cfg(feature = "either")]
impl<L: SsrElementScriptContent, R: SsrElementScriptContent> SsrElementScriptContent
    for either::Either<L, R>
{
    type ScriptContent = async_str_iter::either::IterEither<L::ScriptContent, R::ScriptContent>;

    fn into_script_content(this: Self) -> Self::ScriptContent {
        use async_str_iter::either::IterEither;
        match this {
            either::Either::Left(this) => IterEither::Left(L::into_script_content(this)),
            either::Either::Right(this) => IterEither::Right(R::into_script_content(this)),
        }
    }

    type MaybeStr = either::Either<L::MaybeStr, R::MaybeStr>;

    fn into_maybe_str(this: Self) -> Self::MaybeStr {
        this.map_either(L::into_maybe_str, R::into_maybe_str)
    }
}

pub struct ScriptInnerTextWronglyEncoded<S: MaybeStr>(pub S);

impl<S: MaybeStr> SsrElementScriptContent for ScriptInnerTextWronglyEncoded<S> {
    type ScriptContent =
        frender_ssr::html::script::IterScriptInnerTextWronglyEncoded<S::OneStringOrEmpty>;

    fn into_script_content(this: Self) -> Self::ScriptContent {
        Self::ScriptContent::new(S::into_one_string_or_empty(this.0))
    }

    type MaybeStr = S;

    fn into_maybe_str(this: Self) -> Self::MaybeStr {
        this.0
    }
}
