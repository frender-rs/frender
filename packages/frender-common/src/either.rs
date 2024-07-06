pub enum EitherState<L, R> {
    Left { inner: L },
    Right { inner: R },
}

impl<L, R> EitherState<L, R> {
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn Left(inner: L) -> Self {
        Self::Left { inner }
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn Right(inner: R) -> Self {
        Self::Right { inner }
    }

    pub fn get_left_or_insert_default(&mut self) -> &mut L
    where
        L: Default,
    {
        match self {
            EitherState::Left { inner: this } => this,
            this @ EitherState::Right { inner: _ } => {
                *this = Self::Left {
                    inner: Default::default(),
                };
                if let Self::Left { inner: this } = this {
                    this
                } else {
                    unreachable!()
                }
            }
        }
    }

    pub fn get_right_or_insert_default(&mut self) -> &mut R
    where
        R: Default,
    {
        match self {
            EitherState::Right { inner: this } => this,
            this @ EitherState::Left { inner: _ } => {
                *this = Self::Right {
                    inner: Default::default(),
                };
                if let Self::Right { inner: this } = this {
                    this
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl<L: Default, R: Default> Default for EitherState<L, R> {
    fn default() -> Self {
        Self::Left {
            inner: Default::default(),
        }
    }
}
