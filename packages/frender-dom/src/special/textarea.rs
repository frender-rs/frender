use frender_ssr::html::assert::SafeTextOrEmpty;

use crate::form_control::value::FormControlValue;

pub trait SsrTextAreaValue {
    type IntoSsrTextAreaValue: SafeTextOrEmpty;

    fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue;
}

pub trait TextAreaValue: FormControlValue<str> + SsrTextAreaValue {}

impl<T: ?Sized + FormControlValue<str> + SsrTextAreaValue> TextAreaValue for T {}

mod ssr {
    use async_str_iter::any_str::IterAnyStr;

    use async_str_iter::IntoAsyncStrIterator;
    use frender_ssr::html::escape_safe::Safe;

    use frender_ssr::html::encode::Encode;

    use frender_common::{Empty, IntoStaticStr};

    use crate::known_str::KnownSsrStr;

    use super::SsrTextAreaValue;

    impl SsrTextAreaValue for Empty {
        type IntoSsrTextAreaValue = async_str_iter::empty::Empty;

        fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue {
            async_str_iter::empty::Empty
        }
    }

    impl<S: KnownSsrStr> SsrTextAreaValue for S {
        type IntoSsrTextAreaValue = Encode<Safe, IterAnyStr<S::StaticStr>>;

        fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue {
            Encode::new(
                Safe,
                IterAnyStr::new(self.into_into_static_str().into_static_str()),
            )
        }
    }

    impl<T: SsrTextAreaValue> SsrTextAreaValue for Option<T> {
        type IntoSsrTextAreaValue = async_str_iter::option::IterOption<T::IntoSsrTextAreaValue>;

        fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue {
            self.map(T::into_ssr_text_area_value)
                .into_async_str_iterator()
        }
    }

    #[cfg(feature = "either")]
    impl<L: SsrTextAreaValue, R: SsrTextAreaValue> SsrTextAreaValue for either::Either<L, R> {
        type IntoSsrTextAreaValue =
            async_str_iter::either::IterEither<L::IntoSsrTextAreaValue, R::IntoSsrTextAreaValue>;

        fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue {
            use async_str_iter::either::IterEither;
            match self {
                Self::Left(this) => IterEither::Left(this.into_ssr_text_area_value()),
                Self::Right(this) => IterEither::Right(this.into_ssr_text_area_value()),
            }
        }
    }
}
