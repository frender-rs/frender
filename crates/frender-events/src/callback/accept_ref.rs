use super::{Callback, IsCallback};

/// This type wrapper exists because the following traits are not implemented for higher kinded fn pointers,
/// but `Hkt<_>` implements them.
///
/// - [`PartialEq`] and [`Eq`]
///
/// Fails:
///
/// ```compile_fail
/// # fn assert<IN: ?Sized, Out>() where
/// fn(&IN) -> Out : PartialEq
/// # {} assert::<str, usize>();
/// ```
///
/// ```compile_fail
/// # fn assert<IN: ?Sized, Out>() where
/// fn(&IN) -> Out : Eq
/// # {} assert::<str, usize>();
/// ```
///
/// Compiles:
///
/// ```
/// # fn assert<IN: ?Sized, Out>() where frender_events::callback::
/// HkFn<fn(&IN) -> Out> : PartialEq
/// # {} assert::<str, usize>();
/// ```
///
/// ```
/// # fn assert<IN: ?Sized, Out>() where frender_events::callback::
/// HkFn<fn(&IN) -> Out> : Eq
/// # {} assert::<str, usize>();
/// ```
///
/// - [`for<'input> Callback<&'input IN>`](Callback)
///
/// Fails:
///
/// ```compile_fail
/// # use frender_events::Callback;
/// # fn assert<IN: ?Sized, Out>() where
/// fn(&IN) -> Out : for<'input> Callback<&'input IN, Output = Out>
/// # {} assert::<str, usize>();
/// ```
///
/// Compiles:
///
/// ```
/// # use frender_events::Callback;
/// # fn assert<IN: ?Sized, Out>() where frender_events::callback::
/// HkFn<fn(&IN) -> Out> : for<'input> Callback<&'input IN, Output = Out>
/// # {} assert::<str, usize>();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AcceptRef<F>(pub(super) F);

/// Wrapper for fn pointers where the 2<sup>nd</sup> argument is a higher kinded reference.
///
/// See [`HkFn`] for more.
#[derive(Debug, Clone, Copy)]
pub struct AcceptRef2<F>(pub(super) F);

macro_rules! impl_fn_wrapper {
    ($(
        impl_all! $impl_generics:tt =
        | $self_:ident : $ty:ty, $input:ident : $input_ty:ty |
        $(-> $callback_output_ty:ty $impl_callback:block)?
        $(_)?
    ),* $(,)?) => {$(
        impl_fn_wrapper! {impl_eq! $impl_generics : $ty}
        $(
            impl_fn_wrapper! {
                impl_callback! $impl_generics =
                |$self_ : $ty, $input : $input_ty| -> $callback_output_ty
                $impl_callback
            }
        )?
    )*};
    ($(impl_eq![$($impl_generics:tt)*] : $ty:ty),* $(,)?) => {$(
        impl<$($impl_generics)*> PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }

        impl<$($impl_generics)*> Eq for $ty {}
    )*};
    ($(
        impl_callback![$($impl_generics:tt)*] =
        |$self_:ident : $ty:ty, $input:ident : $input_ty:ty|
        -> $callback_output_ty:ty
        $impl_callback:block
    ),* $(,)?) => {$(
        impl<$($impl_generics)*> IsCallback for $ty {}
        impl<$($impl_generics)*> Callback<$input_ty> for $ty {
            type Output = $callback_output_ty;

            fn emit(&$self_, $input : $input_ty) -> Self::Output
                $impl_callback
        }
    )*};
}

impl_fn_wrapper!(
    impl_all![IN: ?Sized, Out] =
        |self: AcceptRef<fn(&IN) -> Out>, input: &IN| -> Out { self.0(input) }
);

impl_fn_wrapper!(
    //
    impl_eq![IN: ?Sized, State, Out]: AcceptRef<fn(&IN, &State) -> Out>,
    impl_eq![IN, State, Out]: AcceptRef2<fn(IN, &State) -> Out>,
);

pub use super::accept_ref_with_state as with_state;
