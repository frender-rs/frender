use crate::IntoStyle;

pub mod assert {
    use async_str_iter::AsyncStrIterator;

    mod sealed {
        pub trait DeclarationList {}
        pub trait DeclarationListPrefixSemicolon {}
    }

    /// A string stream which is either empty or
    /// a valid [Declaration](ccss::parse::declaration::Declaration) list separated by semicolon.
    pub trait DeclarationList: AsyncStrIterator + sealed::DeclarationList {}

    /// A string stream which is either empty or
    /// starts with a semicolon `;` and the rest is a [`SsrDeclarationList`].
    pub trait DeclarationListPrefixSemicolon:
        AsyncStrIterator + sealed::DeclarationListPrefixSemicolon
    {
    }

    impl sealed::DeclarationList for async_str_iter::empty::Empty {}
    impl DeclarationList for async_str_iter::empty::Empty {}

    impl sealed::DeclarationListPrefixSemicolon for async_str_iter::empty::Empty {}
    impl DeclarationListPrefixSemicolon for async_str_iter::empty::Empty {}

    impl sealed::DeclarationList for async_str_iter::never::Never {}
    impl DeclarationList for async_str_iter::never::Never {}

    impl sealed::DeclarationListPrefixSemicolon for async_str_iter::never::Never {}
    impl DeclarationListPrefixSemicolon for async_str_iter::never::Never {}

    impl<T: DeclarationList> sealed::DeclarationList for async_str_iter::option::IterOption<T> {}
    impl<T: DeclarationList> DeclarationList for async_str_iter::option::IterOption<T> {}

    impl<T: DeclarationListPrefixSemicolon> sealed::DeclarationListPrefixSemicolon
        for async_str_iter::option::IterOption<T>
    {
    }
    impl<T: DeclarationListPrefixSemicolon> DeclarationListPrefixSemicolon
        for async_str_iter::option::IterOption<T>
    {
    }

    impl<A: DeclarationList, B: DeclarationListPrefixSemicolon> sealed::DeclarationList
        for async_str_iter::chain::Chain<A, B>
    {
    }
    impl<A: DeclarationList, B: DeclarationListPrefixSemicolon> DeclarationList
        for async_str_iter::chain::Chain<A, B>
    {
    }

    impl<A: DeclarationListPrefixSemicolon, B: DeclarationListPrefixSemicolon>
        sealed::DeclarationListPrefixSemicolon for async_str_iter::chain::Chain<A, B>
    {
    }
    // let a: impl DeclarationListPrefixSemicolon;
    // let b: impl DeclarationListPrefixSemicolon;
    // let this = a + b;
    // - If `a` is empty, then `this` is `b`,
    //   which is a valid DeclarationListPrefixSemicolon.
    // - Else, `a` starts with a semicolon, so
    //   `a + b` starts with a semicolon, which is a valid DeclarationListPrefixSemicolon.
    impl<A: DeclarationListPrefixSemicolon, B: DeclarationListPrefixSemicolon>
        DeclarationListPrefixSemicolon for async_str_iter::chain::Chain<A, B>
    {
    }

    impl<A: DeclarationList, B: DeclarationList> sealed::DeclarationList
        for async_str_iter::either::IterEither<A, B>
    {
    }
    impl<A: DeclarationList, B: DeclarationList> DeclarationList
        for async_str_iter::either::IterEither<A, B>
    {
    }

    impl<A: DeclarationListPrefixSemicolon, B: DeclarationListPrefixSemicolon>
        sealed::DeclarationListPrefixSemicolon for async_str_iter::either::IterEither<A, B>
    {
    }
    impl<A: DeclarationListPrefixSemicolon, B: DeclarationListPrefixSemicolon>
        DeclarationListPrefixSemicolon for async_str_iter::either::IterEither<A, B>
    {
    }

    impl<T: ?Sized + crate::styles::constness::HasConstDeclarationList> sealed::DeclarationList
        for crate::styles::constness::ssr::ConstDeclarationListIntoSsr<T>
    {
    }
    impl<T: ?Sized + crate::styles::constness::HasConstDeclarationList> DeclarationList
        for crate::styles::constness::ssr::ConstDeclarationListIntoSsr<T>
    {
    }

    impl<T: ?Sized + crate::styles::constness::HasConstDeclarationList>
        sealed::DeclarationListPrefixSemicolon
        for crate::styles::constness::ssr::ConstDeclarationListIntoSsrPrefixSemicolon<T>
    {
    }
    impl<T: ?Sized + crate::styles::constness::HasConstDeclarationList>
        DeclarationListPrefixSemicolon
        for crate::styles::constness::ssr::ConstDeclarationListIntoSsrPrefixSemicolon<T>
    {
    }

    impl<
            // OneDeclarationAsList can only be constructed from valid name and value
            // Thus, the bound is just `: AsyncStrIterator`.
            N: AsyncStrIterator,
            V: AsyncStrIterator,
            I: crate::declaration::important::ssr::assert::BangImportantOrEmpty,
        > sealed::DeclarationList
        for crate::styles::declaration::ssr::OneDeclarationAsList<N, V, I>
    {
    }
    impl<
            N: AsyncStrIterator,
            V: AsyncStrIterator,
            I: crate::declaration::important::ssr::assert::BangImportantOrEmpty,
        > DeclarationList for crate::styles::declaration::ssr::OneDeclarationAsList<N, V, I>
    {
    }

    impl<
            // OneDeclarationAsListPrefixSemicolon can only be constructed from valid name and value
            // Thus, the bound is just `: AsyncStrIterator`.
            N: AsyncStrIterator,
            V: AsyncStrIterator,
            I: crate::declaration::important::ssr::assert::BangImportantOrEmpty,
        > sealed::DeclarationListPrefixSemicolon
        for crate::styles::declaration::ssr::OneDeclarationAsListPrefixSemicolon<N, V, I>
    {
    }
    impl<
            N: AsyncStrIterator,
            V: AsyncStrIterator,
            I: crate::declaration::important::ssr::assert::BangImportantOrEmpty,
        > DeclarationListPrefixSemicolon
        for crate::styles::declaration::ssr::OneDeclarationAsListPrefixSemicolon<N, V, I>
    {
    }
}

mod sealed {
    pub trait SsrDeclarationList {}
}

/// This trait is sealed to make sure <code>
/// [SsrDeclarationList::IntoDeclarationListPrefixSemicolon] == ";" + [SsrDeclarationList::IntoDeclarationList]
/// </code> if [SsrDeclarationList::IntoDeclarationList] is not empty;
pub trait SsrDeclarationList: sealed::SsrDeclarationList {
    type IntoDeclarationList: assert::DeclarationList;
    type IntoDeclarationListPrefixSemicolon: assert::DeclarationListPrefixSemicolon;

    fn into_declaration_list(this: Self) -> Self::IntoDeclarationList;

    fn into_declaration_list_prefix_semicolon(
        this: Self,
    ) -> Self::IntoDeclarationListPrefixSemicolon;
}

mod imp {
    use async_str_iter::IntoAsyncStrIterator;

    use super::{sealed, SsrDeclarationList};

    impl sealed::SsrDeclarationList for frender_common::Empty {}
    impl SsrDeclarationList for frender_common::Empty {
        type IntoDeclarationList = async_str_iter::empty::Empty;
        type IntoDeclarationListPrefixSemicolon = async_str_iter::empty::Empty;

        fn into_declaration_list(Self: Self) -> Self::IntoDeclarationList {
            async_str_iter::empty::Empty
        }

        fn into_declaration_list_prefix_semicolon(
            Self: Self,
        ) -> Self::IntoDeclarationListPrefixSemicolon {
            async_str_iter::empty::Empty
        }
    }

    impl<T: SsrDeclarationList> sealed::SsrDeclarationList for Option<T> {}

    impl<T: SsrDeclarationList> SsrDeclarationList for Option<T> {
        type IntoDeclarationList = async_str_iter::option::IterOption<T::IntoDeclarationList>;
        type IntoDeclarationListPrefixSemicolon =
            async_str_iter::option::IterOption<T::IntoDeclarationListPrefixSemicolon>;

        fn into_declaration_list(this: Self) -> Self::IntoDeclarationList {
            this.map(T::into_declaration_list).into_async_str_iterator()
        }

        fn into_declaration_list_prefix_semicolon(
            this: Self,
        ) -> Self::IntoDeclarationListPrefixSemicolon {
            this.map(T::into_declaration_list_prefix_semicolon)
                .into_async_str_iterator()
        }
    }

    impl<A: SsrDeclarationList, B: SsrDeclarationList> sealed::SsrDeclarationList
        for crate::styles::Chain<A, B>
    {
    }
    impl<A: SsrDeclarationList, B: SsrDeclarationList> SsrDeclarationList
        for crate::styles::Chain<A, B>
    {
        type IntoDeclarationList = async_str_iter::chain::Chain<
            A::IntoDeclarationList,
            B::IntoDeclarationListPrefixSemicolon,
        >;

        type IntoDeclarationListPrefixSemicolon = async_str_iter::chain::Chain<
            A::IntoDeclarationListPrefixSemicolon,
            B::IntoDeclarationListPrefixSemicolon,
        >;

        fn into_declaration_list(Self(a, b): Self) -> Self::IntoDeclarationList {
            async_str_iter::chain::Chain::new(
                A::into_declaration_list(a),
                B::into_declaration_list_prefix_semicolon(b),
            )
        }

        fn into_declaration_list_prefix_semicolon(
            Self(a, b): Self,
        ) -> Self::IntoDeclarationListPrefixSemicolon {
            async_str_iter::chain::Chain::new(
                A::into_declaration_list_prefix_semicolon(a),
                B::into_declaration_list_prefix_semicolon(b),
            )
        }
    }

    impl<A: SsrDeclarationList, B: SsrDeclarationList> sealed::SsrDeclarationList
        for crate::styles::either::ssr::EitherSsrDeclarationList<A, B>
    {
    }
    impl<A: SsrDeclarationList, B: SsrDeclarationList> SsrDeclarationList
        for crate::styles::either::ssr::EitherSsrDeclarationList<A, B>
    {
        type IntoDeclarationList =
            async_str_iter::either::IterEither<A::IntoDeclarationList, B::IntoDeclarationList>;

        type IntoDeclarationListPrefixSemicolon = async_str_iter::either::IterEither<
            A::IntoDeclarationListPrefixSemicolon,
            B::IntoDeclarationListPrefixSemicolon,
        >;

        fn into_declaration_list(this: Self) -> Self::IntoDeclarationList {
            match this {
                Self::A(this) => {
                    async_str_iter::either::IterEither::Left(A::into_declaration_list(this))
                }
                Self::B(this) => {
                    async_str_iter::either::IterEither::Right(B::into_declaration_list(this))
                }
            }
        }

        fn into_declaration_list_prefix_semicolon(
            this: Self,
        ) -> Self::IntoDeclarationListPrefixSemicolon {
            match this {
                Self::A(this) => async_str_iter::either::IterEither::Left(
                    A::into_declaration_list_prefix_semicolon(this),
                ),
                Self::B(this) => async_str_iter::either::IterEither::Right(
                    B::into_declaration_list_prefix_semicolon(this),
                ),
            }
        }
    }

    impl<T: ?Sized + crate::styles::constness::HasConstDeclarationList> sealed::SsrDeclarationList
        for crate::styles::constness::ConstDeclarationList<T>
    {
    }

    impl sealed::SsrDeclarationList for crate::styles::Never {}

    impl<D: crate::declaration::IntoDeclaration> sealed::SsrDeclarationList
        for crate::styles::declaration::IntoDeclarationAsStyle<D>
    {
    }
}

pub trait SsrStyle {
    type IntoSsrDeclarationList: SsrDeclarationList;
    fn into_ssr_declaration_list(this: Self) -> Self::IntoSsrDeclarationList;
}

impl<S: IntoStyle> SsrStyle for S {
    type IntoSsrDeclarationList = <S::IntoStyle as SsrStyle>::IntoSsrDeclarationList;

    fn into_ssr_declaration_list(this: Self) -> Self::IntoSsrDeclarationList {
        <S::IntoStyle>::into_ssr_declaration_list(this.into_style())
    }
}
