mod ssr {
    use async_str_iter::{any_str::IterAnyStr, IntoAsyncStrIterator};
    use frender_common::ToStaticStr;
    use frender_ssr_html::attr_value::AttrEqValue;

    use crate::{ssr::MaybeIntoHtmlAttributeValue, string::KnownStaticStr};

    impl<S: KnownStaticStr> MaybeIntoHtmlAttributeValue<str> for S {
        type HtmlAttributeValue = AttrEqValue<IterAnyStr<S>>;

        fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
            Some(Self::HtmlAttributeValue::new(
                async_str_iter::any_str::AnyStr(this).into_async_str_iterator(),
            ))
        }
    }

    impl<S: ToStaticStr> MaybeIntoHtmlAttributeValue<str> for frender_common::TempStr<S> {
        type HtmlAttributeValue = AttrEqValue<IterAnyStr<S::StaticStr>>;

        fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
            Some(AttrEqValue::new(IterAnyStr::new(this.0.into_static_str())))
        }
    }
}

mod csr {
    use frender_common::{ToAsRefStr, ToStaticCache};

    use crate::{
        csr::{MaybeValue, ValueUpdater},
        string::KnownStaticStr,
    };

    impl<S: KnownStaticStr> MaybeValue<str> for S {
        type UpdateWithState = Option<S>;

        fn update_with_state(
            this: Self,
            state: &mut Self::UpdateWithState,
            updater: impl ValueUpdater<str>,
        ) {
            if let Some(state) = state {
                if *state == this {
                    return;
                }
            }

            updater.update(this.as_ref());
            *state = Some(this);
        }

        fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool {
            state.is_none()
        }
    }

    impl<S: ToStaticCache + ToAsRefStr> MaybeValue<str> for frender_common::TempStr<S> {
        type UpdateWithState = Option<S::StaticCache>;

        fn update_with_state(
            Self(this): Self,
            cache: &mut Self::UpdateWithState,
            updater: impl ValueUpdater<str>,
        ) {
            if let Some(cache) = cache {
                if this.match_cache(cache) {
                    return;
                }
            }

            updater.update(this.to_as_ref_str().as_ref());

            if let Some(cache) = cache {
                this.update_into_static_cache(cache);
            } else {
                *cache = Some(this.into_static_cache());
            }
        }

        fn state_could_skip_remove(state: &Self::UpdateWithState) -> bool {
            state.is_none()
        }
    }
}
