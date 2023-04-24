use super::*;

pub trait IsCallback: Clone + PartialEq {}

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
pub trait Callback<IN>: IsCallback {
    type Output;

    fn emit(&self, input: IN) -> Self::Output;

    fn chain<F: Callback<Self::Output>>(self, f: F) -> chain::Chain<Self, F> {
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
    fn reform<NewInput, F: Callback<NewInput, Output = IN>>(self, f: F) -> chain::Chain<F, Self> {
        f.chain(self)
    }

    fn reform_ref<NewInput, F: for<'i> Callback<&'i NewInput, Output = IN>>(
        self,
        f: F,
    ) -> chain::Chain<F, Self> {
        chain::Chain(f, self)
    }

    // fn reform_mut<NewInput, F: for<'input> Callback<&'input mut NewInput, Output = IN>>(
    //     self,
    //     f: F,
    // ) -> Chain<F, Self> {
    //     f.chain(self)
    // }

    fn with_input(self, input: IN) -> chain::Chain<output::Output<IN>, Self>
    where
        IN: Copy + PartialEq,
    {
        chain::Chain(output::Output { value: input }, self)
    }

    fn with_input_cloned(self, input: IN) -> chain::Chain<output_cloned::OutputCloned<IN>, Self>
    where
        IN: Clone + PartialEq,
    {
        chain::Chain(output_cloned::OutputCloned { value: input }, self)
    }
}

pub trait CallbackExt: IsCallback {
    fn accept_anything(self) -> accept_anything::AcceptAnything<Self>
    where
        Self: Callback<(), Output = ()>,
    {
        accept_anything::AcceptAnything(self)
    }

    fn with_input_ref<IN: Clone + PartialEq, Out>(
        self,
        input: IN,
    ) -> with_input_ref::WithInputRef<IN, Self>
    where
        Self: for<'input> Callback<&'input IN, Output = Out>,
    {
        with_input_ref::WithInputRef {
            callback: self,
            input,
        }
    }
}

impl<C: IsCallback> CallbackExt for C {}
