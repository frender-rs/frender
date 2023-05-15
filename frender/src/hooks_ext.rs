use frender_events::callable::{self, ArgumentType, CallableWithFixedArguments};
use hooks::ShareValue;

pub mod argument {
    macro_rules! wrap_share_value {
        ($(
            $vis:vis struct $name:ident
            <$tp:ident>
            ($fn_name:ident ![
                |$this:ident, $f:ident|
                -> $arg_ty:tt
                $impl_block:block
            ] $(,)?);
        )*) => {$(
            #[derive(Debug, Clone, Copy)]
            $vis struct $name<$tp: ::hooks::ShareValue>(pub(super) $tp);

            impl<$tp: ::hooks::ShareValue> PartialEq for $name<$tp> {
                fn eq(&self, other: &Self) -> bool {
                    self.0.equivalent_to(&other.0)
                }
            }

            impl<$tp: ::hooks::ShareValue> ::frender_events::callable::argument::ProvideArgument for $name<$tp> {
                type ProvideArgumentType = ::frender_events::callable::ArgumentType! $arg_ty;

                fn $fn_name<
                    Out,
                    F: for<'arg> FnOnce(
                        ::frender_events::callable::argument::ArgumentOfType<'arg, Self::ProvideArgumentType>,
                    ) -> Out,
                >(
                    &$this,
                    $f: F,
                ) -> Out
                    $impl_block
            }
        )*};
    }

    wrap_share_value!(
        pub struct Shared<S>(provide_argument_to![|self, f| -> (&S) { f(&self.0) }]);
        pub struct Mapped<S>(provide_argument_to![|self, f| -> (&S::Value) { self.0.map(f) }]);
        pub struct MappedMut<S>(
            provide_argument_to![|self, f| -> (&mut S::Value) { self.0.map_mut(f) }],
        );
    );
}

pub trait ShareValueExt: ShareValue {
    fn into_argument_shared(self) -> argument::Shared<Self>
    where
        Self: Sized,
    {
        argument::Shared(self)
    }

    fn into_argument_mapped(self) -> argument::Mapped<Self>
    where
        Self: Sized,
    {
        argument::Mapped(self)
    }

    fn into_argument_mapped_mut(self) -> argument::MappedMut<Self>
    where
        Self: Sized,
    {
        argument::MappedMut(self)
    }

    fn into_callback<F>(
        self,
        f: F,
    ) -> callable::argument::FirstArgumentProvided<F, argument::Shared<Self>>
    where
        Self: Sized,
        F: CallableWithFixedArguments,
        F::FixedArgumentTypes: callable::argument::ArgumentTypes<First = ArgumentType![&Self]>,
    {
        f.provide_first_argument(argument::Shared(self))
    }

    fn into_callback_map<F>(
        self,
        f: F,
    ) -> callable::argument::FirstArgumentProvided<F, argument::Mapped<Self>>
    where
        Self: Sized,
        F: CallableWithFixedArguments,
        F::FixedArgumentTypes:
            callable::argument::ArgumentTypes<First = ArgumentType![&Self::Value]>,
    {
        f.provide_first_argument(argument::Mapped(self))
    }

    fn into_callback_map_mut<F>(
        self,
        f: F,
    ) -> callable::argument::FirstArgumentProvided<F, argument::MappedMut<Self>>
    where
        Self: Sized,
        F: CallableWithFixedArguments,
        F::FixedArgumentTypes:
            callable::argument::ArgumentTypes<First = ArgumentType![&mut Self::Value]>,
    {
        f.provide_first_argument(argument::MappedMut(self))
    }
}

impl<S: ?Sized> ShareValueExt for S where S: ShareValue {}
