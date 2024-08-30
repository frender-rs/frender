#[derive(Debug, Clone, Copy)]
pub struct ArrayString<const CAP: usize = 64> {
    array: [u8; CAP],
    len: usize,
}

impl<const CAP: usize> ArrayString<CAP> {
    const EMPTY: Self = Self {
        array: [0; CAP],
        len: 0,
    };
    pub const fn new() -> Self {
        Self::EMPTY
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn as_str(&self) -> &str {
        // Choose safety over performance
        match std::str::from_utf8(self.as_bytes()) {
            Ok(v) => v,
            Err(_) => unreachable!(),
        }
    }

    const fn as_bytes(&self) -> &[u8] {
        self.array.split_at(self.len).0
    }

    const fn with_try_push_str(mut self, s: &str) -> Result<Self, Self> {
        let new_len = self.len + s.len();
        if new_len <= CAP {
            let at;
            (self.array, at) = super::put_at(self.array, self.len, s.as_bytes());

            debug_assert!(at == new_len);

            self.len = new_len;

            Ok(self)
        } else {
            Err(self)
        }
    }

    pub const fn with_push_str(self, s: &str) -> Self {
        match self.with_try_push_str(s) {
            Ok(this) => this,
            Err(_) => panic!("capacity overflow"),
        }
    }

    pub const fn with_push_str_or_elide(self, s: &str) -> Self {
        match self.with_try_push_str(s) {
            Ok(this) => this,
            Err(mut this) => {
                let remaining_capacity = this.remaining_capacity();

                if remaining_capacity > 2 {
                    (this.array, this.len) = super::put_at(
                        this.array,
                        this.len,
                        s.as_bytes().split_at(remaining_capacity - 2).0,
                    );
                }

                while this.len < CAP {
                    this.array[this.len] = b'.';
                    this.len += 1;
                }
                this
            }
        }
    }

    pub const fn remaining_capacity(&self) -> usize {
        CAP - self.len
    }

    pub const fn try_into_filled_bytes(self) -> Result<[u8; CAP], Self> {
        if self.len == CAP {
            Ok(self.array)
        } else {
            Err(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayString;

    #[test]
    fn elide() {
        assert_eq!(
            ArrayString::<10>::new()
                .with_push_str_or_elide("012345678")
                .as_str(),
            "012345678"
        );
        assert_eq!(
            ArrayString::<10>::new()
                .with_push_str_or_elide("0123456789")
                .as_str(),
            "0123456789"
        );
        assert_eq!(
            ArrayString::<10>::new()
                .with_push_str_or_elide("0123456789*")
                .as_str(),
            "01234567.."
        );
    }
}
