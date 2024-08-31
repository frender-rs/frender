pub use frender_const::ConstUsize;

use frender_common::const_utils::put_at;

use crate::{
    dom_token::UniqueDomTokenArrayVec, ChainableDomTokens, DomToken, DomTokens, UniqueDomTokenArray,
};

mod sealed {
    pub trait IsUniqueDomTokenArray<'a> {}
    pub trait IsUniqueDomTokenArrayVec<'a> {}
}

pub trait IsConstUsize: frender_const::IsConstUsize {
    type UniqueDomTokenArray<'a>: IsUniqueDomTokenArray<'a, Len = Self>;
    type UniqueDomTokenArrayVec<'a>: IsUniqueDomTokenArrayVec<'a, Cap = Self>;
}

pub trait IsUniqueDomTokenArray<'a>:
    sealed::IsUniqueDomTokenArray<'a>
    + std::ops::Deref<Target = <Self::Len as frender_const::IsConstUsize>::Array<DomToken<'a>>>
{
    type Len: IsConstUsize;
}

pub trait IsUniqueDomTokenArrayVec<'a>:
    sealed::IsUniqueDomTokenArrayVec<'a>
    + std::fmt::Debug
    + Copy
    + AsRef<[DomToken<'a>]>
    + TryInto<<Self::Cap as IsConstUsize>::UniqueDomTokenArray<'a>, Error = Self>
{
    type Cap: IsConstUsize;
}

impl<const N: usize> IsConstUsize for ConstUsize<N> {
    type UniqueDomTokenArray<'a> = UniqueDomTokenArray<'a, N>;
    type UniqueDomTokenArrayVec<'a> = UniqueDomTokenArrayVec<'a, N>;
}

impl<'a, const N: usize> sealed::IsUniqueDomTokenArray<'a> for UniqueDomTokenArray<'a, N> {}
impl<'a, const N: usize> IsUniqueDomTokenArray<'a> for UniqueDomTokenArray<'a, N> {
    type Len = ConstUsize<N>;
}

impl<'a, const CAP: usize> sealed::IsUniqueDomTokenArrayVec<'a>
    for UniqueDomTokenArrayVec<'a, CAP>
{
}
impl<'a, const CAP: usize> IsUniqueDomTokenArrayVec<'a> for UniqueDomTokenArrayVec<'a, CAP> {
    type Cap = ConstUsize<CAP>;
}

pub trait HasConstKnownPossibleDomTokens {
    type KnownPossibleDomTokensArrayVecCap: IsConstUsize;
    const KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC:
        <Self::KnownPossibleDomTokensArrayVecCap as IsConstUsize>::UniqueDomTokenArrayVec<'static>;
}

/// A string which satisfies [`crate::ssr::asserts::DomTokensPrefixSpace`].
#[derive(Debug, Clone, Copy)]
pub struct DomTokensPrefixSpaceString<const N: usize> {
    inner: [u8; N],
}

impl<const N: usize> DomTokensPrefixSpaceString<N> {
    pub const fn from_dom_tokens(dom_tokens: &[DomToken]) -> Self {
        Self {
            inner: {
                let mut res = [0; N];
                let mut at = 0;

                let mut i = 0;

                while i < dom_tokens.len() {
                    {
                        // space
                        (res, at) = put_at(res, at, " ".as_bytes());

                        let t = dom_tokens[i];
                        let t = t.as_str().as_bytes();
                        // dom token
                        (res, at) = put_at(res, at, t);
                    }
                    i += 1;
                }

                assert!(at == N);

                res
            },
        }
    }

    pub const fn as_str(&self) -> DomTokensPrefixSpaceStr<'_> {
        // this could be optimized with unsafe but this is called in const runtime
        //  so the performance doesn't matter
        if let Ok(inner) = std::str::from_utf8(&self.inner) {
            DomTokensPrefixSpaceStr { inner }
        } else {
            panic!("invalid utf8")
        }
    }
}

/// A str which satisfies [`crate::ssr::asserts::DomTokensPrefixSpace`].
#[derive(Debug, Clone, Copy)]
pub struct DomTokensPrefixSpaceStr<'a> {
    inner: &'a str,
}

impl<'a> DomTokensPrefixSpaceStr<'a> {
    const fn to_str(self) -> &'a str {
        self.inner
    }

    fn to_str_without_prefix_space(self) -> &'a str {
        &self.to_str()[1..]
    }
}

pub trait HasConstDomTokens {
    // ssr
    const DOM_TOKENS_PREFIX_SPACE: DomTokensPrefixSpaceStr<'static>;

    // Note that implementations might have different values for ssr and csr
    // csr
    type DomTokensLen: IsConstUsize;
    const DOM_TOKENS: <Self::DomTokensLen as IsConstUsize>::UniqueDomTokenArray<'static>;
}

frender_macro_rules::define_phantom_wrapper!(
    #[always_derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ConstDomTokens<T: ?Sized + HasConstDomTokens>;
);

pub mod ssr {
    use std::task::Poll;

    use async_str_iter::AsyncStrIterator;

    use super::{ConstDomTokens, HasConstDomTokens};

    pub struct ConstDomTokensIntoAsyncStrIter<T: ?Sized + HasConstDomTokens> {
        pub(crate) _const: ConstDomTokens<T>,
        pub(crate) yielded: bool,
    }

    impl<T: ?Sized + HasConstDomTokens> Unpin for ConstDomTokensIntoAsyncStrIter<T> {}

    impl<T: ?Sized + HasConstDomTokens> AsyncStrIterator for ConstDomTokensIntoAsyncStrIter<T> {
        fn poll_next_str(
            self: std::pin::Pin<&mut Self>,
            _: &mut std::task::Context<'_>,
        ) -> Poll<Option<&str>> {
            Poll::Ready({
                let this = self.get_mut();
                if this.yielded {
                    None
                } else {
                    this.yielded = true;
                    Some(T::DOM_TOKENS_PREFIX_SPACE.to_str_without_prefix_space())
                }
            })
        }
    }

    pub struct ConstDomTokensPrefixSpaceIntoAsyncStrIter<T: ?Sized + HasConstDomTokens> {
        pub(crate) _const: ConstDomTokens<T>,
        pub(crate) yielded: bool,
    }

    impl<T: ?Sized + HasConstDomTokens> Unpin for ConstDomTokensPrefixSpaceIntoAsyncStrIter<T> {}

    impl<T: ?Sized + HasConstDomTokens> AsyncStrIterator
        for ConstDomTokensPrefixSpaceIntoAsyncStrIter<T>
    {
        fn poll_next_str(
            self: std::pin::Pin<&mut Self>,
            _: &mut std::task::Context<'_>,
        ) -> Poll<Option<&str>> {
            Poll::Ready({
                let this = self.get_mut();
                if this.yielded {
                    None
                } else {
                    this.yielded = true;
                    Some(T::DOM_TOKENS_PREFIX_SPACE.to_str())
                }
            })
        }
    }
}

impl<T: ?Sized + HasConstDomTokens> DomTokens for ConstDomTokens<T> {
    type UpdateWithState = bool; // whether updated

    fn update_with_state(
        _: Self,
        dom_token_list: &mut impl crate::DomTokenList,
        state: &mut Self::UpdateWithState,
    ) {
        if *state {
            return;
        }
        *state = true;
        T::DOM_TOKENS
            .as_ref()
            .iter()
            .for_each(|t| dom_token_list.add_1(*t))
    }

    fn remove_with_state(
        dom_token_list: &mut impl crate::DomTokenList,
        state: &mut Self::UpdateWithState,
    ) {
        if *state {
            *state = false;
            T::DOM_TOKENS
                .as_ref()
                .iter()
                .for_each(|t| dom_token_list.remove_1(*t))
        }
    }

    type DomTokensIntoAsyncStrIter = ssr::ConstDomTokensIntoAsyncStrIter<T>;

    fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter {
        ssr::ConstDomTokensIntoAsyncStrIter {
            _const: this,
            yielded: false,
        }
    }
}

impl<T: ?Sized + HasConstDomTokens<DomTokensLen = ConstUsize<N>>, const N: usize> ChainableDomTokens
    for ConstDomTokens<T>
{
    type DomTokensPrefixSpaceIntoAsyncStrIter = ssr::ConstDomTokensPrefixSpaceIntoAsyncStrIter<T>;

    fn dom_tokens_prefix_space_into_async_str_iter(
        this: Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
        ssr::ConstDomTokensPrefixSpaceIntoAsyncStrIter {
            _const: this,
            yielded: false,
        }
    }
}

impl<T: ?Sized + HasConstDomTokens<DomTokensLen = ConstUsize<N>>, const N: usize>
    HasConstKnownPossibleDomTokens for ConstDomTokens<T>
{
    type KnownPossibleDomTokensArrayVecCap = T::DomTokensLen;

    const KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC: UniqueDomTokenArrayVec<'static, N> =
        UniqueDomTokenArrayVec::from_array(T::DOM_TOKENS);
}

pub struct DomTokensInfo {
    pub count: usize,
    pub prefix_space_len: usize,
}

#[macro_export]
macro_rules! impl_has_const_dom_tokens_for {
    (impl <$(__)?> $($rest:tt)*) => {
        $crate::impl_has_const_dom_tokens_for! {
            impl $($rest)*
        }
    };
    (
        impl $for_ty:ty {
            const $NAME:tt: _ = $const_expr:expr;
        }
    ) => {
        $crate::impl_has_const_dom_tokens_for! {
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
            const DOM_TOKENS_CONST_EXPR: $crate::constness::DomTokensConstExpr<$str_ty> = $crate::constness::DomTokensConstExpr($const_expr);

            $crate::__expand_if_not_underscore! {
                $NAME
                impl $for_ty {
                    const $NAME: $str_ty = DOM_TOKENS_CONST_EXPR.0;
                }
            }

            const DOM_TOKENS_INFO: $crate::constness::DomTokensInfo = DOM_TOKENS_CONST_EXPR.into_info();

            const DOM_TOKENS_ARRAY: [$crate::DomToken<'static>; DOM_TOKENS_INFO.count] =
                DOM_TOKENS_CONST_EXPR.into_dom_tokens_array();

            impl $crate::constness::HasConstDomTokens for $for_ty {
                const DOM_TOKENS_PREFIX_SPACE: $crate::constness::DomTokensPrefixSpaceStr<'static> = {
                    $crate::constness::DomTokensPrefixSpaceString::<{ DOM_TOKENS_INFO.prefix_space_len }>::from_dom_tokens(
                        &DOM_TOKENS_ARRAY,
                    )
                    .as_str()
                };

                type DomTokensLen = $crate::constness::ConstUsize<{ DOM_TOKENS_ARRAY.len() }>;

                const DOM_TOKENS: $crate::UniqueDomTokenArray<'static, { DOM_TOKENS_ARRAY.len() }> =
                    $crate::UniqueDomTokenArray::new_const(DOM_TOKENS_ARRAY);
            }
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expand_if_not_underscore {
    (_     $($rest:tt)*) => {};
    ($t:tt $($rest:tt)*) => {
        $($rest)*
    };
}

pub type StaticStr = &'static str;

pub struct DomTokensConstExpr<T>(pub T);

impl<'a> DomTokensConstExpr<&'a str> {
    pub const fn into_info(self) -> DomTokensInfo {
        DomTokensInfo::collect_info_from(self.0)
    }
    pub const fn into_dom_tokens_array<const LEN: usize>(self) -> [DomToken<'a>; LEN] {
        crate::dom_token::separate::separate_dom_tokens(self.0)
    }
}

impl<'a, const N: usize> DomTokensConstExpr<[&'a str; N]> {
    /// &str is not validated for DomToken
    pub const fn into_info(self) -> DomTokensInfo {
        DomTokensConstExpr(self.0.as_slice()).into_info()
    }
    pub const fn into_dom_tokens_array<const LEN: usize>(self) -> [DomToken<'a>; LEN] {
        DomTokensConstExpr(self.0.as_slice()).into_dom_tokens_array()
    }
}

impl<'a> DomTokensConstExpr<&[&'a str]> {
    /// &str is not validated for DomToken
    pub const fn into_info(self) -> DomTokensInfo {
        DomTokensInfo {
            count: self.0.len(),
            prefix_space_len: {
                let mut res = 0;
                let mut i = 0;
                let list = self.0;

                while i < self.0.len() {
                    res += 1 + list[i].len();
                    i += 1;
                }

                res
            },
        }
    }
    pub const fn into_dom_tokens_array<const LEN: usize>(self) -> [DomToken<'a>; LEN] {
        let mut res = [DomToken::DUMMY; LEN];
        let mut i = 0;
        while i < self.0.len() {
            let s = self.0[i];
            res[i] = DomToken::new_const(s);
            i += 1;
        }

        assert!(i == LEN);

        res
    }
}

impl<'a, const N: usize> DomTokensConstExpr<[DomToken<'a>; N]> {
    pub const fn into_info(self) -> DomTokensInfo {
        DomTokensConstExpr(self.0.as_slice()).into_info()
    }
    pub const fn into_dom_tokens_array<const LEN: usize>(self) -> [DomToken<'a>; LEN] {
        DomTokensConstExpr(self.0.as_slice()).into_dom_tokens_array()
    }
}

impl<'a> DomTokensConstExpr<&[DomToken<'a>]> {
    pub const fn into_info(self) -> DomTokensInfo {
        DomTokensInfo {
            count: self.0.len(),
            prefix_space_len: {
                let mut res = 0;
                let mut i = 0;
                let list = self.0;

                while i < self.0.len() {
                    res += 1 + list[i].as_str().len();
                    i += 1;
                }

                res
            },
        }
    }

    pub const fn into_dom_tokens_array<const LEN: usize>(self) -> [DomToken<'a>; LEN] {
        let (res, at) = put_at([DomToken::DUMMY; LEN], 0, self.0);
        assert!(at == LEN);
        res
    }
}
