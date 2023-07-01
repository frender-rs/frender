use std::borrow::Cow;

use crate::dom_token::DomTokenList;

pub trait HtmlAttributeClass {
    type UpdateAttributeState;

    fn update_attribute(
        this: Self,
        state: &mut Self::UpdateAttributeState,
        class_list: impl DomTokenList,
    );

    type StringChunks: Iterator<Item = Cow<'static, str>>;

    fn into_string_chunks(this: Self) -> Self::StringChunks;
}

macro_rules! impl_for_many {
    ( impl $trait_path:ident for EachOf![$($ty:ty),+] ) => {};
}

/// A single string is considered as the whole `class` attribute.
mod impl_for_str {
    use super::*;

    impl HtmlAttributeClass for &str {
        type UpdateAttributeState = ();

        fn update_attribute(
            this: Self,
            (): &mut Self::UpdateAttributeState,
            class_list: impl DomTokenList,
        ) {
            class_list.set_value(this)
        }

        type StringChunks = core::iter::Once<Cow<'static, str>>;

        fn into_string_chunks(this: Self) -> Self::StringChunks {
            core::iter::once(this.to_owned().into())
        }
    }

    impl HtmlAttributeClass for String {
        type UpdateAttributeState = String;

        fn update_attribute(
            this: Self,
            old_value: &mut Self::UpdateAttributeState,
            class_list: impl DomTokenList,
        ) {
            if *old_value != this {
                class_list.set_value(&this);
                *old_value = this;
            }
        }

        type StringChunks = core::iter::Once<Cow<'static, str>>;

        fn into_string_chunks(this: Self) -> Self::StringChunks {
            core::iter::once(this.into())
        }
    }

    impl HtmlAttributeClass for Cow<'_, str> {
        type UpdateAttributeState = Option<String>;

        fn update_attribute(
            this: Self,
            old_value: &mut Self::UpdateAttributeState,
            class_list: impl DomTokenList,
        ) {
            if old_value.as_deref() != Some(&this) {
                class_list.set_value(&this);
                *old_value = match this {
                    Cow::Borrowed(_) => None,
                    Cow::Owned(s) => Some(s),
                };
            }
        }

        type StringChunks = core::iter::Once<Cow<'static, str>>;

        fn into_string_chunks(this: Self) -> Self::StringChunks {
            core::iter::once(this.into_owned().into())
        }
    }

    // TODO: impl HtmlAttributeClass for StaticText
}
