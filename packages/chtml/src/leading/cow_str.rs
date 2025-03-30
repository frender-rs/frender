use super::string::LeadingString;

#[derive(Debug, Clone, Copy)]
pub(crate) enum LeadingCowStr<'a, const CAP: usize> {
    /// Whole. No need to decode.
    Borrowed(&'a str),
    /// Whole or leading. Decoded.
    Owned(LeadingString<CAP>),
}

impl<const CAP: usize> LeadingCowStr<'_, CAP> {
    pub(crate) const fn new() -> Self {
        Self::Borrowed("")
    }

    pub const fn len(&self) -> usize {
        match self {
            LeadingCowStr::Borrowed(s) => s.len(),
            LeadingCowStr::Owned(s) => s.len(),
        }
    }

    pub const fn is_empty(&self) -> bool {
        match self {
            LeadingCowStr::Borrowed(s) => s.is_empty(),
            LeadingCowStr::Owned(s) => s.is_empty(),
        }
    }

    pub(crate) const fn push(&mut self, ch: char) {
        self.make_mut_owned().push(ch)
    }

    const fn push_str(&mut self, s: &str) {
        self.make_mut_owned().push_str(s)
    }

    const fn make_mut_owned(&mut self) -> &mut LeadingString<CAP> {
        if let LeadingCowStr::Borrowed(v) = self {
            let mut this = LeadingString::new();
            this.push_str(v);
            *self = Self::Owned(this);
        }

        let LeadingCowStr::Owned(this) = self else {
            unreachable!()
        };

        this
    }

    pub const fn try_as_str(&self) -> Option<&str> {
        match self {
            LeadingCowStr::Borrowed(s) => Some(s),
            LeadingCowStr::Owned(s) => s.try_as_str(),
        }
    }

    /// Panics if self doesn't contain the whole string.
    pub const fn as_str(&self) -> &str {
        match self {
            LeadingCowStr::Borrowed(s) => s,
            LeadingCowStr::Owned(s) => s.as_str(),
        }
    }

    const fn push_leading_string<const CAP2: usize>(&mut self, s: &LeadingString<CAP2>) {
        self.make_mut_owned().push_leading_string(s)
    }
}

impl<'a, const CAP: usize> LeadingCowStr<'a, CAP> {
    pub(crate) const fn as_long_live_str(&self) -> Result<&'a str, &LeadingString<CAP>> {
        match self {
            LeadingCowStr::Borrowed(s) => Ok(s),
            LeadingCowStr::Owned(s) => {
                if s.is_empty() {
                    Ok("")
                } else {
                    Err(s)
                }
            }
        }
    }

    pub(crate) const fn push_long_live_str(&mut self, s: &'a str) {
        if self.is_empty() {
            *self = Self::Borrowed(s)
        } else {
            self.push_str(s)
        }
    }

    pub(crate) const fn push_long_live_leading_cow_str<const CAP2: usize>(
        &mut self,
        s: &LeadingCowStr<'a, CAP2>,
    ) {
        match s.as_long_live_str() {
            Ok(s) => self.push_long_live_str(s),
            Err(s) => self.push_leading_string(s),
        }
    }
}
