use ccss::collections::array_vec::ArrayVec;
use frender_common::const_utils::ArrayString;

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
const fn collect_info(input: &str) -> DeclarationListInfo {
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

struct ParsedDeclarationArray<'a, const N: usize>(
    [ccss::parse::declaration::Declaration<'a, ccss::collections::collect_nothing::CollectNothing>;
        N],
);

const fn parse_as_array_vec<'a, const CAP: usize>(
    input: &'a str,
) -> ArrayVec<
    ccss::parse::declaration::Declaration<ccss::collections::collect_nothing::CollectNothing>,
    CAP,
> {
    let res = ccss::parse::declaration::Declaration::<
        ccss::collections::collect_nothing::CollectNothing,
    >::parse_list_from_str(input)
    .try_collect_into_known::<ArrayVec<_, CAP>, CAP, 0>();
    let list = unwrap_declaration_list_parse_result!(res);
    *list.as_array_vec()
}

impl<'a, const N: usize> ParsedDeclarationArray<'a, N> {
    const fn from_str(input: &'a str) -> Self {
        let list = parse_as_array_vec::<N>(input);
        let list = match list.try_into_filled_array::<N>() {
            Ok(v) => v,
            Err(_) => panic!("the const generic is not exactly the parsed length"),
        };

        Self(list)
    }
}

impl<const N: usize> ParsedDeclarationArray<'static, N> {
    const fn into_array(self) -> [StaticStrDeclaration<true>; N] {
        let mut res = [DUMMY_DECLARATION; N];
        let mut i = 0;

        while i < N {
            let d = self.0[i];
            res[i] = {
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

const DUMMY_DECLARATION: StaticStrDeclaration<true> = Declaration {
    name: DeclarationName::new_const("_"),
    value: DeclarationValue::new_const(""),
    important: false,
};

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
        parsed: DeclarationArray<N>,
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
                let d = &list[i];

                assert!(
                    !d.important,
                    "AnyHasImportant<false> can't collect declaration with !important flag"
                );

                Declaration {
                    name: d.name,
                    value: d.value,
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
        parsed: DeclarationArray<N>,
    ) -> [StaticStrDeclaration<true>; N] {
        parsed.0
    }
}

pub struct DeclarationListInfo {
    pub len: usize,
    pub any_has_important: bool,
    pub prefix_semicolon_str_len: usize,
}

impl DeclarationListInfo {
    const EMPTY: Self = Self {
        len: 0,
        any_has_important: false,
        prefix_semicolon_str_len: 0,
    };
    const fn add(self, other: Self) -> Self {
        Self {
            len: self.len + other.len,
            any_has_important: self.any_has_important || other.any_has_important,
            prefix_semicolon_str_len: self.prefix_semicolon_str_len
                + other.prefix_semicolon_str_len,
        }
    }
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
                const $NAME: $crate::styles::constness::StaticStr = $const_expr;
            }
        }
    };
    (
        impl $for_ty:ty {
            const $NAME:tt: $str_ty:ty = $const_expr:expr;
        }
    ) => {
        const _: () = {
            const DECLARATION_LIST_STR: $crate::styles::constness::DeclarationListConstExpr<$str_ty> =
                $crate::styles::constness::DeclarationListConstExpr($const_expr);

            $crate::__expand_if_not_underscore! {
                $NAME
                impl $for_ty {
                    const $NAME: $str_ty = DECLARATION_LIST_STR;
                }
            }

            const DECLARATION_LIST_INFO: $crate::styles::constness::DeclarationListInfo =
                DECLARATION_LIST_STR.into_info();

            const DECLARATION_ARRAY: $crate::styles::constness::DeclarationArray<{ DECLARATION_LIST_INFO.len }> =
                DECLARATION_LIST_STR.into_array();

            impl $crate::styles::constness::HasConstDeclarationList for $for_ty {
                const DECLARATION_LIST_PREFIX_SEMICOLON: $crate::styles::constness::DeclarationListPrefixSemicolonStr<'static> =
                    DECLARATION_ARRAY.to_string_prefix_semicolon::<{DECLARATION_LIST_INFO.prefix_semicolon_str_len}>().as_str();

                type DeclarationNameStr = $crate::styles::constness::StaticStr;
                type DeclarationValueStr = $crate::styles::constness::StaticStr;
                type DeclarationImportant =
                    $crate::styles::constness::Important<{ DECLARATION_LIST_INFO.any_has_important }>;
                type DeclarationList = [$crate::styles::constness::StaticStrDeclaration<
                    { DECLARATION_LIST_INFO.any_has_important },
                >; DECLARATION_LIST_INFO.len];

                const DECLARATION_LIST: Self::DeclarationList =
                    $crate::styles::constness::AnyHasImportant::<
                        { DECLARATION_LIST_INFO.any_has_important },
                    >::map_array(DECLARATION_ARRAY);
            }
        };
    };
}

pub struct DeclarationArray<const N: usize>(pub [StaticStrDeclaration<true>; N]);

const BANG_IMPORTANT: &str = "!important";

impl<const N: usize> DeclarationArray<N> {
    pub const fn to_string_prefix_semicolon<const LEN: usize>(
        &self,
    ) -> DeclarationListStringPrefixSemicolon<LEN> {
        let mut res = ArrayString::<LEN>::new();

        let mut i = 0;

        while i < N {
            let d = &self.0[i];

            res = res
                .with_push_str(";")
                .with_push_str(d.name.unparsed())
                .with_push_str(":")
                .with_push_str(d.value.unparsed());

            if d.important {
                res = res.with_push_str(BANG_IMPORTANT);
            }

            i += 1;
        }

        let bytes = match res.try_into_filled_bytes() {
            Ok(bytes) => bytes,
            Err(_) => panic!("const generic LEN is larger than the actual length"),
        };

        DeclarationListStringPrefixSemicolon { bytes }
    }
}

pub struct DeclarationListConstExpr<T>(pub T);

impl<'a> DeclarationListConstExpr<&'a str> {
    pub const fn into_info(self) -> DeclarationListInfo {
        collect_info(self.0)
    }
}
impl DeclarationListConstExpr<&'static str> {
    pub const fn into_array<const N: usize>(self) -> DeclarationArray<N> {
        DeclarationArray(ParsedDeclarationArray::from_str(self.0).into_array())
    }
}

impl<'a> DeclarationListConstExpr<&[&'a str]> {
    pub const fn into_info(self) -> DeclarationListInfo {
        let this = self.0;
        let mut i = 0;
        let mut info = DeclarationListInfo::EMPTY;
        while i < this.len() {
            info = info.add(DeclarationListConstExpr(this[i]).into_info());
            i += 1;
        }

        info
    }
}

impl DeclarationListConstExpr<&[&'static str]> {
    pub const fn into_array<const N: usize>(self) -> DeclarationArray<N> {
        let mut res = ArrayVec::<_, N>::EMPTY;

        let mut i = 0;
        while i < self.0.len() {
            res = res.with_extend_from_slice(parse_as_array_vec::<N>(self.0[i]).as_slice());

            i += 1;
        }
        DeclarationArray(
            ParsedDeclarationArray(match res.try_into_filled_array() {
                Ok(v) => v,
                Err(_) => panic!("the const generic is not exactly the parsed length"),
            })
            .into_array(),
        )
    }
}

impl<const M: usize> DeclarationListConstExpr<[&str; M]> {
    pub const fn into_info(self) -> DeclarationListInfo {
        DeclarationListConstExpr(self.0.as_slice()).into_info()
    }
}
impl<const M: usize> DeclarationListConstExpr<[&'static str; M]> {
    pub const fn into_array<const N: usize>(self) -> DeclarationArray<N> {
        DeclarationListConstExpr(self.0.as_slice()).into_array()
    }
}

impl DeclarationListConstExpr<&[(&str, &str)]> {
    pub const fn into_info(self) -> DeclarationListInfo {
        DeclarationListInfo {
            len: self.0.len(),
            any_has_important: false,
            prefix_semicolon_str_len: {
                let mut res = 0;
                let mut i = 0;
                while i < self.0.len() {
                    let d = self.0[i];
                    res += 2 + d.0.len() + d.1.len();
                    i += 1;
                }

                res
            },
        }
    }
}

impl DeclarationListConstExpr<&[(&'static str, &'static str)]> {
    pub const fn into_array<const N: usize>(self) -> DeclarationArray<N> {
        assert!(self.0.len() == N);
        let mut res = [("", "", false); N];
        let mut i = 0;
        while i < N {
            let (name, value) = self.0[i];
            res[i] = (name, value, false);
            i += 1;
        }

        DeclarationListConstExpr(res).into_array()
    }
}

impl<const M: usize> DeclarationListConstExpr<[(&str, &str); M]> {
    pub const fn into_info(self) -> DeclarationListInfo {
        DeclarationListConstExpr(self.0.as_slice()).into_info()
    }
}

impl<const M: usize> DeclarationListConstExpr<[(&'static str, &'static str); M]> {
    pub const fn into_array<const N: usize>(self) -> DeclarationArray<N> {
        DeclarationListConstExpr(self.0.as_slice()).into_array()
    }
}

impl DeclarationListConstExpr<&[(&str, &str, frender_common::Empty)]> {
    pub const fn into_info(self) -> DeclarationListInfo {
        DeclarationListInfo {
            len: self.0.len(),
            any_has_important: false,
            prefix_semicolon_str_len: {
                let mut res = 0;
                let mut i = 0;
                while i < self.0.len() {
                    let d = self.0[i];
                    res += 2 + d.0.len() + d.1.len();
                    i += 1;
                }

                res
            },
        }
    }
}

impl DeclarationListConstExpr<&[(&'static str, &'static str, frender_common::Empty)]> {
    pub const fn into_array<const N: usize>(self) -> DeclarationArray<N> {
        assert!(self.0.len() == N);
        let mut res = [("", "", false); N];
        let mut i = 0;
        while i < N {
            let (name, value, frender_common::Empty) = self.0[i];
            res[i] = (name, value, false);
            i += 1;
        }

        DeclarationListConstExpr(res).into_array()
    }
}

impl<const M: usize> DeclarationListConstExpr<[(&str, &str, frender_common::Empty); M]> {
    pub const fn into_info(self) -> DeclarationListInfo {
        DeclarationListConstExpr(self.0.as_slice()).into_info()
    }
}

impl<const M: usize>
    DeclarationListConstExpr<[(&'static str, &'static str, frender_common::Empty); M]>
{
    pub const fn into_array<const N: usize>(self) -> DeclarationArray<N> {
        DeclarationListConstExpr(self.0.as_slice()).into_array()
    }
}

impl DeclarationListConstExpr<&[(&str, &str, bool)]> {
    pub const fn into_info(self) -> DeclarationListInfo {
        let mut any_has_important = false;
        let mut prefix_semicolon_str_len = 0;
        let mut i = 0;
        while i < self.0.len() {
            let d = self.0[i];
            prefix_semicolon_str_len += 2 + d.0.len() + d.1.len();

            if d.2 {
                any_has_important = true;

                const BANG_IMPORTANT_LEN: usize = "!important".len();
                prefix_semicolon_str_len += BANG_IMPORTANT_LEN;
            }

            i += 1;
        }

        DeclarationListInfo {
            len: self.0.len(),
            any_has_important,
            prefix_semicolon_str_len,
        }
    }
}

impl DeclarationListConstExpr<&[(&'static str, &'static str, bool)]> {
    pub const fn into_array<const N: usize>(self) -> DeclarationArray<N> {
        assert!(self.0.len() == N);
        let mut res = [DUMMY_DECLARATION; N];
        let mut i = 0;

        while i < self.0.len() {
            let (name, value, important) = self.0[i];
            res[i] = Declaration {
                name: DeclarationName::new_const(name),
                value: DeclarationValue::new_const(value),
                important,
            };
            i += 1;
        }

        DeclarationArray(res)
    }
}

impl<const M: usize> DeclarationListConstExpr<[(&str, &str, bool); M]> {
    pub const fn into_info(self) -> DeclarationListInfo {
        DeclarationListConstExpr(self.0.as_slice()).into_info()
    }
}

impl<const M: usize> DeclarationListConstExpr<[(&'static str, &'static str, bool); M]> {
    pub const fn into_array<const N: usize>(self) -> DeclarationArray<N> {
        DeclarationListConstExpr(self.0.as_slice()).into_array()
    }
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
    use std::marker::PhantomData;

    use crate::csr::{CsrStyle, CsrStyleStateUnmount};

    use super::{ConstDeclarationList, HasConstDeclarationList};

    pub struct State<T: ?Sized + HasConstDeclarationList>(PhantomData<T>);

    impl<T: ?Sized + HasConstDeclarationList> CsrStyleStateUnmount for State<T> {
        fn csr_style_state_unmount(_: &mut Self, style: &mut impl crate::csr::CssStyleDeclaration) {
            T::DECLARATION_LIST
                .into_iter()
                .for_each(|d| style.remove_property(d.name.as_ref_str()));
        }
    }

    impl<T: ?Sized + HasConstDeclarationList> CsrStyle for ConstDeclarationList<T> {
        type State = State<T>;

        fn csr_style_render_init(
            _: Self,
            style: &mut impl crate::csr::CssStyleDeclaration,
        ) -> Self::State {
            T::DECLARATION_LIST.into_iter().for_each(|d| {
                let name = d.name;
                let value = d.value;
                crate::styles::declaration::csr::update_style(
                    style,
                    name.as_ref_str(),
                    value.as_ref_str(),
                    d.important,
                );
            });
            State(PhantomData)
        }

        fn csr_style_render_update(
            _: Self,
            _: &mut impl crate::csr::CssStyleDeclaration,
            _: &mut Self::State,
        ) {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use async_str_iter::ext::AsyncStrIteratorExt;
    use futures_lite::future::block_on;

    use crate::{ssr::SsrDeclarationList, styles::constness::HasConstDeclarationList};

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
