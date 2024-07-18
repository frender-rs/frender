use async_str_iter::IntoAsyncStrIterator;
use frender_html_common::MaybeValue;
use frender_ssr::html::assert::{self, OneStringOrEmpty};

pub struct ScriptContentNoInnerText;

impl MaybeValue<str> for ScriptContentNoInnerText {
    // whether initialized
    type UpdateWithState = bool;

    fn update_with_state(
        Self: Self,
        state: &mut Self::UpdateWithState,
        updater: impl frender_html_common::ValueUpdater<str>,
    ) {
        if !*state {
            updater.remove();
            *state = true;
        }
    }
}

pub trait IntoScriptContent {
    type IntoScriptContent: assert::ScriptContent;
    fn into_script_content(this: Self) -> Self::IntoScriptContent;

    type IntoScriptInnerText: MaybeValue<str>;
    fn into_script_inner_text(this: Self) -> Self::IntoScriptInnerText;
}

impl IntoScriptContent for crate::Empty {
    type IntoScriptContent = async_str_iter::empty::Empty;

    fn into_script_content(Self: Self) -> Self::IntoScriptContent {
        async_str_iter::empty::Empty
    }

    type IntoScriptInnerText = ScriptContentNoInnerText;

    fn into_script_inner_text(Self: Self) -> Self::IntoScriptInnerText {
        ScriptContentNoInnerText
    }
}

impl<T: IntoScriptContent> IntoScriptContent for Option<T> {
    type IntoScriptContent = async_str_iter::option::IterOption<T::IntoScriptContent>;

    fn into_script_content(this: Self) -> Self::IntoScriptContent {
        this.map(T::into_script_content).into_async_str_iterator()
    }

    type IntoScriptInnerText = Option<T::IntoScriptInnerText>;

    fn into_script_inner_text(this: Self) -> Self::IntoScriptInnerText {
        this.map(T::into_script_inner_text)
    }
}

#[cfg(feature = "either")]
impl<L: IntoScriptContent, R: IntoScriptContent> IntoScriptContent for either::Either<L, R> {
    type IntoScriptContent =
        async_str_iter::either::IterEither<L::IntoScriptContent, R::IntoScriptContent>;

    fn into_script_content(this: Self) -> Self::IntoScriptContent {
        use async_str_iter::either::IterEither;
        match this {
            either::Either::Left(this) => IterEither::Left(L::into_script_content(this)),
            either::Either::Right(this) => IterEither::Right(R::into_script_content(this)),
        }
    }

    type IntoScriptInnerText = either::Either<L::IntoScriptInnerText, R::IntoScriptInnerText>;

    fn into_script_inner_text(this: Self) -> Self::IntoScriptInnerText {
        this.map_either(L::into_script_inner_text, R::into_script_inner_text)
    }
}

pub struct ScriptInnerTextWronglyEncoded<S: MaybeValue<str>>(pub S);

impl<S: MaybeValue<str> + IntoAsyncStrIterator> IntoScriptContent
    for ScriptInnerTextWronglyEncoded<S>
where
    // multiple string chunks might be dangerous, so only one string is allowed
    S::IntoAsyncStrIterator: OneStringOrEmpty,
{
    type IntoScriptContent =
        frender_ssr::html::script::IterScriptInnerTextWronglyEncoded<S::IntoAsyncStrIterator>;

    fn into_script_content(this: Self) -> Self::IntoScriptContent {
        Self::IntoScriptContent::new(S::into_async_str_iterator(this.0))
    }

    type IntoScriptInnerText = S;

    fn into_script_inner_text(this: Self) -> Self::IntoScriptInnerText {
        this.0
    }
}
