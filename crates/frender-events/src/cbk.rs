use crate::IsCallback;

mod sealed {
    pub trait Tuple {}
}

mod argument {
    pub trait ArgumentType<'a, ImplicitBounds = &'a Self> {
        type Argument;
    }
    pub trait ProvideArgument<A: ?Sized + for<'a> ArgumentType<'a>> {
        fn provide_argument(&self) -> <A as ArgumentType<'_>>::Argument;
    }

    pub struct Value<T>(T);
    pub struct ByRef<T: ?Sized>(T);
    pub struct ByMut<T: ?Sized>(T);

    pub use ByMut as r#mut;
    pub use ByRef as r#ref;
    pub use Value as value;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Copied<T: Copy>(pub(super) T);
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Cloned<T: Clone>(pub(super) T);
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Borrowed<B>(pub(super) B);
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

    impl<T: Copy> ProvideArgument<Value<T>> for Copied<T> {
        fn provide_argument(&self) -> <Value<T> as ArgumentType<'_>>::Argument {
            self.0
        }
    }
    impl<T: Clone> ProvideArgument<Value<T>> for Cloned<T> {
        fn provide_argument(&self) -> <Value<T> as ArgumentType<'_>>::Argument {
            self.0.clone()
        }
    }
    impl<B: std::borrow::Borrow<T>, T: ?Sized> ProvideArgument<ByRef<T>> for Borrowed<B> {
        fn provide_argument(&self) -> &T {
            self.0.borrow()
        }
    }
    impl<T> ProvideArgument<ByRef<T>> for Refed<T> {
        fn provide_argument(&self) -> &T {
            &self.0
        }
    }
}

use sealed::Tuple;

pub trait Fn<Args: Tuple> {
    type Output;

    fn call(&self, args: Args) -> Self::Output;
}

pub fn r#fn<Out>(f: fn() -> Out) -> fn() -> Out {
    f
}

// #[cfg(feature = "impl_with_macro_rules")]
mod imp_macros;

// #[cfg(feature = "impl_with_macro_rules")]
mod imp {
    super::imp_macros::impl_with_macro_rules!(a1: A1, a2: A2, a3: A3, a4: A4, a5: A5);
}

// #[cfg(not(feature = "impl_with_macro_rules"))]
// mod imp;

pub use imp::*;

#[test]
fn testa() {
    let _: fn() = r#fn(|| {});

    let _: fn(()) = r#fn::value(|()| {});
    let _: r#fn::HkFn<fn(&())> = r#fn::r#ref(|&()| {});
    let _: r#fn::HkFn<fn(&mut ())> = r#fn::r#mut(|&mut ()| {});

    let _: fn((), ()) = r#fn::value::value(|(), ()| {});
    let _: r#fn::value::HkFn<fn((), &())> = r#fn::value::r#ref(|(), &()| {});
}

impl<Out> IsCallback for fn() -> Out {}

impl<A1, F: Fn<(A1,)> + IsCallback> crate::Callback<A1> for F {
    type Output = <F as Fn<(A1,)>>::Output;

    fn emit(&self, input: A1) -> Self::Output {
        self.call((input,))
    }
}

pub trait HasLastArgument {
    type LastArgumentType: ?Sized + for<'a> argument::ArgumentType<'a>;

    fn provide_last_argument_with<A>(self, last_argument: A) -> LastArgumentProvided<Self, A>
    where
        Self: Sized,
        A: argument::ProvideArgument<Self::LastArgumentType>,
    {
        LastArgumentProvided {
            f: self,
            last_argument,
        }
    }

    fn provide_last_argument_copied<A>(
        self,
        last_argument: A,
    ) -> LastArgumentProvided<Self, argument::Copied<A>>
    where
        Self: Sized + HasLastArgument<LastArgumentType = argument::Value<A>>,
        A: Copy,
    {
        LastArgumentProvided {
            f: self,
            last_argument: argument::Copied(last_argument),
        }
    }

    fn provide_last_argument_cloned<A>(
        self,
        last_argument: A,
    ) -> LastArgumentProvided<Self, argument::Cloned<A>>
    where
        Self: Sized + HasLastArgument<LastArgumentType = argument::Value<A>>,
        A: Clone,
    {
        LastArgumentProvided {
            f: self,
            last_argument: argument::Cloned(last_argument),
        }
    }

    fn provide_last_argument_borrowed<A>(
        self,
        last_argument: A::Owned,
    ) -> LastArgumentProvided<Self, argument::Borrowed<A::Owned>>
    where
        Self: Sized + HasLastArgument<LastArgumentType = argument::ByRef<A>>,
        A: ?Sized + ToOwned,
    {
        LastArgumentProvided {
            f: self,
            last_argument: argument::Borrowed(last_argument),
        }
    }

    fn provide_last_argument_refed<A>(
        self,
        last_argument: A,
    ) -> LastArgumentProvided<Self, argument::Refed<A>>
    where
        Self: Sized + HasLastArgument<LastArgumentType = argument::ByRef<A>>,
    {
        LastArgumentProvided {
            f: self,
            last_argument: argument::Refed(last_argument),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LastArgumentProvided<
    F: HasLastArgument,
    A: argument::ProvideArgument<F::LastArgumentType>,
> {
    f: F,
    last_argument: A,
}

impl<
        F: HasLastArgument + Clone + PartialEq,
        A: argument::ProvideArgument<F::LastArgumentType> + Clone + PartialEq,
    > crate::callback::IsCallback for LastArgumentProvided<F, A>
{
}

pub fn provide_last_argument_refed<
    F: HasLastArgument<LastArgumentType = argument::ByRef<Last>>,
    Last,
>(
    f: F,
    last_argument: Last,
) -> LastArgumentProvided<F, argument::Refed<Last>> {
    f.provide_last_argument_refed(last_argument)
}
