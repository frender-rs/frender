use super::{argument::ProvideArgument, *};

pub trait IsCallable {
    fn accept_anything(self) -> accept_anything::AcceptAnything<Self>
    where
        Self: Sized + Callable<()>,
    {
        accept_anything::AcceptAnything(self)
    }

    fn provide_last_argument<
        A: ProvideArgument<
            ProvideArgumentType = <Self::FixedArgumentTypes as argument::ArgumentTypes>::Last,
        >,
    >(
        self,
        last_argument: A,
    ) -> super::argument::LastArgumentProvided<Self, A>
    where
        Self: Sized + CallableWithFixedArguments,
    {
        super::argument::LastArgumentProvided {
            f: self,
            last_argument,
        }
    }

    fn provide_first_argument<
        A: ProvideArgument<
            ProvideArgumentType = <Self::FixedArgumentTypes as argument::ArgumentTypes>::First,
        >,
    >(
        self,
        first_argument: A,
    ) -> super::argument::FirstArgumentProvided<Self, A>
    where
        Self: Sized + CallableWithFixedArguments,
    {
        super::argument::FirstArgumentProvided {
            f: self,
            first_argument,
        }
    }

    fn provide_last_argument_refed<T: 'static>(
        self,
        last_argument: T,
    ) -> super::argument::LastArgumentProvided<Self, argument::Refed<T>>
    where
        Self: Sized + CallableWithFixedArguments,
        Self::FixedArgumentTypes: argument::ArgumentTypes<Last = argument::ByRef<T>>,
    {
        super::argument::LastArgumentProvided {
            f: self,
            last_argument: argument::Refed(last_argument),
        }
    }

    fn provide_first_argument_refed<T: 'static>(
        self,
        first_argument: T,
    ) -> super::argument::FirstArgumentProvided<Self, argument::Refed<T>>
    where
        Self: Sized + CallableWithFixedArguments,
        Self::FixedArgumentTypes: argument::ArgumentTypes<First = argument::ByRef<T>>,
    {
        super::argument::FirstArgumentProvided {
            f: self,
            first_argument: argument::Refed(first_argument),
        }
    }

    fn provide_first_argument_copied<T: Copy + 'static>(
        self,
        first_argument: T,
    ) -> super::argument::FirstArgumentProvided<Self, argument::Copied<T>>
    where
        Self: Sized + CallableWithFixedArguments,
        Self::FixedArgumentTypes: argument::ArgumentTypes<First = argument::Value<T>>,
    {
        self.provide_first_argument(argument::Copied(first_argument))
    }

    fn provide_last_argument_copied<T: Copy + 'static>(
        self,
        last_argument: T,
    ) -> super::argument::LastArgumentProvided<Self, argument::Copied<T>>
    where
        Self: Sized + CallableWithFixedArguments,
        Self::FixedArgumentTypes: argument::ArgumentTypes<Last = argument::Value<T>>,
    {
        super::argument::LastArgumentProvided {
            f: self,
            last_argument: argument::Copied(last_argument),
        }
    }

    fn provide_last_argument_cloned<T: Clone + 'static>(
        self,
        last_argument: T,
    ) -> super::argument::LastArgumentProvided<Self, argument::Cloned<T>>
    where
        Self: Sized + CallableWithFixedArguments,
        Self::FixedArgumentTypes: argument::ArgumentTypes<Last = argument::Value<T>>,
    {
        super::argument::LastArgumentProvided {
            f: self,
            last_argument: argument::Cloned(last_argument),
        }
    }
}

/// Anything implementing Callback has the following traits:
///
/// - Clone-able: impl [`Clone`]
/// - Comparable: impl [`PartialEq`]
/// - Marked as callback: impl [`IsCallback`]
/// - Callable: impl [`Callback<IN>`],
///   which has associated type [`Output`](Callback::Output)
///   and method [`emit()`](Callback::emit).
///
/// ## Why not closures?
///
/// Rust has support for closures. They automatically impl [`Fn`] traits.
/// And closures also automatically impl [`Clone`]
/// [if it only captured Clone-able data](https://doc.rust-lang.org/reference/types/closure.html#other-traits).
///
/// However, closure doesn't automatically impl [`PartialEq`].
///
/// ## Notable implementors:
///
/// ```
/// # use frender_events::Callback;
/// # fn assert<IN, Out>() where
/// fn(IN) -> Out : Callback<IN, Output = Out>
/// # {} assert::<String, usize>()
/// ```
///
/// ```
/// # use frender_events::{Callback, callback::HkFn};
/// # fn assert<IN, Out>() where
/// HkFn<fn(&IN) -> Out> : for<'input> Callback<&'input IN, Output = Out>
/// # {} assert::<String, usize>()
/// ```
///
/// Note that `fn(&IN) -> Out` doesn't implement `for<'input> Callback<&'input IN, Output = Out>`
/// due to limitations of higher kind types in rust.
pub trait Callback<IN>: Callable<(IN,)> {
    // type Output;

    fn emit(&self, input: IN) -> Self::Output {
        self.call_fn((input,))
    }

    fn chain<F: Callback<Self::Output>>(self, f: F) -> chain::Chain<Self, F>
    where
        Self: Sized,
    {
        chain::Chain(self, f)
    }

    /// Provide input with another callback.
    ///
    /// ```
    /// # use frender_events::{callback, Callback, CallbackExt};
    /// let plus_1 = callback(|v: i32| v + 1);
    /// let plus_2 = plus_1.reform(plus_1);
    /// let plus_4 = plus_2.reform(callback(|v| v + 2));
    ///
    /// assert_eq!(plus_1.emit(1), 2);
    /// assert_eq!(plus_2.emit(1), 3);
    /// assert_eq!(plus_4.emit(1), 5);
    /// ```
    fn reform<NewInput, F: Callback<NewInput, Output = IN>>(self, f: F) -> chain::Chain<F, Self>
    where
        Self: Sized,
    {
        f.chain(self)
    }

    fn reform_ref<NewInput, F: for<'i> Callback<&'i NewInput, Output = IN>>(
        self,
        f: F,
    ) -> chain::Chain<F, Self>
    where
        Self: Sized,
    {
        chain::Chain(f, self)
    }

    // fn reform_mut<NewInput, F: for<'input> Callback<&'input mut NewInput, Output = IN>>(
    //     self,
    //     f: F,
    // ) -> Chain<F, Self> {
    //     f.chain(self)
    // }
}

impl<A1, F: Callable<(A1,)>> crate::Callback<A1> for F {}

pub trait Callable<Args: sealed::Tuple>: crate::callback::IsCallable {
    type Output;

    fn call_fn(&self, args: Args) -> Self::Output;
}

pub trait CallableWithFixedArguments:
    IsCallable
    + for<'a> crate::Callable<super::argument::ArgumentsOfTypes<'a, Self::FixedArgumentTypes>>
{
    type FixedArgumentTypes: argument::ArgumentTypes;
    // type LastArgumentProvided: CallableWithFixedArguments<
    //     FixedArgumentTypes = <Self::FixedArgumentTypes as argument::ArgumentTypes>::LastTrimmed,
    // >;

    fn call_with_prepended_args<'first: 'out, 'args: 'out, 'out>(
        &self,
        first: super::argument::ArgumentOfType<
            'first,
            <Self::FixedArgumentTypes as argument::ArgumentTypes>::First,
        >,
        args: super::argument::ArgumentsOfTypes<
            'args,
            <Self::FixedArgumentTypes as argument::ArgumentTypes>::FirstTrimmed,
        >,
    ) -> <Self as Callable<super::argument::ArgumentsOfTypes<'out, Self::FixedArgumentTypes>>>::Output
    {
        let first = <<Self::FixedArgumentTypes as argument::ArgumentTypes>::First as argument::ArgumentType>::re_borrow(first);
        let args = <<Self::FixedArgumentTypes as argument::ArgumentTypes>::FirstTrimmed as argument::ArgumentTypes>::re_borrow(args);
        self.call_fn(
            <Self::FixedArgumentTypes as argument::ArgumentTypes>::from_prepended(first, args),
        )
    }

    fn call_with_appended_args<'last: 'out, 'args: 'out, 'out>(
        &self,
        args: super::argument::ArgumentsOfTypes<
            'args,
            <Self::FixedArgumentTypes as argument::ArgumentTypes>::LastTrimmed,
        >,
        last: super::argument::ArgumentOfType<
            'last,
            <Self::FixedArgumentTypes as argument::ArgumentTypes>::Last,
        >,
    ) -> <Self as Callable<super::argument::ArgumentsOfTypes<'out, Self::FixedArgumentTypes>>>::Output
    {
        let args = <<Self::FixedArgumentTypes as argument::ArgumentTypes>::LastTrimmed as argument::ArgumentTypes>::re_borrow(args);
        let last = <<Self::FixedArgumentTypes as argument::ArgumentTypes>::Last as argument::ArgumentType>::re_borrow(last);
        self.call_fn(
            <Self::FixedArgumentTypes as argument::ArgumentTypes>::from_appended(args, last),
        )
    }
}

impl<Out> IsCallable for fn() -> Out {}
impl<Out> CallableWithFixedArguments for fn() -> Out {
    type FixedArgumentTypes = ();
}
