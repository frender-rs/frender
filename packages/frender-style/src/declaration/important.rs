use frender_common::Empty;
use frender_reactive_value::non_reactive::Uncached;

mod sealed {
    pub trait IntoDeclarationImportant {}
}

#[cfg(feature = "ssr")]
#[cfg(feature = "csr")]
pub trait IntoDeclarationImportant:
    sealed::IntoDeclarationImportant
    + ssr::IntoSsrDeclarationImportant
    + csr::IntoCsrDeclarationImportant
{
}

#[cfg(not(feature = "ssr"))]
#[cfg(feature = "csr")]
pub trait IntoDeclarationImportant:
    sealed::IntoDeclarationImportant + csr::IntoCsrDeclarationImportant
{
}

#[cfg(feature = "ssr")]
#[cfg(not(feature = "csr"))]
pub trait IntoDeclarationImportant:
    sealed::IntoDeclarationImportant + ssr::IntoSsrDeclarationImportant
{
}

#[cfg(not(feature = "ssr"))]
#[cfg(not(feature = "csr"))]
pub trait IntoDeclarationImportant: sealed::IntoDeclarationImportant {}

impl sealed::IntoDeclarationImportant for Empty {}
impl IntoDeclarationImportant for Empty {}

impl sealed::IntoDeclarationImportant for bool {}
impl IntoDeclarationImportant for bool {}

impl sealed::IntoDeclarationImportant for Uncached<bool> {}
impl IntoDeclarationImportant for Uncached<bool> {}

// TODO: impl for AlwaysTrue

#[cfg(feature = "ssr")]
pub(crate) mod ssr {
    use frender_reactive_value::non_reactive::Uncached;

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

    impl IntoSsrDeclarationImportant for Uncached<bool> {
        type BangImportant = <bool as IntoSsrDeclarationImportant>::BangImportant;

        fn into_ssr_declaration_important(self) -> Self::BangImportant {
            self.0.into_ssr_declaration_important()
        }
    }
}
#[cfg(feature = "csr")]
pub(crate) mod csr;
