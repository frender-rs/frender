use std::marker::PhantomData;

pub use first::FirstArgumentProvided;
pub use last::LastArgumentProvided;
pub use ByMut as r#mut;
pub use ByRef as r#ref;
pub use Value as value;

pub trait ArgumentType<'a>: 'static {
    type Argument;
}
pub trait ProvideArgument {
    type ProvideArgumentType: ?Sized + for<'a> ArgumentType<'a>;

    // fn provide_argument(&self) -> <Self::ProvideArgumentType as ArgumentType<'_>>::Argument;

    fn provide_argument_to<
        Out,
        F: for<'arg> FnOnce(<Self::ProvideArgumentType as ArgumentType<'arg>>::Argument) -> Out,
    >(
        &self,
        f: F,
    ) -> Out;
}

pub struct Value<T>(PhantomData<T>);
pub struct ByRef<T: ?Sized>(PhantomData<T>);
pub struct ByMut<T: ?Sized>(PhantomData<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Copied<T: Copy>(pub(super) T);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cloned<T: Clone>(pub(super) T);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Refed<T>(pub(super) T);

impl<'a, T: 'static> ArgumentType<'a> for Value<T> {
    type Argument = T;
}

impl<'a, T: ?Sized + 'static> ArgumentType<'a> for ByRef<T> {
    type Argument = &'a T;
}

impl<'a, T: ?Sized + 'static> ArgumentType<'a> for ByMut<T> {
    type Argument = &'a mut T;
}

impl<T: Copy + 'static> ProvideArgument for Copied<T> {
    type ProvideArgumentType = Value<T>;

    // fn provide_argument(&self) -> <Value<T> as ArgumentType<'_>>::Argument {
    //     self.0
    // }

    fn provide_argument_to<
        Out,
        F: for<'arg> FnOnce(<Self::ProvideArgumentType as ArgumentType<'arg>>::Argument) -> Out,
    >(
        &self,
        f: F,
    ) -> Out {
        f(self.0)
    }
}
impl<T: Clone + 'static> ProvideArgument for Cloned<T> {
    type ProvideArgumentType = Value<T>;

    // fn provide_argument(&self) -> <Value<T> as ArgumentType<'_>>::Argument {
    //     self.0.clone()
    // }

    fn provide_argument_to<
        Out,
        F: for<'arg> FnOnce(<Self::ProvideArgumentType as ArgumentType<'arg>>::Argument) -> Out,
    >(
        &self,
        f: F,
    ) -> Out {
        f(self.0.clone())
    }
}

impl<T: 'static> ProvideArgument for Refed<T> {
    type ProvideArgumentType = ByRef<T>;

    // fn provide_argument(&self) -> &T {
    //     &self.0
    // }

    fn provide_argument_to<
        Out,
        F: for<'arg> FnOnce(<Self::ProvideArgumentType as ArgumentType<'arg>>::Argument) -> Out,
    >(
        &self,
        f: F,
    ) -> Out {
        f(&self.0)
    }
}

mod last {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LastArgumentProvided<
        F: super::super::CallableWithFixedArguments,
        A: ProvideArgument<ProvideArgumentType = <F::FixedArgumentTypes as ArgumentTypes>::Last>,
    > {
        pub(crate) f: F,
        pub(crate) last_argument: A,
    }

    impl<
            F: super::super::CallableWithFixedArguments,
            A: ProvideArgument<ProvideArgumentType = <F::FixedArgumentTypes as ArgumentTypes>::Last>,
        > crate::callback::IsCallable for LastArgumentProvided<F, A>
    {
    }

    // FIXME: Output should be <F as super::Callable<ArgumentsOfTypes<'_, F::FixedArgumentTypes>>>::Output
    impl<
            Out,
            F: super::super::CallableWithFixedArguments<Output = Out>,
            A: ProvideArgument<ProvideArgumentType = <F::FixedArgumentTypes as ArgumentTypes>::Last>,
        >
        crate::callback::Callable<
            ArgumentsOfTypes<'_, <F::FixedArgumentTypes as ArgumentTypes>::LastTrimmed>,
        > for LastArgumentProvided<F, A>
    {
        type Output = Out;

        fn call_fn(
            &self,
            args: ArgumentsOfTypes<'_, <F::FixedArgumentTypes as ArgumentTypes>::LastTrimmed>,
        ) -> Self::Output {
            self.last_argument
                .provide_argument_to(move |last| self.f.call_with_last(args, last))
        }
    }

    impl<
            Out,
            F: super::super::CallableWithFixedArguments<Output = Out>,
            A: ProvideArgument<ProvideArgumentType = <F::FixedArgumentTypes as ArgumentTypes>::Last>,
        > crate::callback::CallableWithFixedArguments for LastArgumentProvided<F, A>
    {
        type FixedArgumentTypes = <F::FixedArgumentTypes as ArgumentTypes>::LastTrimmed;

        fn call_with_last<'last>(
            &self,
            args: crate::callback::argument::ArgumentsOfTypes<
                '_,
                <Self::FixedArgumentTypes as super::ArgumentTypes>::LastTrimmed,
            >,
            last: <<Self::FixedArgumentTypes as super::ArgumentTypes>::Last as crate::callback::argument::ArgumentType<'last>>::Argument,
        ) -> <Self as crate::Callable<
            crate::callback::argument::ArgumentsOfTypes<'last, Self::FixedArgumentTypes>,
        >>::Output {
            todo!()
        }

        fn call_with_first<'first>(
            &self,
            first: <<Self::FixedArgumentTypes as super::ArgumentTypes>::First as crate::callback::argument::ArgumentType<'first>>::Argument,
            args: crate::callback::argument::ArgumentsOfTypes<
                '_,
                <Self::FixedArgumentTypes as super::ArgumentTypes>::FirstTrimmed,
            >,
        ) -> <Self as crate::Callable<
            crate::callback::argument::ArgumentsOfTypes<'first, Self::FixedArgumentTypes>,
        >>::Output {
            todo!()
        }
    }
}

mod first {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FirstArgumentProvided<
        F: super::super::CallableWithFixedArguments,
        A: ProvideArgument<ProvideArgumentType = <F::FixedArgumentTypes as ArgumentTypes>::First>,
    > {
        pub(crate) f: F,
        pub(crate) first_argument: A,
    }

    impl<
            F: super::super::CallableWithFixedArguments,
            A: ProvideArgument<ProvideArgumentType = <F::FixedArgumentTypes as ArgumentTypes>::First>,
        > super::super::IsCallable for FirstArgumentProvided<F, A>
    {
    }

    impl<
            Out,
            F: super::super::CallableWithFixedArguments<Output = Out>,
            A: ProvideArgument<ProvideArgumentType = <F::FixedArgumentTypes as ArgumentTypes>::First>,
        >
        super::super::Callable<
            ArgumentsOfTypes<'_, <F::FixedArgumentTypes as ArgumentTypes>::FirstTrimmed>,
        > for FirstArgumentProvided<F, A>
    {
        type Output = Out;

        fn call_fn(
            &self,
            args: ArgumentsOfTypes<'_, <F::FixedArgumentTypes as ArgumentTypes>::FirstTrimmed>,
        ) -> Self::Output {
            self.first_argument
                .provide_argument_to(move |first| self.f.call_with_first(first, args))
        }
    }

    impl<
            Out,
            F: super::super::CallableWithFixedArguments<Output = Out>,
            A: ProvideArgument<ProvideArgumentType = <F::FixedArgumentTypes as ArgumentTypes>::First>,
        > super::super::CallableWithFixedArguments for FirstArgumentProvided<F, A>
    {
        type FixedArgumentTypes = <F::FixedArgumentTypes as ArgumentTypes>::FirstTrimmed;

        fn call_with_last<'last>(
            &self,
            args: crate::callback::argument::ArgumentsOfTypes<
                '_,
                <Self::FixedArgumentTypes as super::ArgumentTypes>::LastTrimmed,
            >,
            last: <<Self::FixedArgumentTypes as super::ArgumentTypes>::Last as crate::callback::argument::ArgumentType<'last>>::Argument,
        ) -> <Self as crate::Callable<
            crate::callback::argument::ArgumentsOfTypes<'last, Self::FixedArgumentTypes>,
        >>::Output {
            self.first_argument
                .provide_argument_to(|first| self.f.call_with_first(first, args))
        }

        fn call_with_first<'first>(
            &self,
            first: <<Self::FixedArgumentTypes as super::ArgumentTypes>::First as crate::callback::argument::ArgumentType<'first>>::Argument,
            args: crate::callback::argument::ArgumentsOfTypes<
                '_,
                <Self::FixedArgumentTypes as super::ArgumentTypes>::FirstTrimmed,
            >,
        ) -> <Self as crate::Callable<
            crate::callback::argument::ArgumentsOfTypes<'first, Self::FixedArgumentTypes>,
        >>::Output {
            todo!()
        }
    }
}

pub trait Arguments<'a>: 'static {
    type Arguments: super::sealed::Tuple;
}

pub trait ArgumentTypes: super::sealed::Tuple + for<'a> Arguments<'a> {
    type First: for<'a> ArgumentType<'a>;
    type FirstTrimmed: ArgumentTypes;

    type Last: for<'a> ArgumentType<'a>;
    type LastTrimmed: ArgumentTypes;
}

pub type ArgumentsOfTypes<'a, A> = <A as Arguments<'a>>::Arguments;

pub enum Invalid {}

impl<'a> ArgumentType<'a> for Invalid {
    type Argument = Invalid;
}

pub enum InvalidTuple {}

impl super::sealed::Tuple for InvalidTuple {}

impl<'a> Arguments<'a> for InvalidTuple {
    type Arguments = InvalidTuple;
}

impl ArgumentTypes for InvalidTuple {
    type First = Invalid;
    type FirstTrimmed = InvalidTuple;
    type Last = Invalid2;
    type LastTrimmed = InvalidTuple;
}

pub enum Invalid2 {}

impl<'a> ArgumentType<'a> for Invalid2 {
    type Argument = Invalid2;
}

impl ProvideArgument for Invalid {
    type ProvideArgumentType = Invalid;

    // fn provide_argument(&self) -> <Self::ProvideArgumentType as ArgumentType<'_>>::Argument {
    //     match *self {}
    // }

    fn provide_argument_to<
        Out,
        F: for<'arg> FnOnce(<Self::ProvideArgumentType as ArgumentType<'arg>>::Argument) -> Out,
    >(
        &self,
        _: F,
    ) -> Out {
        match *self {}
    }
}
