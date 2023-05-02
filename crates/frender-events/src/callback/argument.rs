use std::marker::PhantomData;

pub use ByMut as r#mut;
pub use ByRef as r#ref;
pub use Value as value;

pub trait ArgumentType<'a, ImplicitBounds = &'a Self> {
    type Argument;
}
pub trait ProvideArgument {
    type ProvideArgumentType: ?Sized + for<'a> ArgumentType<'a>;
    // fn provide_argument(&self) -> <Self::ProvideArgumentType as ArgumentType<'_>>::Argument;
    // fn provide_last_argument_to_callable<'arg, C: super::CallableWithFixedArguments>(
    //     &self,
    //     callable: &C,
    //     arguments: ArgumentsOfTypes<'arg, <C::FixedArgumentTypes as ArgumentTypes>::LastTrimmed>,
    // ) -> <C as super::Callable<ArgumentsOfTypes<'arg, C::FixedArgumentTypes>>>::Output
    // where
    //     C::FixedArgumentTypes: ArgumentTypes<Last = Self::ProvideArgumentType>;

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

impl<'a, T> ArgumentType<'a> for Value<T> {
    type Argument = T;
}

impl<'a, T: ?Sized> ArgumentType<'a> for ByRef<T> {
    type Argument = &'a T;
}

impl<'a, T: ?Sized> ArgumentType<'a> for ByMut<T> {
    type Argument = &'a mut T;
}

impl<T: Copy> ProvideArgument for Copied<T> {
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
impl<T: Clone> ProvideArgument for Cloned<T> {
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

impl<T> ProvideArgument for Refed<T> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LastArgumentProvided<
    F: super::CallableWithFixedArguments,
    A: ProvideArgument<ProvideArgumentType = <F::FixedArgumentTypes as ArgumentTypes>::Last>,
> {
    pub(super) f: F,
    pub(super) last_argument: A,
}

impl<
        F: super::CallableWithFixedArguments,
        A: ProvideArgument<ProvideArgumentType = <F::FixedArgumentTypes as ArgumentTypes>::Last>,
    > crate::callback::IsCallable for LastArgumentProvided<F, A>
{
}

pub trait Arguments<'a, ImplicitBounds = &'a Self> {
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
        f: F,
    ) -> Out {
        match *self {}
    }
}
