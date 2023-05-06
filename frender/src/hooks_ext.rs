use frender_events::callback::{self, CallableWithFixedArguments, Callback};
use hooks::ShareValue;

pub trait Equivalent {
    fn equivalent(&self, other: &Self) -> bool;
}

macro_rules! impl_equivalent {
    (
        $(
            impl_!($($impl_generics:tt)*) : $ty:ty,
        )*
    ) => {$(
        impl <$($impl_generics)*> Equivalent for $ty {
            fn equivalent(&self, other: &Self) -> bool {
                *self as usize == *other as usize
            }
        }
    )*};
}

macro_rules! hkt_wrapper {
    ($(
        $vis:vis struct $name:ident
        $(<$($tp:ident),* $(,)?>)?
        ($fn_ty:ty);
    )*) => {$(
        $vis struct $name
        $(<$($tp),*>)?
        ($fn_ty);

        // impl $(<$($tp),*>)? PartialEq for $name $(<$($tp),*>)? {
        //     fn eq(&self, other: &Self) -> bool {
        //         self.0 as usize == other.0 as usize
        //     }
        // }

        // impl $(<$($tp),*>)? Eq for $name $(<$($tp),*>)? {}

        impl $(<$($tp),*>)? Equivalent for $name $(<$($tp),*>)? {
            fn equivalent(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }

        impl $(<$($tp),*>)? Clone for $name $(<$($tp),*>)? {
            fn clone(&self) -> Self {
                Self(self.0)
            }
        }

        impl $(<$($tp),*>)? Copy for $name $(<$($tp),*>)? {}
    )*};
}

hkt_wrapper!(
    pub struct Use<S, R>(fn(&S) -> R);
    pub struct Hkt2<S, R>(fn(&mut S) -> R);
    pub struct Hkt21<S, IN, R>(fn(&mut S, &IN) -> R);
);

pub trait ShareValueExt: ShareValue + 'static {
    fn into_callback<F>(
        self,
        f: F,
    ) -> callback::argument::FirstArgumentProvided<F, callback::argument::Refed<Self>>
    where
        Self: Sized,
        F: CallableWithFixedArguments,
        F::FixedArgumentTypes:
            callback::argument::ArgumentTypes<First = callback::argument::ByRef<Self>>,
    {
        f.provide_first_argument_refed(self)
    }

    // fn into_callback_map<R>(self, f: fn(&Self::Value) -> R) -> IntoCallbackMap<R, Self>
    // where
    //     Self: Sized + Clone,
    // {
    //     IntoCallbackMap {
    //         share_value: self,
    //         f,
    //         dependency: (),
    //     }
    // }

    fn into_callback_map_mut<F>(
        self,
        f: F,
    ) -> callback::argument::FirstArgumentProvided<F, MapMut<Self>>
    where
        Self: Sized,
        F: CallableWithFixedArguments,
        F::FixedArgumentTypes:
            callback::argument::ArgumentTypes<First = callback::argument::ByMut<Self::Value>>,
    {
        f.provide_first_argument(MapMut(self))
    }

    // fn into_callback_map_mut_accept<IN, R>(
    //     self,
    //     f: fn(&mut Self::Value, IN) -> R,
    // ) -> IntoCallbackMapMutAcceptInput<IN, R, Self>
    // where
    //     Self: Sized + Clone,
    // {
    //     IntoCallbackMapMutAcceptInput {
    //         share_value: self,
    //         f,
    //         dependency: (),
    //     }
    // }
}

impl<S: ?Sized + 'static> ShareValueExt for S where S: ShareValue {}

impl_equivalent!(
    impl_!(A0, R): fn(&A0) -> R,
    impl_!(A0, R): fn(&mut A0) -> R,
    impl_!(A0, A1, R): fn(&mut A0, A1) -> R,
);

pub struct MapMut<S: ShareValue>(S);

impl<S: ShareValue + 'static> callback::argument::ProvideArgument for MapMut<S> {
    type ProvideArgumentType = callback::argument::ByMut<S::Value>;

    fn provide_argument_to<
        Out,
        F: for<'arg> FnOnce(
            callback::argument::ArgumentOfType<'arg, Self::ProvideArgumentType>,
        ) -> Out,
    >(
        &self,
        f: F,
    ) -> Out {
        self.0.map_mut(f)
    }
}
