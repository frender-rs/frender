use const_array_string::ArrayString;

#[derive(Clone, Copy)]
pub(crate) struct LeadingString<const CAP: usize> {
    len: usize,
    /// ```compile_fail
    /// if let Some(whole_or_leading) = self.whole_or_leading {
    ///     if CAP >= self.len { whole_str + padding_bytes } else { leading_bytes }
    /// } else {
    ///     unknown
    /// }
    /// ```
    whole_or_leading: Option<[u8; CAP]>,
}

impl<const CAP: usize> std::fmt::Debug for LeadingString<CAP> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.try_as_str() {
            Some(v) => v,
            None => "unknown",
        }
        .fmt(f)
    }
}

const fn unwrap_whole<Whole>(v: Option<Whole>) -> Whole
where
    Whole: Copy,
{
    match v {
        Some(v) => v,
        None => panic!("LeadingString doesn't contain the whole string"),
    }
}

impl<const CAP: usize> LeadingString<CAP> {
    pub const fn new() -> Self {
        Self {
            len: 0,
            whole_or_leading: Some([0; CAP]),
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub(crate) const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(crate) const fn try_to_array_string(self) -> Option<ArrayString<CAP>> {
        match self.try_as_str() {
            Some(whole) => Some({
                let mut res = ArrayString::new();
                res.push_str(whole);
                res
            }),
            None => None,
        }
    }

    /// Panics if self doesn't contain the whole string.
    pub(crate) const fn to_array_string(self) -> ArrayString<CAP> {
        unwrap_whole(self.try_to_array_string())
    }

    pub const fn try_as_str(&self) -> Option<&str> {
        match self.as_whole_or_leading_str() {
            Some(WholeOrLeadingStr::Whole(s)) => Some(s),
            _ => None,
        }
    }

    /// Panics if self doesn't contain the whole string.
    pub const fn as_str(&self) -> &str {
        unwrap_whole(self.try_as_str())
    }

    const fn as_whole_or_leading_str(&self) -> Option<WholeOrLeadingStr<'_>> {
        if let Some(whole_or_leading) = &self.whole_or_leading {
            Some(if CAP >= self.len {
                let (whole, _) = whole_or_leading.split_at(self.len);
                WholeOrLeadingStr::Whole(match ::core::str::from_utf8(whole) {
                    Ok(v) => v,
                    Err(_) => unreachable!(),
                })
            } else {
                WholeOrLeadingStr::Leading(whole_or_leading)
            })
        } else {
            None
        }
    }

    /// [`String::push`]
    pub(crate) const fn push(&mut self, ch: char) {
        self.push_str(ch.encode_utf8(&mut [0; 4]))
    }

    /// [`String::push_str`]
    pub(crate) const fn push_str(&mut self, s: &str) {
        match &mut self.whole_or_leading {
            Some(whole_or_leading) => match whole_or_leading.split_at_mut_checked(self.len) {
                Some((_, padding)) => {
                    // push as much as possible
                    let mut i = 0usize;

                    let s = s.as_bytes();
                    while i < s.len() && i < padding.len() {
                        padding[i] = s[i];
                        i += 1;
                    }
                }
                None => {
                    // skip pushing
                }
            },
            None => {
                // skip pushing
            }
        }

        self.len += s.len();
    }

    pub(crate) const fn push_leading_string<const CAP2: usize>(&mut self, s: &LeadingString<CAP2>) {
        if s.is_empty() {
            return;
        }

        match &mut self.whole_or_leading {
            Some(whole_or_leading) => match whole_or_leading.split_at_mut_checked(self.len) {
                Some((_, padding)) => {
                    // push as much as possible

                    match s.as_whole_or_leading_str() {
                        Some(WholeOrLeadingStr::Whole(s)) => {
                            let s = s.as_bytes();
                            let mut i = 0usize;
                            while i < s.len() && i < padding.len() {
                                padding[i] = s[i];
                                i += 1;
                            }
                        }
                        Some(WholeOrLeadingStr::Leading(s)) => {
                            if s.len() >= padding.len() {
                                // known part of s can fill padding
                                let mut i = 0usize;
                                while i < padding.len() {
                                    padding[i] = s[i];
                                    i += 1;
                                }
                            } else {
                                self.whole_or_leading = None;
                            }
                        }
                        None => {
                            self.whole_or_leading = None;
                        }
                    }
                }
                None => {
                    // skip pushing
                }
            },
            None => {
                // skip pushing
            }
        }

        self.len += s.len();
    }
}

enum WholeOrLeadingStr<'a> {
    Whole(&'a str),
    Leading(&'a [u8]),
}
