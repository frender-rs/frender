pub trait IntoDeclarationImportant:
    ssr::IntoSsrDeclarationImportant + csr::CsrDeclarationImportant
{
}

impl<T: ssr::IntoSsrDeclarationImportant + csr::CsrDeclarationImportant> IntoDeclarationImportant
    for T
{
}

pub mod ssr {
    async_str_iter::Strings!(
        enum BangImportantState {}
        pub struct BangImportant(bang_important!("!important"));
    );

    pub mod assert {
        use async_str_iter::AsyncStrIterator;

        mod sealed {
            pub trait BangImportantOrEmpty {}
        }

        pub trait BangImportantOrEmpty: AsyncStrIterator + sealed::BangImportantOrEmpty {}

        impl sealed::BangImportantOrEmpty for async_str_iter::empty::Empty {}
        impl BangImportantOrEmpty for async_str_iter::empty::Empty {}

        impl sealed::BangImportantOrEmpty for super::BangImportant {}
        impl BangImportantOrEmpty for super::BangImportant {}

        impl<T: BangImportantOrEmpty> sealed::BangImportantOrEmpty
            for async_str_iter::option::IterOption<T>
        {
        }
        impl<T: BangImportantOrEmpty> BangImportantOrEmpty for async_str_iter::option::IterOption<T> {}

        impl<L: BangImportantOrEmpty, R: BangImportantOrEmpty> sealed::BangImportantOrEmpty
            for async_str_iter::either::IterEither<L, R>
        {
        }
        impl<L: BangImportantOrEmpty, R: BangImportantOrEmpty> BangImportantOrEmpty
            for async_str_iter::either::IterEither<L, R>
        {
        }
    }

    pub trait IntoSsrDeclarationImportant {
        type BangImportant: assert::BangImportantOrEmpty;
        fn into_ssr_declaration_important(self) -> Self::BangImportant;
    }

    impl IntoSsrDeclarationImportant for frender_common::Empty {
        type BangImportant = async_str_iter::empty::Empty;

        fn into_ssr_declaration_important(self) -> Self::BangImportant {
            async_str_iter::empty::Empty
        }
    }

    impl IntoSsrDeclarationImportant for bool {
        type BangImportant = BangImportant; // This is an optimized version of async_str_iter::option::IterOption<BangImportant>.

        fn into_ssr_declaration_important(self) -> Self::BangImportant {
            let state = if self {
                BangImportantState::bang_important
            } else {
                BangImportantState::__AllDone
            };
            BangImportant {
                _state: state,
                bang_important: (),
            }
        }
    }
}

pub mod csr {
    use crate::csr::Priority;

    pub trait CsrDeclarationImportant {
        type StaticCache: 'static;

        fn match_cache(&self, cache: &Self::StaticCache) -> bool;

        fn into_static_cache(self) -> Self::StaticCache;
        fn update_into_cache(self, cache: &mut Self::StaticCache);

        fn update_style(&self, style: impl UpdateStyleWithDeclarationImportant);
    }

    impl CsrDeclarationImportant for frender_common::Empty {
        type StaticCache = ();

        fn match_cache(&self, (): &Self::StaticCache) -> bool {
            true
        }

        fn into_static_cache(self) -> Self::StaticCache {}

        fn update_into_cache(self, (): &mut Self::StaticCache) {}

        fn update_style(&self, style: impl UpdateStyleWithDeclarationImportant) {
            style.update_not_important()
        }
    }

    impl CsrDeclarationImportant for bool {
        type StaticCache = Self;

        fn match_cache(&self, cache: &Self::StaticCache) -> bool {
            self == cache
        }

        fn into_static_cache(self) -> Self::StaticCache {
            self
        }

        fn update_into_cache(self, cache: &mut Self::StaticCache) {
            *cache = self
        }

        fn update_style(&self, style: impl UpdateStyleWithDeclarationImportant) {
            style.update_with_priority(Priority::from_bool(*self))
        }
    }

    pub trait UpdateStyleWithDeclarationImportant {
        fn update_not_important(self);
        fn update_with_priority(self, priority: Priority);
    }
}

// TODO: impl for AlwaysTrue
