use super::{argument::ProvideArgument, *};

pub trait IsCallable {
    fn accept_anything(self) -> accept_anything::AcceptAnything<Self>
    where
        Self: Sized + Callable<()>,
    {
        accept_anything::AcceptAnything(self)
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

    fn provide_last_argument<
        A: ProvideArgument<
            ProvideArgumentType = <Self::FixedArgumentTypes as argument::ArgumentTypes>::Last,
        >,
    >(
        self,
        last_argument: A,
    ) -> super::argument::LastArgumentProvided<Self, A>
    where
        Self: Sized,
    {
        super::argument::LastArgumentProvided {
            f: self,
            last_argument,
        }
    }

    fn provide_last_argument_refed<T>(
        self,
        last_argument: T,
    ) -> super::argument::LastArgumentProvided<Self, argument::Refed<T>>
    where
        Self: Sized,
        Self::FixedArgumentTypes: argument::ArgumentTypes<Last = argument::ByRef<T>>,
    {
        super::argument::LastArgumentProvided {
            f: self,
            last_argument: argument::Refed(last_argument),
        }
    }

    fn provide_last_argument_copied<T: Copy>(
        self,
        last_argument: T,
    ) -> super::argument::LastArgumentProvided<Self, argument::Copied<T>>
    where
        Self: Sized,
        Self::FixedArgumentTypes: argument::ArgumentTypes<Last = argument::Value<T>>,
    {
        super::argument::LastArgumentProvided {
            f: self,
            last_argument: argument::Copied(last_argument),
        }
    }
}

impl<Out> IsCallable for fn() -> Out {}
impl<Out> CallableWithFixedArguments for fn() -> Out {
    type FixedArgumentTypes = ();
    // type LastArgumentProvided = super::argument::LastArgumentProvided<Self, argument::Invalid>;
}
