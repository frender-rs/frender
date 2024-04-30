use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub struct DomToken<'a>(&'a str);

impl<'a> Deref for DomToken<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
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

    pub const fn new_array_const<const N: usize>(arr: [&'a str; N]) -> [Self; N] {
        let mut res = [Self(""); N];

        let mut i = 0;
        while i < N {
            assert_valid_dom_token(arr[i]);
            i += 1;
        }

        res
    }

    pub const fn as_str(self) -> &'a str {
        self.0
    }
}

// TODO: remove
pub const fn assert_valid_dom_tokens(tokens: &[&str]) {
    let mut i = 0;
    while i < tokens.len() {
        assert_valid_dom_token(tokens[i]);
        i += 1;
    }
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
    mut arr: [DomToken<'a>; N],
    mut at: usize,
    tokens: &[DomToken<'a>],
) -> ([DomToken<'a>; N], usize) {
    let mut j = 0;
    while j < tokens.len() {
        arr[at] = tokens[j];
        at += 1;
        j += 1;
    }

    (arr, at)
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
}

impl<'a, const N: usize> Deref for UniqueDomTokenArray<'a, N> {
    type Target = [DomToken<'a>; N];

    fn deref(&self) -> &Self::Target {
        &self.0
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
