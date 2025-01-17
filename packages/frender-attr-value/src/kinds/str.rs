mod kind {
    use crate::csr::ValueKind;

    impl ValueKind for str {
        type Value<'a> = &'a str;
    }
}

mod ssr {
    use async_str_iter::{any_str::IterAnyStr, IntoAsyncStrIterator};
    use frender_common::IntoStaticStr;
    use frender_ssr_html::attr_value::AttrEqValue;

    use crate::{ssr::SsrAttrValue, values::str::KnownSsrStr};

    impl<S: KnownSsrStr> SsrAttrValue<str> for S {
        type HtmlAttributeValue = AttrEqValue<IterAnyStr<S::StaticStr>>;

        fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
            Some(Self::HtmlAttributeValue::new(
                async_str_iter::any_str::AnyStr(this.into_into_static_str().into_static_str())
                    .into_async_str_iterator(),
            ))
        }
    }
}

mod csr {
    use frender_common::{IntoStaticStrCache, ToAsRefStr};

    use crate::{csr::CsrAttrValue, impl_csr_attr_value_with_cache, values::str::KnownCsrStr};

    impl<S: KnownCsrStr> CsrAttrValue<str> for S {
        type State = S::StaticStrCache;

        impl_csr_attr_value_with_cache!(
            //
            kind![str],
            before_set = {
                let cache = this.into_into_static_str_cache().into_static_str_cache();
            },
            set = |this| cache.to_as_ref_str().as_ref(),
            into_cache = cache,
            update_cache = |state| {
                this.into_into_static_str_cache()
                    .update_into_static_str_cache(state);

                state.to_as_ref_str().as_ref()
            },
            eq = |this, cache| this.match_static_str_cache(cache),
        );
    }
}
