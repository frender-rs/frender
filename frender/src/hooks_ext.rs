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

pub mod setter {
    use frender_events::callable::{
        ArgumentTypes, Callable, CallableWithFixedArguments, IsCallable,
    };
    use hooks::ShareValue;

    #[derive(Debug, Clone)]
    pub struct Setter<S: ShareValue>(pub S);

    #[derive(Debug, Clone)]
    pub struct SetterConditional<S: ShareValue>(pub S);

    impl<S: ShareValue> PartialEq for Setter<S> {
        fn eq(&self, other: &Self) -> bool {
            S::equivalent_to(&self.0, &other.0)
        }
    }

    impl<S: ShareValue> IsCallable for Setter<S> {}

    impl<S: ShareValue> Callable<(S::Value,)> for Setter<S> {
        type Output = ();

        fn call_fn(&self, (new_value,): (S::Value,)) -> Self::Output {
            self.0.set(new_value)
        }
    }

    impl<S: ShareValue> CallableWithFixedArguments for Setter<S> {
        type FixedArgumentTypes = ArgumentTypes!(S::Value,);
    }

    impl<S: ShareValue> PartialEq for SetterConditional<S> {
        fn eq(&self, other: &Self) -> bool {
            S::equivalent_to(&self.0, &other.0)
        }
    }

    impl<S: ShareValue> IsCallable for SetterConditional<S> {}

    impl<S: ShareValue> Callable<(Option<S::Value>,)> for SetterConditional<S> {
        type Output = ();

        fn call_fn(&self, (new_value,): (Option<S::Value>,)) -> Self::Output {
            if let Some(new_value) = new_value {
                self.0.set(new_value)
            }
        }
    }

    impl<S: ShareValue> CallableWithFixedArguments for SetterConditional<S> {
        type FixedArgumentTypes = ArgumentTypes!(Option<S::Value>,);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct EventTargetFormControlValue;

    impl IsCallable for EventTargetFormControlValue {}
    impl<E: ?Sized + frender_events::event::Event> Callable<(&E,)> for EventTargetFormControlValue {
        type Output = Option<String>;

        fn call_fn(&self, (e,): (&E,)) -> Self::Output {
            e.target_form_control_value()
                .map(std::borrow::Cow::into_owned)
        }
    }
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

    fn into_setter(self) -> setter::Setter<Self>
    where
        Self: Sized,
    {
        setter::Setter(self)
    }

    fn into_setter_conditional(self) -> setter::SetterConditional<Self>
    where
        Self: Sized,
    {
        setter::SetterConditional(self)
    }

    fn into_setter_form_control_value(
        self,
    ) -> callable::chain::Chain<setter::EventTargetFormControlValue, setter::SetterConditional<Self>>
    where
        Self: Sized + ShareValue<Value = String>,
    {
        callable::chain::Chain(
            setter::EventTargetFormControlValue,
            setter::SetterConditional(self),
        )
    }
}

impl<S: ?Sized> ShareValueExt for S where S: ShareValue {}
