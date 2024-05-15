use async_str_iter::IntoAsyncStrIterator;
use frender_html_common::MaybeValue;
use frender_ssr::html::assert::{self, OneStringOrEmpty};

pub trait SsrElementScriptContent: MaybeValue<str> {
    type ScriptContent: assert::ScriptContent;
    fn into_script_content(this: Self) -> Self::ScriptContent;
}

impl SsrElementScriptContent for () {
    type ScriptContent = async_str_iter::empty::Empty;

    fn into_script_content((): Self) -> Self::ScriptContent {
        async_str_iter::empty::Empty
    }
}

impl<T: SsrElementScriptContent> SsrElementScriptContent for Option<T> {
    type ScriptContent = async_str_iter::option::IterOption<T::ScriptContent>;

    fn into_script_content(this: Self) -> Self::ScriptContent {
        this.map(T::into_script_content).into_async_str_iterator()
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
}

pub struct ScriptInnerTextWronglyEncoded<S: MaybeValue<str>>(pub S);

impl<S: MaybeValue<str>> MaybeValue<str> for ScriptInnerTextWronglyEncoded<S> {
    type UpdateWithState = <S as MaybeValue<str>>::UpdateWithState;

    fn update_with_state(
        Self(this): Self,
        state: &mut Self::UpdateWithState,
        updater: impl frender_html_common::ValueUpdater<str>,
    ) {
        S::update_with_state(this, state, updater)
    }
}

impl<S: MaybeValue<str> + IntoAsyncStrIterator> SsrElementScriptContent
    for ScriptInnerTextWronglyEncoded<S>
where
    // multiple string chunks might be dangerous, so only one string is allowed
    S::IntoAsyncStrIterator: OneStringOrEmpty,
{
    type ScriptContent =
        frender_ssr::html::script::IterScriptInnerTextWronglyEncoded<S::IntoAsyncStrIterator>;

    fn into_script_content(this: Self) -> Self::ScriptContent {
        Self::ScriptContent::new(S::into_async_str_iterator(this.0))
    }
}
