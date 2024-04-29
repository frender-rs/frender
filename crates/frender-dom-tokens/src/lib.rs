use std::str;

#[derive(Debug, Clone, Copy)]
pub struct DomToken<'a>(&'a str);

impl<'a> std::ops::Deref for DomToken<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> DomToken<'a> {
    pub const fn new_const(s: &'a str) -> Self {
        assert_valid_dom_token(s);
        Self(s)
    }

    pub const fn as_str(self) -> &'a str {
        self.0
    }
}

const fn assert_ascii(s: &str) {
    if !s.is_ascii() {
        panic!("currently only ascii dom tokens are supported")
    }
}

// See https://www.w3.org/TR/2011/WD-html5-20110525/common-microsyntaxes.html#space-character
const fn is_space_char(v: u8) -> bool {
    match v {
        b'\x20' | b'\x09' | b'\x0A' | b'\x0C' | b'\x0D' => true,
        _ => false,
    }
}

pub const fn assert_valid_dom_token(s: &str) {
    assert_ascii(s);
    let s = s.as_bytes();

    let mut i = 0;

    while i < s.len() {
        if is_space_char(s[i]) {
            panic!("dom token can't contain space characters")
        }
        i += 1;
    }
}

pub const fn assert_valid_dom_tokens(tokens: &[&str]) {
    let mut i = 0;
    while i < tokens.len() {
        assert_valid_dom_token(tokens[i]);
        i += 1;
    }
}

const fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && {
        let mut i = 0;
        while i < a.len() {
            if a[i] != b[i] {
                return false;
            }

            i += 1;
        }

        true
    }
}

pub const fn assert_each_unique(tokens: &[&str]) {
    if tokens.len() <= 1 {
        return;
    }
    let mut i = 0;
    while i < tokens.len() - 1 {
        let mut j = i + 1;
        while j < tokens.len() {
            if bytes_eq(tokens[i].as_bytes(), tokens[j].as_bytes()) {
                panic!("each item of dom tokens must be unique")
            }
            j += 1;
        }
        i += 1;
    }
}

pub trait ConstPossibleDomTokens {
    const POSSIBLE_DOM_TOKENS: &'static [&'static str];
}

pub mod __private {
    pub const fn put_tokens_at<'a, const N: usize>(
        mut res: [&'a str; N],
        mut at: usize,
        tokens: &[&'a str],
    ) -> ([&'a str; N], usize) {
        let mut j = 0;
        while j < tokens.len() {
            res[at] = tokens[j];
            at += 1;
            j += 1;
        }

        (res, at)
    }

    pub use bool;
    pub use Option;

    pub const fn join_dedup_count(a: &[&str], b: &[&str]) -> usize {
        use super::{assert_each_unique, assert_valid_dom_tokens, contains};

        assert_valid_dom_tokens(a);
        assert_valid_dom_tokens(b);

        assert_each_unique(a);
        assert_each_unique(b);

        let mut count = a.len();
        let mut i = 0;
        while i < b.len() {
            if !contains(a, b[i]) {
                count += 1;
            }
            i += 1;
        }

        count
    }

    pub const fn join_dedup<'a, const N: usize>(a: &[&'a str], b: &[&'a str]) -> [&'a str; N] {
        use super::{assert_each_unique, assert_valid_dom_tokens, contains};

        assert_valid_dom_tokens(a);
        assert_valid_dom_tokens(b);

        assert_each_unique(a);
        assert_each_unique(b);

        let mut res = [""; N];
        let mut at = 0;
        (res, at) = put_tokens_at(res, at, &a);

        let mut i = 0;
        while i < b.len() {
            let b = b[i];
            if !contains(&a, b) {
                res[at] = b;
                at += 1;
            }
            i += 1;
        }

        assert!(at == N);

        res
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __dom_tokens_array_or_slice {
    () => {
        []
    };
    ($dom_token:literal) => {
        [$dom_token]
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        [$($dom_token),+]
    };
    (( $e:expr ) as $as_ty:ty) => {
        <$as_ty as $crate::ConstPossibleDomTokens>::POSSIBLE_DOM_TOKENS
    };
    (if ( $e:expr ) $if_block:tt) => {
        $crate::__dom_tokens_array_or_slice! $if_block
    };
    (if ( $e:expr ) $if_block:tt else $else_block:tt) => {{
        const A: &[&'static str] = &$crate::__dom_tokens_array_or_slice! $if_block;
        const B: &[&'static str] = &$crate::__dom_tokens_array_or_slice! $else_block;

        const N: usize = $crate::__private::join_dedup_count(A, B);

        $crate::__private::join_dedup::<N>(A, B)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __dom_tokens_count {
    () => {
        0
    };
    ($dom_token:literal) => {
        1
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        [$($dom_token),+].len()
    };
    (( $e:expr ) as $as_ty:ty) => {
        <$as_ty as $crate::ConstPossibleDomTokens>::POSSIBLE_DOM_TOKENS.len()
    };
    (if ( $e:expr ) $if_block:tt) => {
        $crate::__dom_tokens_count! $if_block
    };
    (if ( $e:expr ) $if_block:tt else $else_block:tt) => {
        $crate::__private::join_dedup_count(
            &$crate::__dom_tokens_array_or_slice! $if_block,
            &$crate::__dom_tokens_array_or_slice! $else_block,
        )
    };
}

const fn non_empty_count<const N: usize>(tokens: [&'static str; N]) -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < N {
        if !tokens[i].is_empty() {
            count += 1;
        }
        i += 1;
    }

    count
}

// duplicated tokens in `a` will be replaced with `""`.
const fn dedup_from<'a, const A: usize>(mut a: [&'a str; A], b: &[&str]) -> ([&'a str; A], usize) {
    assert_valid_dom_tokens(&b);
    assert_valid_dom_tokens(&a);

    assert_each_unique(&b);
    assert_each_unique(&a);

    let mut i = 0;
    let mut removed = 0;
    while i < b.len() {
        let b = b[i].as_bytes();
        let mut j = 0;
        while j < a.len() {
            if bytes_eq(b, a[j].as_bytes()) {
                a[j] = "";
                removed += 1;
            }
            j += 1;
        }

        i += 1;
    }

    (a, removed)
}

const fn contains(c: &[&str], s: &str) -> bool {
    let mut i = 0;
    while i < c.len() {
        if bytes_eq(c[i].as_bytes(), s.as_bytes()) {
            return true;
        }
    }
    false
}

#[doc(hidden)]
#[macro_export]
macro_rules! __put_dom_tokens_at {
    ({$dom_token:literal} $res:ident, $at:ident) => {{
        $res[$at] = $dom_token;
        $at += 1;
    }};
    ($dom_tokens:tt $res:ident, $at:ident) => {{
        ($res, $at) = $crate::__private::put_tokens_at($res, $at, &$crate::__dom_tokens_array_or_slice! $dom_tokens)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __dom_token_predicate {
    ($dom_token:literal) => {
        ()
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        ()
    };
    (( $e:expr ) as $as_ty:ty) => {
        $e
    };
    (if ( $e:expr ) $if_block:tt $(else $else_block:tt)?) => {
        $e
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __dom_token_predicate_ty {
    ($dom_token:literal) => {
        ()
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        ()
    };
    (( $e:expr ) as $as_ty:ty) => {
        $crate::__private::bool
    };
    (if ( $e:expr ) $if_block:tt $(else $else_block:tt)?) => {
        $crate::__private::bool
    };
}

#[macro_export]
macro_rules! __anonymous_custom_dom_tokens {
    (
        $($dom_token:tt)+
    ) => {{
        struct AnonymousCustomDomTokens {
            _inner: ($($crate::__dom_token_predicate_ty! $dom_token ,)+)
        }

        const _: () = {
            const POSSIBLE_DOM_TOKENS_COUNT: usize = {
                0
                $(+ $crate::__dom_tokens_count! $dom_token)+
            };

            impl $crate::ConstPossibleDomTokens for AnonymousCustomDomTokens {
                const POSSIBLE_DOM_TOKENS: &'static [&'static str] = &{
                    let mut res = [""; POSSIBLE_DOM_TOKENS_COUNT];
                    let mut i = 0;
                    $($crate::__put_dom_tokens_at!($dom_token res, i);)+
                    assert!(i == POSSIBLE_DOM_TOKENS_COUNT);

                    $crate::assert_valid_dom_tokens(&res);
                    $crate::assert_each_unique(&res);

                    res
                };
            }
        };

        AnonymousCustomDomTokens {
            _inner: ($($crate::__dom_token_predicate! $dom_token ,)+)
        }
    }};
}

#[macro_export]
macro_rules! dom_tokens {
    () => {
        ()
    };
    ($($t:tt)*) => {
        $crate::__parse_dom_tokens!([]{$($t)*}{$($t)*})
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_dom_tokens {
    ($t:tt {}{}) => {
        $crate::__anonymous_custom_dom_tokens! $t
    };
    (
        [$($t:tt)*]
        {$dom_token:literal $(, $($rest:tt)*)?}
        $tee:tt
    ) => {
        $crate::__parse_dom_tokens!(
            [$($t)* { $dom_token }]
            {$($($rest)*)?}
            {$($($rest)*)?}
        )
    };
    ($t:tt { if $_pred:tt $($_rest:tt)* } { $if:ident $pred:tt $($rest:tt)* }) => {
        $crate::__parse_dom_tokens!($t if { $if } ($pred) {$($_rest)*} {$($rest)*})
    };
    ($t:tt if {$($if:tt)*} $pred:tt {{$($_block:tt)*} $($_rest:tt)*} {$block:tt $($rest:tt)*}) => {
        $crate::__parse_dom_tokens!($t if_end { $($if)* $pred $block } {$($_rest)*} {$($rest)*})
    };
    (
        [$($t:tt)*]
        if_end $if:tt
        {$(, $($_rest:tt)*)?}
        {$(, $($rest:tt )*)?}
    ) => {
        $crate::__parse_dom_tokens!([$($t)* $if] { $($($_rest)*)? } { $($($rest)*)? })
    };
    (
        [$($t:tt)*]
        if_end {$($if:tt)*}
        { else        {$($_else_block:tt)*} $(, $($_rest:tt)*)? }
        { $else:tt $else_block:tt        $(, $($rest:tt )*)? }
    ) => {
        $crate::__parse_dom_tokens!(
            [$($t)* { $($if)* $else  $else_block }]
            { $($($_rest)*)? }
            { $($($rest)*)? }
        )
    };
    (
        $t:tt
        if_end {$($if:tt)*}
        { else     if          $_pred:tt $($_rest:tt)* }
        { $else:tt $else_if:tt $pred:tt  $($rest:tt)*  }
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            if {$($if:tt)* $else $else_if} ($pred)
            { $($($_rest)*)? }
            { $($($rest)*)? }
        )
    };
    (
        [$($t:tt)*]
        { [$($_array:tt)*] $(, $($_rest:tt)*)? }
        { $array:tt        $(, $($rest:tt )*)? }
    ) => {
        $crate::__parse_dom_tokens!(
            [$($t)* { $array }]
            {$($($_rest)*)?}
            {$($($rest )*)?}
        )
    };
    (
        $t:tt
        { $e:tt $($rest:tt)* }
        $tee:tt
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            expr_as ($e)
            {$($rest)*}
            {$($rest)*}
        )
    };
    (
        [$($t:tt)*]
        expr_as $e:tt
        { as     $_as_ty:ty $(, $($_rest:tt)*)? }
        { $as:tt $as_ty:ty  $(, $($rest:tt )*)? }
    ) => {
        $crate::__parse_dom_tokens!(
            [$($t)* { $e $as $as_ty }]
            {$($($_rest)*)?}
            {$($($rest )*)?}
        )
    };
    (
        $t:tt
        expr_as ($($pre:tt)*)
        { $e:tt $($rest:tt)* }
        $tee:tt
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            expr_as ($($pre)* $e)
            {$($rest)*}
            {$($rest)*}
        )
    };
}

#[cfg(test)]
mod tests {
    mod anonymous {
        const _: () = {
            let _: () = dom_tokens!();
            {
                let tokens = dom_tokens!("a");
            }
            {
                let circle = true;
                let array = false;
                let tokens = dom_tokens!(
                    "my-btn",
                    if circle {
                        "circle"
                    },
                    if primary {
                    } else {
                    },
                    if array {
                        ["a", "b"]
                    }
                );
            }
        };
    }

    use crate::{ConstPossibleDomTokens, __private::put_tokens_at};

    struct CustomDomTokens;

    impl ConstPossibleDomTokens for CustomDomTokens {
        // const DOM_TOKEN_COUNT: usize = 3;
        // const DOM_TOKENS: [&'static str; Self::DOM_TOKEN_COUNT] = ["a", "b", "c"];
        const POSSIBLE_DOM_TOKENS: &'static [&'static str] = &["a", "b", "c"];
    }

    struct CustomDomTokens2;

    impl ConstPossibleDomTokens for CustomDomTokens2 {
        // const DOM_TOKEN_COUNT: usize = 2;
        // const DOM_TOKENS: [&'static str; Self::DOM_TOKEN_COUNT] = ["d", "e"];
        const POSSIBLE_DOM_TOKENS: &'static [&'static str] = &["d", "e"];
    }

    struct CustomDomTokens3;

    impl ConstPossibleDomTokens for CustomDomTokens3 {
        // const DOM_TOKEN_COUNT: usize =
        //     CustomDomTokens::DOM_TOKENS.len() + CustomDomTokens2::DOM_TOKENS.len();
        // const DOM_TOKENS: [&'static str; Self::DOM_TOKEN_COUNT] = {
        //     let mut res = [""; Self::DOM_TOKEN_COUNT];
        //
        //     let mut i = 0;
        //     // while i<
        //     res
        // };
        const POSSIBLE_DOM_TOKENS: &'static [&'static str] = &{
            let mut res = [""; {
                CustomDomTokens::POSSIBLE_DOM_TOKENS.len()
                    + CustomDomTokens2::POSSIBLE_DOM_TOKENS.len()
            }];

            let mut i = 0;
            (res, i) = put_tokens_at(res, i, CustomDomTokens::POSSIBLE_DOM_TOKENS);
            (res, i) = put_tokens_at(res, i, CustomDomTokens2::POSSIBLE_DOM_TOKENS);

            assert!(i == res.len());

            res
        };
    }

    #[test]
    fn custom() {
        assert_eq!(
            CustomDomTokens3::POSSIBLE_DOM_TOKENS,
            ["a", "b", "c", "d", "e"]
        );
    }
}
