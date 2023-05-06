use std::marker::PhantomData;

pub use first::FirstArgumentProvided;
pub use last::LastArgumentProvided;
pub use ByMut as r#mut;
pub use ByRef as r#ref;
pub use Value as value;

pub trait Argument<'a>: 'static {
    type Argument;
}

pub trait ArgumentType: for<'a> Argument<'a> {
    fn re_borrow<'a: 'b, 'b>(arg: ArgumentOfType<'a, Self>) -> ArgumentOfType<'b, Self>;
}

pub trait ProvideArgument {
    type ProvideArgumentType: ArgumentType;

    // fn provide_argument(&self) -> <Self::ProvideArgumentType as ArgumentType<'_>>::Argument;

    fn provide_argument_to<
        Out,
        F: for<'arg> FnOnce(ArgumentOfType<'arg, Self::ProvideArgumentType>) -> Out,
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

impl<'a, T: 'static> Argument<'a> for Value<T> {
    type Argument = T;
}

impl<T: 'static> ArgumentType for Value<T> {
    fn re_borrow<'a: 'b, 'b>(arg: ArgumentOfType<'a, Self>) -> ArgumentOfType<'b, Self> {
        arg
    }
}

impl<'a, T: ?Sized + 'static> Argument<'a> for ByRef<T> {
    type Argument = &'a T;
}

impl<T: ?Sized + 'static> ArgumentType for ByRef<T> {
    fn re_borrow<'a: 'b, 'b>(arg: ArgumentOfType<'a, Self>) -> ArgumentOfType<'b, Self> {
        arg
    }
}

impl<'a, T: ?Sized + 'static> Argument<'a> for ByMut<T> {
    type Argument = &'a mut T;
}

impl<T: ?Sized + 'static> ArgumentType for ByMut<T> {
    fn re_borrow<'a: 'b, 'b>(arg: ArgumentOfType<'a, Self>) -> ArgumentOfType<'b, Self> {
        arg
    }
}

impl<T: Copy + 'static> ProvideArgument for Copied<T> {
    type ProvideArgumentType = Value<T>;

    // fn provide_argument(&self) -> <Value<T> as ArgumentType<'_>>::Argument {
    //     self.0
    // }

    fn provide_argument_to<
        Out,
        F: for<'arg> FnOnce(ArgumentOfType<'arg, Self::ProvideArgumentType>) -> Out,
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
        F: for<'arg> FnOnce(ArgumentOfType<'arg, Self::ProvideArgumentType>) -> Out,
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
        F: for<'arg> FnOnce(ArgumentOfType<'arg, Self::ProvideArgumentType>) -> Out,
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
                .provide_argument_to(move |last| self.f.call_with_appended_args(args, last))
        }
    }

    impl<
            Out,
            F: super::super::CallableWithFixedArguments<Output = Out>,
            A: ProvideArgument<ProvideArgumentType = <F::FixedArgumentTypes as ArgumentTypes>::Last>,
        > crate::callback::CallableWithFixedArguments for LastArgumentProvided<F, A>
    {
        type FixedArgumentTypes = <F::FixedArgumentTypes as ArgumentTypes>::LastTrimmed;
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
                .provide_argument_to(move |first| self.f.call_with_prepended_args(first, args))
        }
    }

    impl<
            Out,
            F: super::super::CallableWithFixedArguments<Output = Out>,
            A: ProvideArgument<ProvideArgumentType = <F::FixedArgumentTypes as ArgumentTypes>::First>,
        > super::super::CallableWithFixedArguments for FirstArgumentProvided<F, A>
    {
        type FixedArgumentTypes = <F::FixedArgumentTypes as ArgumentTypes>::FirstTrimmed;
    }
}

pub trait Arguments<'a>: 'static {
    type Arguments: super::sealed::Tuple;
}

pub trait PrependArgument<Arg: ArgumentType>: ArgumentTypes {
    type Prepended: ArgumentTypes<First = Arg, FirstTrimmed = Self>;

    fn prepend_argument<'a>(
        first: ArgumentOfType<'a, Arg>,
        args: ArgumentsOfTypes<'a, Self>,
    ) -> ArgumentsOfTypes<'a, Self::Prepended>;
}

pub trait AppendArgument<Arg: ArgumentType>: ArgumentTypes {
    type Appended: ArgumentTypes<Last = Arg, LastTrimmed = Self>;

    fn append_argument<'a>(
        args: ArgumentsOfTypes<'a, Self>,
        last: ArgumentOfType<'a, Arg>,
    ) -> ArgumentsOfTypes<'a, Self::Appended>;
}

pub trait ArgumentTypes: super::sealed::Tuple + for<'a> Arguments<'a> {
    type First: ArgumentType;
    type FirstTrimmed: ArgumentTypes + PrependArgument<Self::First, Prepended = Self>;

    type Last: ArgumentType;
    type LastTrimmed: ArgumentTypes + AppendArgument<Self::Last, Appended = Self>;

    fn re_borrow<'a: 'b, 'b>(args: ArgumentsOfTypes<'a, Self>) -> ArgumentsOfTypes<'b, Self>;

    fn from_prepended<'a>(
        first: ArgumentOfType<'a, Self::First>,
        args: ArgumentsOfTypes<'a, Self::FirstTrimmed>,
    ) -> ArgumentsOfTypes<'a, Self> {
        <Self::FirstTrimmed as PrependArgument<Self::First>>::prepend_argument(first, args)
    }

    fn from_appended<'a>(
        args: ArgumentsOfTypes<'a, Self::LastTrimmed>,
        last: ArgumentOfType<'a, Self::Last>,
    ) -> ArgumentsOfTypes<'a, Self> {
        <Self::LastTrimmed as AppendArgument<Self::Last>>::append_argument(args, last)
    }
}

pub type ArgumentOfType<'a, A> = <A as Argument<'a>>::Argument;
pub type ArgumentsOfTypes<'a, A> = <A as Arguments<'a>>::Arguments;

pub enum Invalid {}

impl<'a> Argument<'a> for Invalid {
    type Argument = Invalid;
}

impl ArgumentType for Invalid {
    fn re_borrow<'a: 'b, 'b>(arg: ArgumentOfType<'a, Self>) -> ArgumentOfType<'b, Self> {
        match arg {}
    }
}

pub enum InvalidTuple {}

impl super::sealed::Tuple for InvalidTuple {}

impl<'a> Arguments<'a> for InvalidTuple {
    type Arguments = InvalidTuple;
}

impl ArgumentTypes for InvalidTuple {
    type First = Invalid2;
    type FirstTrimmed = InvalidTuple;
    type Last = Invalid2;
    type LastTrimmed = InvalidTuple;

    fn re_borrow<'a: 'b, 'b>(args: ArgumentsOfTypes<'a, Self>) -> ArgumentsOfTypes<'b, Self> {
        match args {}
    }
}

impl PrependArgument<Invalid> for InvalidTuple {
    type Prepended = ();

    fn prepend_argument<'a>(
        first: ArgumentOfType<'a, Invalid>,
        _: ArgumentsOfTypes<'a, Self>,
    ) -> ArgumentsOfTypes<'a, Self::Prepended> {
        match first {}
    }
}

impl PrependArgument<Invalid2> for InvalidTuple {
    type Prepended = InvalidTuple;

    fn prepend_argument<'a>(
        first: ArgumentOfType<'a, Invalid2>,
        _: ArgumentsOfTypes<'a, Self>,
    ) -> ArgumentsOfTypes<'a, Self::Prepended> {
        match first {}
    }
}

impl AppendArgument<Invalid> for InvalidTuple {
    type Appended = ();

    fn append_argument<'a>(
        _: ArgumentsOfTypes<'a, Self>,
        last: ArgumentOfType<'a, Invalid>,
    ) -> ArgumentsOfTypes<'a, Self::Appended> {
        match last {}
    }
}

impl AppendArgument<Invalid2> for InvalidTuple {
    type Appended = InvalidTuple;

    fn append_argument<'a>(
        _: ArgumentsOfTypes<'a, Self>,
        last: ArgumentOfType<'a, Invalid2>,
    ) -> ArgumentsOfTypes<'a, Self::Appended> {
        match last {}
    }
}

pub enum Invalid2 {}

impl<'a> Argument<'a> for Invalid2 {
    type Argument = Invalid2;
}

impl ArgumentType for Invalid2 {
    fn re_borrow<'a: 'b, 'b>(arg: ArgumentOfType<'a, Self>) -> ArgumentOfType<'b, Self> {
        match arg {}
    }
}

impl ProvideArgument for Invalid {
    type ProvideArgumentType = Invalid;

    // fn provide_argument(&self) -> <Self::ProvideArgumentType as ArgumentType<'_>>::Argument {
    //     match *self {}
    // }

    fn provide_argument_to<
        Out,
        F: for<'arg> FnOnce(ArgumentOfType<'arg, Self::ProvideArgumentType>) -> Out,
    >(
        &self,
        _: F,
    ) -> Out {
        match *self {}
    }
}
