use ccss::collections::array_vec::ArrayVec;

use crate::declaration::{
    important::IntoDeclarationImportant, Declaration, DeclarationName, DeclarationValue,
};

/// Note that implementations might have different values for csr and ssr.
pub trait HasConstDeclarationList {
    /// This is used for ssr.
    const DECLARATION_LIST_PREFIX_SEMICOLON: DeclarationListPrefixSemicolonStr<'static>;

    type DeclarationNameStr: 'static + AsRef<str> + Clone + PartialEq;
    type DeclarationValueStr: 'static + AsRef<str> + Clone + PartialEq;
    type DeclarationImportant: IntoDeclarationImportant;

    /// This is used for csr.
    type DeclarationList: AsRef<
            [Declaration<
                DeclarationName<Self::DeclarationNameStr>,
                DeclarationValue<Self::DeclarationValueStr>,
                Self::DeclarationImportant,
            >],
        > + IntoIterator<
            Item = Declaration<
                DeclarationName<Self::DeclarationNameStr>,
                DeclarationValue<Self::DeclarationValueStr>,
                Self::DeclarationImportant,
            >,
        >;
    const DECLARATION_LIST: Self::DeclarationList;
}

macro_rules! unwrap_or_else {
    ($e:expr, |$err:pat_param| $else:expr) => {
        match $e {
            Ok(v) => v,
            Err($err) => $else,
        }
    };
}

macro_rules! unwrap_declaration_list_parse_result {
    ($res:expr) => {
        unwrap_or_else! {
            $res, |_| panic!("invalid style declarations")
        }
    };
}

pub struct AnyHasImportant<const YES: bool>;

pub trait HasImportantType {
    type Important: IntoDeclarationImportant;
}

impl HasImportantType for AnyHasImportant<true> {
    type Important = bool;
}

impl HasImportantType for AnyHasImportant<false> {
    type Important = frender_common::Empty;
}

pub type Important<const ANY_HAS_IMPORTANT: bool> =
    <AnyHasImportant<ANY_HAS_IMPORTANT> as HasImportantType>::Important;

/// Returns false for empty declaration list.
pub const fn collect_info(input: &'static str) -> DeclarationListInfo {
    let mut parse_list = ccss::parse::declaration::Declaration::<
        ccss::collections::collect_nothing::CollectNothing,
    >::parse_list_from_str(input);

    let mut len = 0;
    let mut any_has_important = false;

    let mut prefix_semicolon_str_len = 0;

    loop {
        let d;
        (d, parse_list) = unwrap_declaration_list_parse_result!(parse_list.try_into_next());
        if let Some(d) = d {
            len += 1;
            // ;name:value
            prefix_semicolon_str_len +=
                2 + d.name_as_original_str().len() + d.value_as_original_str().len();
            if d.is_important() {
                any_has_important = true;

                const BANG_IMPORTANT_LEN: usize = "!important".len();
                prefix_semicolon_str_len += BANG_IMPORTANT_LEN;
            }
        } else {
            break;
        }
    }

    DeclarationListInfo {
        len,
        any_has_important,
        prefix_semicolon_str_len,
    }
}

pub type StaticStrDeclaration<const ANY_HAS_IMPORTANT: bool> = Declaration<
    DeclarationName<&'static str>,
    DeclarationValue<&'static str>,
    Important<{ ANY_HAS_IMPORTANT }>,
>;

pub struct ParsedDeclarationArray<'a, const N: usize>(
    [ccss::parse::declaration::Declaration<'a, ccss::collections::collect_nothing::CollectNothing>;
        N],
);

impl<'a, const N: usize> ParsedDeclarationArray<'a, N> {
    pub const fn from_str(input: &'a str) -> Self {
        let res = ccss::parse::declaration::Declaration::<
            ccss::collections::collect_nothing::CollectNothing,
        >::parse_list_from_str(input)
        .try_collect_into_known::<ArrayVec<_, N>, N, 0>();

        let list = unwrap_declaration_list_parse_result!(res);

        let list = match list.as_array_vec().try_into_filled_array::<N>() {
            Ok(v) => v,
            Err(_) => panic!("the const generic is not exactly the parsed length"),
        };

        Self(list)
    }

    pub const fn to_string_prefix_semicolon<const LEN: usize>(
        &self,
    ) -> DeclarationListStringPrefixSemicolon<LEN> {
        let mut res = [0; LEN];
        let mut cur = 0;

        let mut i = 0;

        while i < N {
            let d = self.0[i];

            {
                res[cur] = b';';
                cur += 1;
            }

            {
                let v = d.name_as_original_str().as_bytes();
                let mut j = 0;
                while j < v.len() {
                    res[cur] = v[j];
                    j += 1;
                    cur += 1;
                }
            }

            {
                res[cur] = b':';
                cur += 1;
            }

            {
                let v = d.value_as_original_str().as_bytes();
                let mut j = 0;
                while j < v.len() {
                    res[cur] = v[j];
                    j += 1;
                    cur += 1;
                }
            }

            if d.is_important() {
                let v = "!important".as_bytes();
                let mut j = 0;
                while j < v.len() {
                    res[cur] = v[j];
                    j += 1;
                    cur += 1;
                }
            }

            i += 1;
        }

        assert!(cur == LEN);

        DeclarationListStringPrefixSemicolon { bytes: res }
    }
}

pub struct DeclarationListStringPrefixSemicolon<const N: usize> {
    bytes: [u8; N],
}

impl<const N: usize> DeclarationListStringPrefixSemicolon<N> {
    pub const fn as_str(&self) -> DeclarationListPrefixSemicolonStr {
        DeclarationListPrefixSemicolonStr(match core::str::from_utf8(&self.bytes) {
            Ok(v) => v,
            Err(_) => panic!("invalid utf8"),
        })
    }
}

#[derive(Clone, Copy)]
pub struct DeclarationListPrefixSemicolonStr<'a>(&'a str);

impl<'a> DeclarationListPrefixSemicolonStr<'a> {
    pub const fn to_str(self) -> &'a str {
        self.0
    }
    pub fn to_str_without_prefix_semicolon(self) -> &'a str {
        let s = self.0;
        if s.is_empty() {
            s
        } else {
            s.split_at(1).1
        }
    }
}

impl AnyHasImportant<false> {
    pub const fn map_array<const N: usize>(
        parsed: ParsedDeclarationArray<'static, N>,
    ) -> [StaticStrDeclaration<false>; N] {
        let list = parsed.0;

        const DUMMY: StaticStrDeclaration<false> = Declaration {
            name: DeclarationName::new_const("_"),
            value: DeclarationValue::new_const(""),
            important: frender_common::Empty,
        };
        let mut res = [DUMMY; N];

        let mut i = 0;
        while i < N {
            res[i] = {
                let d = list[i];

                assert!(
                    !d.is_important(),
                    "AnyHasImportant<false> can't collect declaration with !important flag"
                );

                Declaration {
                    name: DeclarationName::from_parsed(d.name()),
                    value: DeclarationValue::from_parsed(d.value()),
                    important: frender_common::Empty,
                }
            };

            i += 1;
        }

        res
    }
}

impl AnyHasImportant<true> {
    pub const fn map_array<const N: usize>(
        parsed: ParsedDeclarationArray<'static, N>,
    ) -> [StaticStrDeclaration<true>; N] {
        let list = parsed.0;

        const DUMMY: StaticStrDeclaration<true> = Declaration {
            name: DeclarationName::new_const("_"),
            value: DeclarationValue::new_const(""),
            important: false,
        };
        let mut res = [DUMMY; N];

        let mut i = 0;
        while i < N {
            res[i] = {
                let d = list[i];

                Declaration {
                    name: DeclarationName::from_parsed(d.name()),
                    value: DeclarationValue::from_parsed(d.value()),
                    important: d.is_important(),
                }
            };

            i += 1;
        }

        res
    }
}

pub struct DeclarationListInfo {
    pub len: usize,
    pub any_has_important: bool,
    pub prefix_semicolon_str_len: usize,
}

#[macro_export]
macro_rules! impl_has_const_declaration_list_for {
    (impl <$(__)?> $($rest:tt)*) => {
        $crate::impl_has_const_declaration_list_for! {
            impl $($rest)*
        }
    };
    (
        impl $for_ty:ty {
            const $NAME:tt: _ = $const_expr:expr;
        }
    ) => {
        $crate::impl_has_const_declaration_list_for! {
            impl $for_ty {
                const $NAME: $crate::constness::StaticStr = $const_expr;
            }
        }
    };
    (
        impl $for_ty:ty {
            const $NAME:tt: $str_ty:ty = $const_expr:expr;
        }
    ) => {
        const _: () = {
            const DECLARATION_LIST_STR: $str_ty = $const_expr;

            $crate::__expand_if_not_underscore! {
                $NAME
                impl $for_ty {
                    const $NAME: $str_ty = DECLARATION_LIST_STR;
                }
            }

            const DECLARATION_LIST_INFO: $crate::constness::DeclarationListInfo =
                $crate::constness::collect_info(DECLARATION_LIST_STR);

            const PARSED_DECLARATION_ARRAY: $crate::constness::ParsedDeclarationArray<{DECLARATION_LIST_INFO.len}> =
                $crate::constness::ParsedDeclarationArray::from_str(DECLARATION_LIST_STR);

            impl $crate::constness::HasConstDeclarationList for $for_ty {
                const DECLARATION_LIST_PREFIX_SEMICOLON: $crate::constness::DeclarationListPrefixSemicolonStr<'static> =
                    PARSED_DECLARATION_ARRAY.to_string_prefix_semicolon::<{DECLARATION_LIST_INFO.prefix_semicolon_str_len}>().as_str();

                type DeclarationNameStr = $str_ty;
                type DeclarationValueStr = $str_ty;
                type DeclarationImportant =
                    $crate::constness::Important<{ DECLARATION_LIST_INFO.any_has_important }>;
                type DeclarationList = [$crate::constness::StaticStrDeclaration<
                    { DECLARATION_LIST_INFO.any_has_important },
                >; DECLARATION_LIST_INFO.len];

                const DECLARATION_LIST: Self::DeclarationList =
                    $crate::constness::AnyHasImportant::<
                        { DECLARATION_LIST_INFO.any_has_important },
                    >::map_array(PARSED_DECLARATION_ARRAY);
            }
        };
    };
}

pub type StaticStr = &'static str;

#[doc(hidden)]
pub mod __private {

    pub use str;

    pub const fn from_utf8(v: &[u8]) -> &str {
        match core::str::from_utf8(v) {
            Ok(v) => v,
            Err(_) => panic!("invalid utf8"),
        }
    }
}

frender_macro_rules::define_phantom_wrapper!(
    #[always_derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ConstDeclarationList<T: ?Sized + HasConstDeclarationList>;
);

pub mod ssr {
    use std::{marker::PhantomData, task::Poll};

    use async_str_iter::AsyncStrIterator;

    use crate::ssr::{SsrDeclarationList, SsrStyle};

    use super::{ConstDeclarationList, HasConstDeclarationList};

    pub struct ConstDeclarationListIntoSsr<T: ?Sized + HasConstDeclarationList> {
        yielded: bool,
        __: PhantomData<T>,
    }

    impl<T: ?Sized + HasConstDeclarationList> Unpin for ConstDeclarationListIntoSsr<T> {}

    impl<T: ?Sized + HasConstDeclarationList> AsyncStrIterator for ConstDeclarationListIntoSsr<T> {
        fn poll_next_str(
            self: std::pin::Pin<&mut Self>,
            _: &mut std::task::Context<'_>,
        ) -> Poll<Option<&str>> {
            let this = self.get_mut();
            if this.yielded {
                Poll::Ready(None)
            } else {
                this.yielded = true;
                Poll::Ready(Some(
                    T::DECLARATION_LIST_PREFIX_SEMICOLON.to_str_without_prefix_semicolon(),
                ))
            }
        }
    }

    pub struct ConstDeclarationListIntoSsrPrefixSemicolon<T: ?Sized + HasConstDeclarationList> {
        yielded: bool,
        __: PhantomData<T>,
    }

    impl<T: ?Sized + HasConstDeclarationList> Unpin for ConstDeclarationListIntoSsrPrefixSemicolon<T> {}
    impl<T: ?Sized + HasConstDeclarationList> AsyncStrIterator
        for ConstDeclarationListIntoSsrPrefixSemicolon<T>
    {
        fn poll_next_str(
            self: std::pin::Pin<&mut Self>,
            _: &mut std::task::Context<'_>,
        ) -> Poll<Option<&str>> {
            // TODO: ASSERT match csr
            let this = self.get_mut();
            if this.yielded {
                Poll::Ready(None)
            } else {
                this.yielded = true;
                Poll::Ready(Some(T::DECLARATION_LIST_PREFIX_SEMICOLON.to_str()))
            }
        }
    }

    impl<T: ?Sized + HasConstDeclarationList> SsrDeclarationList for ConstDeclarationList<T> {
        type IntoDeclarationList = ConstDeclarationListIntoSsr<T>;

        type IntoDeclarationListPrefixSemicolon = ConstDeclarationListIntoSsrPrefixSemicolon<T>;

        fn into_declaration_list(_: Self) -> Self::IntoDeclarationList {
            ConstDeclarationListIntoSsr {
                yielded: false,
                __: PhantomData,
            }
        }

        fn into_declaration_list_prefix_semicolon(
            _: Self,
        ) -> Self::IntoDeclarationListPrefixSemicolon {
            ConstDeclarationListIntoSsrPrefixSemicolon {
                yielded: false,
                __: PhantomData,
            }
        }
    }

    impl<T: ?Sized + HasConstDeclarationList> SsrStyle for ConstDeclarationList<T> {
        type IntoSsrDeclarationList = Self;

        fn into_ssr_declaration_list(this: Self) -> Self::IntoSsrDeclarationList {
            this
        }
    }
}

pub mod csr {
    use crate::csr::CsrStyle;

    use super::{ConstDeclarationList, HasConstDeclarationList};

    impl<T: ?Sized + HasConstDeclarationList> CsrStyle for ConstDeclarationList<T> {
        type UpdateWithState = bool; // whether updated

        fn update_with_state(
            _: Self,
            state: &mut Self::UpdateWithState,
            style: &mut impl crate::csr::CssStyleDeclaration,
        ) {
            if *state {
                return;
            }
            *state = true;

            T::DECLARATION_LIST.into_iter().for_each(|d| {
                let name = d.name;
                let value = d.value;
                crate::csr::update_style(style, name.as_ref_str(), value.as_ref_str(), d.important);
            })
        }

        fn remove_with_state(
            state: &mut Self::UpdateWithState,
            style: &mut impl crate::csr::CssStyleDeclaration,
        ) {
            if !*state {
                return;
            }
            *state = false;
            T::DECLARATION_LIST
                .as_ref()
                .iter()
                .for_each(|d| style.remove_property(d.name.as_ref_str()));
        }
    }
}

#[cfg(test)]
mod tests {
    use async_str_iter::ext::AsyncStrIteratorExt;
    use futures_lite::future::block_on;

    use crate::{constness::HasConstDeclarationList, ssr::SsrDeclarationList};

    enum Demo {}

    impl_has_const_declaration_list_for!(
        impl<__> Demo {
            const _: _ = r"
                font-size: large;
                animation: 3s infinite alternate slidein;
            ";
        }
    );

    #[test]
    fn test() {
        let v = super::ConstDeclarationList::<Demo>();

        assert_eq!(
            Demo::DECLARATION_LIST_PREFIX_SEMICOLON.to_str(),
            ";font-size:large;animation:3s infinite alternate slidein"
        );

        assert_eq!(
            Demo::DECLARATION_LIST_PREFIX_SEMICOLON.to_str_without_prefix_semicolon(),
            "font-size:large;animation:3s infinite alternate slidein"
        );

        {
            let s = block_on(SsrDeclarationList::into_declaration_list(v).collect::<String>());
            assert_eq!(
                s,
                Demo::DECLARATION_LIST_PREFIX_SEMICOLON.to_str_without_prefix_semicolon()
            )
        }

        {
            let s = block_on(
                SsrDeclarationList::into_declaration_list_prefix_semicolon(v).collect::<String>(),
            );
            assert_eq!(s, Demo::DECLARATION_LIST_PREFIX_SEMICOLON.to_str())
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expand_if_not_underscore {
    (_     $($rest:tt)*) => {};
    ($t:tt $($rest:tt)*) => {
        $($rest)*
    };
}
