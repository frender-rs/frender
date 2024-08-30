use std::ops::Deref;

use crate::constness::DomTokensInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DomToken<'a>(&'a str);

impl<'a> Deref for DomToken<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> PartialEq<str> for DomToken<'a> {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl<'a> PartialEq<&str> for DomToken<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

// See https://www.w3.org/TR/2011/WD-html5-20110525/common-microsyntaxes.html#space-character
const fn is_space_char(v: u8) -> bool {
    match v {
        b'\x20' | b'\x09' | b'\x0A' | b'\x0C' | b'\x0D' => true,
        _ => false,
    }
}

const fn assert_ascii(s: &str) {
    if !s.is_ascii() {
        panic!("currently only ascii dom tokens are supported")
    }
}

const fn assert_valid_dom_token(s: &str) {
    if s.is_empty() {
        panic!("dom token can't be empty")
    }

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

impl<'a> DomToken<'a> {
    pub const fn new_const(s: &'a str) -> Self {
        assert_valid_dom_token(s);
        Self(s)
    }

    pub const fn as_str(self) -> &'a str {
        self.0
    }

    pub(crate) const DUMMY: Self = Self::new_const("_");
}

#[derive(Debug, Clone, Copy)]
pub struct UniqueDomTokens<'a, 'b>(&'a [DomToken<'b>]);

impl<'a, 'b> Deref for UniqueDomTokens<'a, 'b> {
    type Target = [DomToken<'b>];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, 'b> UniqueDomTokens<'a, 'b> {
    pub const EMPTY: Self = UniqueDomTokens(&[]);

    pub const fn new_const(v: &'a [DomToken<'b>]) -> Self {
        assert_each_unique(v);
        Self(v)
    }

    pub const fn as_slice(self) -> &'a [DomToken<'b>] {
        self.0
    }

    pub const fn contains(self, s: &str) -> bool {
        dom_tokens_contain(self.as_slice(), s)
    }

    pub const fn join_count(self, b: UniqueDomTokens) -> usize {
        let this = self.as_slice();
        let b = b.as_slice();
        let mut count = this.len();
        let mut i = 0;
        while i < b.len() {
            if !self.contains(b[i].as_str()) {
                count += 1;
            }
            i += 1;
        }

        count
    }

    pub const fn join<const N: usize>(
        self,
        b: UniqueDomTokens<'_, 'b>,
    ) -> UniqueDomTokenArray<'b, N> {
        let b = b.as_slice();
        let mut res = [DomToken::new_const("_"); N];
        let mut at = 0;
        (res, at) = put_tokens_at(res, at, self.as_slice());

        let mut i = 0;
        while i < b.len() {
            let b = b[i];
            if !self.contains(b.as_str()) {
                res[at] = b;
                at += 1;
            }
            i += 1;
        }

        assert!(at == N);

        UniqueDomTokenArray::new_const(res)
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

const fn assert_each_unique(tokens: &[DomToken]) {
    if tokens.len() <= 1 {
        return;
    }
    let mut i = 0;
    while i < tokens.len() - 1 {
        let mut j = i + 1;
        while j < tokens.len() {
            if bytes_eq(tokens[i].as_str().as_bytes(), tokens[j].as_str().as_bytes()) {
                panic!("each item of dom tokens must be unique")
            }
            j += 1;
        }
        i += 1;
    }
}

pub const fn put_tokens_at<'a, const N: usize>(
    arr: [DomToken<'a>; N],
    at: usize,
    tokens: &[DomToken<'a>],
) -> ([DomToken<'a>; N], usize) {
    frender_common::const_utils::put_at(arr, at, tokens)
}

#[derive(Debug, Clone, Copy)]
pub struct UniqueDomTokenArray<'a, const N: usize>([DomToken<'a>; N]);

impl<'a, const N: usize> UniqueDomTokenArray<'a, N> {
    pub const fn new_const(arr: [DomToken<'a>; N]) -> Self {
        assert_each_unique(&arr);
        Self(arr)
    }

    pub const fn into_array(self) -> [DomToken<'a>; N] {
        self.0
    }

    pub const fn as_unique_dom_tokens(&self) -> UniqueDomTokens<'_, 'a> {
        UniqueDomTokens(&self.0)
    }

    pub const fn as_slice(&self) -> &'_ [DomToken<'a>] {
        &self.0
    }

    pub fn for_each(self, f: impl FnMut(DomToken<'a>)) {
        self.0.into_iter().for_each(f)
    }
}

impl<'a, const N: usize> Deref for UniqueDomTokenArray<'a, N> {
    type Target = [DomToken<'a>; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UniqueDomTokenArrayVec<'a, const CAP: usize> {
    // array[..len] must be a set
    array: [DomToken<'a>; CAP],
    len: usize,
}

impl<'a, const CAP: usize> UniqueDomTokenArrayVec<'a, CAP> {
    pub const fn as_slice(&self) -> &[DomToken<'a>] {
        self.array.split_at(self.len).0
    }

    pub const fn as_unique_dom_tokens(&self) -> UniqueDomTokens<'_, 'a> {
        // self.as_slice() is a set
        UniqueDomTokens(self.as_slice())
    }

    pub(crate) const EMPTY: Self = UniqueDomTokenArrayVec {
        array: [DomToken::DUMMY; CAP],
        len: 0,
    };

    pub(crate) const fn with_extend_unique_dom_tokens(
        mut self,
        other: UniqueDomTokens<'_, 'a>,
    ) -> Self {
        let old_len = self.len;

        let other = other.as_slice();

        let mut i = 0;

        while i < other.len() {
            let t = other[i];

            // other is a set so we only need to check old doesn't contain other[i];
            let old = self.array.split_at(old_len).0;
            assert_dom_tokens_not_contain(old, t.as_str());

            self.array[self.len] = t;
            self.len += 1;

            i += 1;
        }

        self
    }

    pub(crate) const fn with_extend_unique_dom_tokens_and_remove_duplicated(
        mut self,
        other: UniqueDomTokens<'_, 'a>,
    ) -> Self {
        let old_len = self.len;

        let other = other.as_slice();

        let mut i = 0;

        while i < other.len() {
            let t = other[i];

            // other is a set so we only need to check old doesn't contain other[i];
            let old = self.array.split_at(old_len).0;

            if !dom_tokens_contain(old, t.as_str()) {
                self.array[self.len] = t;
                self.len += 1;
            }

            i += 1;
        }

        self
    }

    pub(crate) const fn with_capacity<const NEW_CAP: usize>(
        self,
    ) -> UniqueDomTokenArrayVec<'a, NEW_CAP> {
        let (array, len) =
            frender_common::const_utils::put_at([DomToken::DUMMY; NEW_CAP], 0, self.as_slice());

        debug_assert!(len == self.len);

        // array is a set
        UniqueDomTokenArrayVec { array, len }
    }

    pub(crate) const fn from_array(dom_tokens: UniqueDomTokenArray<'a, CAP>) -> Self {
        Self {
            array: dom_tokens.into_array(),
            len: CAP,
        }
    }
}

impl<'a, const CAP: usize> AsRef<[DomToken<'a>]> for UniqueDomTokenArrayVec<'a, CAP> {
    fn as_ref(&self) -> &[DomToken<'a>] {
        self.as_slice()
    }
}

impl<'a, const CAP: usize, const N: usize> TryFrom<UniqueDomTokenArrayVec<'a, CAP>>
    for UniqueDomTokenArray<'a, N>
{
    type Error = UniqueDomTokenArrayVec<'a, CAP>;

    fn try_from(value: UniqueDomTokenArrayVec<'a, CAP>) -> Result<Self, Self::Error> {
        if value.len == N {
            Ok(
                // self.array is a set
                UniqueDomTokenArray({
                    let (res, at) = frender_common::const_utils::put_at(
                        [DomToken("_"); N],
                        0,
                        value.as_slice(),
                    );

                    assert!(at == N);

                    res
                }),
            )
        } else {
            Err(value)
        }
    }
}

pub const fn dom_tokens_contain(this: &[DomToken], s: &str) -> bool {
    let mut i = 0;
    while i < this.len() {
        if bytes_eq(this[i].as_str().as_bytes(), s.as_bytes()) {
            return true;
        }
        i += 1;
    }
    false
}

const fn assert_dom_tokens_not_contain(this: &[DomToken], s: &str) {
    if dom_tokens_contain(this, s) {
        const MSG_CAP: usize = 64;

        let msg = frender_common::const_utils::ArrayString::<MSG_CAP>::new()
            .with_push_str("duplicated dom token: ")
            .with_push_str_or_elide(s);

        panic!("{}", msg.as_str());
    }
}

const fn slice_range<T>(s: &[T], std::ops::Range { start, end }: std::ops::Range<usize>) -> &[T] {
    s.split_at(end).0.split_at(start).1
}

pub mod separate {
    use super::*;

    pub const fn separate_dom_tokens_count(s: &str) -> usize {
        assert_ascii(s);

        let s = s.as_bytes();

        let mut count = 0;

        let mut i = 0;
        let mut current_is_token = false;

        while i < s.len() {
            if is_space_char(s[i]) {
                current_is_token = false;
            } else {
                if !current_is_token {
                    count += 1;
                    current_is_token = true;
                }
            }
            i += 1;
        }

        count
    }

    pub const fn separate_dom_tokens<const N: usize>(s: &str) -> [DomToken<'_>; N] {
        assert_ascii(s);

        let bytes = s.as_bytes();

        let mut res = [DomToken::new_const("_"); N];

        let mut i = 0;
        let mut cur_token_start = None::<usize>;
        let mut i_of_res = 0;

        while i < bytes.len() {
            if is_space_char(bytes[i]) {
                if let Some(cur_token_start) = cur_token_start {
                    let token = slice_range(bytes, cur_token_start..i);
                    let token = match std::str::from_utf8(token) {
                        Ok(v) => v,
                        Err(_) => panic!("bytes are not valid utf8"),
                    };
                    res[i_of_res] = DomToken::new_const(token);
                    i_of_res += 1;
                }
                cur_token_start = None;
            } else {
                if cur_token_start.is_none() {
                    cur_token_start = Some(i);
                }
            }

            i += 1;
        }

        assert!(i == bytes.len());

        {
            if let Some(cur_token_start) = cur_token_start {
                let token = slice_range(bytes, cur_token_start..i);
                let token = match std::str::from_utf8(token) {
                    Ok(v) => v,
                    Err(_) => panic!("bytes are not valid utf8"),
                };
                res[i_of_res] = DomToken::new_const(token);
                i_of_res += 1;
            }
            cur_token_start = None;
        }

        assert!(i_of_res == N);
        assert!(cur_token_start.is_none());

        res
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn count() {
            assert_eq!(super::separate_dom_tokens_count(""), 0);
            assert_eq!(super::separate_dom_tokens_count(" "), 0);
            assert_eq!(super::separate_dom_tokens_count("\r"), 0);
            assert_eq!(super::separate_dom_tokens_count("  "), 0);
            assert_eq!(super::separate_dom_tokens_count(" \r"), 0);

            assert_eq!(super::separate_dom_tokens_count("a"), 1);
            assert_eq!(super::separate_dom_tokens_count("a "), 1);
            assert_eq!(super::separate_dom_tokens_count(" a"), 1);
            assert_eq!(super::separate_dom_tokens_count(" a "), 1);

            assert_eq!(super::separate_dom_tokens_count("a b"), 2);
            assert_eq!(super::separate_dom_tokens_count("a  b"), 2);
            assert_eq!(super::separate_dom_tokens_count(" a b"), 2);
            assert_eq!(super::separate_dom_tokens_count("a b "), 2);
            assert_eq!(super::separate_dom_tokens_count(" a b "), 2);
        }

        #[test]
        fn array() {
            const EMPTY: [&str; 0] = [];
            assert_eq!(super::separate_dom_tokens::<0>(""), EMPTY);
            assert_eq!(super::separate_dom_tokens::<0>(" "), EMPTY);
            assert_eq!(super::separate_dom_tokens::<0>("\r"), EMPTY);
            assert_eq!(super::separate_dom_tokens::<0>("  "), EMPTY);
            assert_eq!(super::separate_dom_tokens::<0>(" \r"), EMPTY);

            assert_eq!(super::separate_dom_tokens::<1>("a"), ["a"]);
            assert_eq!(super::separate_dom_tokens::<1>("a "), ["a"]);
            assert_eq!(super::separate_dom_tokens::<1>(" a"), ["a"]);
            assert_eq!(super::separate_dom_tokens::<1>(" a "), ["a"]);

            assert_eq!(super::separate_dom_tokens::<2>("a b"), ["a", "b"]);
            assert_eq!(super::separate_dom_tokens::<2>("a  b"), ["a", "b"]);
            assert_eq!(super::separate_dom_tokens::<2>(" a b"), ["a", "b"]);
            assert_eq!(super::separate_dom_tokens::<2>("a b "), ["a", "b"]);
            assert_eq!(super::separate_dom_tokens::<2>(" a b "), ["a", "b"]);
        }
    }
}

const fn str_from_utf8(s: &[u8]) -> &str {
    if let Ok(s) = std::str::from_utf8(s) {
        s
    } else {
        panic!("invalid utf8")
    }
}

// `(space_and_dom_tokens, dom_tokens)` is returned.
pub const fn make_dom_tokens_strings(spaces_and_dom_tokens: &str) -> (&str, &str) {
    assert_ascii(spaces_and_dom_tokens);

    let bytes = spaces_and_dom_tokens.as_bytes();

    let mut index_of_last_space_char_before_dom_tokens = None;
    let mut index_of_last_char_of_dom_tokens = None;

    let mut i = 0;

    while i < bytes.len() {
        if index_of_last_char_of_dom_tokens.is_none() {
            if is_space_char(bytes[i]) {
                index_of_last_space_char_before_dom_tokens = Some(i)
            } else {
                if index_of_last_space_char_before_dom_tokens.is_none() {
                    panic!("expect space char before dom tokens")
                } else {
                    index_of_last_char_of_dom_tokens = Some(i)
                }
            }
        } else {
            if !is_space_char(bytes[i]) {
                index_of_last_char_of_dom_tokens = Some(i)
            }
        }

        i += 1;
    }

    if let Some(index_of_last_char_of_dom_tokens) = index_of_last_char_of_dom_tokens {
        let index_of_last_space_char_before_dom_tokens =
            match index_of_last_space_char_before_dom_tokens {
                Some(v) => v,
                None => unreachable!(),
            };

        let space_and_dom_tokens = slice_range(
            bytes,
            index_of_last_space_char_before_dom_tokens..(index_of_last_char_of_dom_tokens + 1),
        );

        let dom_tokens = match space_and_dom_tokens.split_first() {
            Some(val) => val,
            None => unreachable!(),
        }
        .1;

        let space_and_dom_tokens = str_from_utf8(space_and_dom_tokens);
        let dom_tokens = str_from_utf8(dom_tokens);

        (space_and_dom_tokens, dom_tokens)
    } else {
        // no dom tokens
        ("", "")
    }
}

impl DomTokensInfo {
    pub const fn collect_info_from(s: &str) -> Self {
        assert_ascii(s);

        let s = s.as_bytes();

        let mut count = 0;
        let mut prefix_space_len = 0;

        let mut i = 0;
        let mut current_is_token = false;

        while i < s.len() {
            if is_space_char(s[i]) {
                current_is_token = false;
            } else {
                if !current_is_token {
                    count += 1;
                    prefix_space_len += 2; // space and current char

                    current_is_token = true;
                } else {
                    prefix_space_len += 1; // current char
                }
            }
            i += 1;
        }

        DomTokensInfo {
            count,
            prefix_space_len,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn dom_tokens_strings() {
        use super::make_dom_tokens_strings;

        assert_eq!(("", ""), make_dom_tokens_strings(""));
        assert_eq!(("", ""), make_dom_tokens_strings(" "));
        assert_eq!(("", ""), make_dom_tokens_strings("  "));

        assert_eq!((" a", "a"), make_dom_tokens_strings(" a"));
        assert_eq!((" a", "a"), make_dom_tokens_strings("  a"));
        assert_eq!((" a", "a"), make_dom_tokens_strings(" a "));
        assert_eq!((" a", "a"), make_dom_tokens_strings(" a  "));
        assert_eq!((" a", "a"), make_dom_tokens_strings("  a "));
        assert_eq!((" a", "a"), make_dom_tokens_strings("  a  "));

        assert_eq!((" a  b", "a  b"), make_dom_tokens_strings(" a  b"));
        assert_eq!((" a  b", "a  b"), make_dom_tokens_strings("  a  b"));
        assert_eq!((" a  b", "a  b"), make_dom_tokens_strings(" a  b "));
        assert_eq!((" a  b", "a  b"), make_dom_tokens_strings(" a  b  "));
        assert_eq!((" a  b", "a  b"), make_dom_tokens_strings("  a  b  "));
    }
}
