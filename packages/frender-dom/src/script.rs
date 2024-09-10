use async_str_iter::{any_str::IterAnyStr, IntoAsyncStrIterator};
use frender_attr_value::csr::CsrAttrValue;
use frender_common::{
    strings::{CsrStr, SsrStr},
    IntoStaticStr, IntoStaticStrCache, ToAsRefStr,
};
use frender_ssr::html::assert;

pub struct ScriptContentNoInnerText;

impl CsrAttrValue<str> for ScriptContentNoInnerText {
    type State = ();

    fn update_absent_attribute_value_into_state(
        Self: Self,
        _: impl frender_attr_value::csr::UpdateAttrValue<Kind = str>,
    ) -> Self::State {
    }

    fn update_attribute_value_into_state(
        Self: Self,
        updater: impl frender_attr_value::csr::UpdateAttrValue<Kind = str>,
    ) -> Self::State {
        updater.remove()
    }

    fn can_skip_update(Self: &Self, (): &Self::State) -> bool {
        true
    }

    fn update_attribute_value_with_state(
        Self: Self,
        _: impl frender_attr_value::csr::UpdateAttrValue<Kind = str>,
        (): &mut Self::State,
    ) {
    }

    fn force_update_attribute_value_with_state(
        this: Self,
        updater: impl frender_attr_value::csr::UpdateAttrValue<Kind = str>,
        (): &mut Self::State,
    ) {
        Self::update_attribute_value_into_state(this, updater)
    }

    fn attribute_is_known_as_absent((): &Self::State) -> bool {
        true
    }
}

pub trait IntoScriptContent {
    type IntoScriptContent: assert::ScriptContent;
    fn into_script_content(this: Self) -> Self::IntoScriptContent;

    type IntoScriptInnerText: CsrAttrValue<str>;
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

pub struct ScriptInnerTextWronglyEncoded<S: SsrStr + CsrStr>(pub S);

impl<S: SsrStr + CsrStr> CsrAttrValue<str> for ScriptInnerTextWronglyEncoded<S> {
    type State = S::StaticStrCache;

    frender_attr_value::impl_csr_attr_value_with_cache!(
        kind![str],
        before_set = {
            let cache = this.0.into_into_static_str_cache().into_static_str_cache();
        },
        set = |this| cache.to_as_ref_str().as_ref(),
        into_cache = cache,
        eq = |this, cache| *cache == this.0,
    );
}

impl<S: SsrStr + CsrStr> IntoScriptContent for ScriptInnerTextWronglyEncoded<S> {
    type IntoScriptContent =
        frender_ssr::html::script::IterScriptInnerTextWronglyEncoded<IterAnyStr<S::StaticStr>>;

    fn into_script_content(this: Self) -> Self::IntoScriptContent {
        Self::IntoScriptContent::new(IterAnyStr::new(
            this.0.into_into_static_str().into_static_str(),
        ))
    }

    type IntoScriptInnerText = Self;

    fn into_script_inner_text(this: Self) -> Self::IntoScriptInnerText {
        this
    }
}
