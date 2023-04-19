use frender_events::callback::{Callback, IsCallback};
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

pub trait ShareValueExt: ShareValue {
    fn into_callback<R>(self, f: fn(&Self) -> R) -> IntoCallback<Use<Self, R>, (), Self>
    where
        Self: Sized + Clone,
    {
        IntoCallback {
            share_value: self,
            f: Use(f),
            dependency: (),
        }
    }

    fn into_callback_map<R>(self, f: fn(&Self::Value) -> R) -> IntoCallbackMap<R, Self>
    where
        Self: Sized + Clone,
    {
        IntoCallbackMap {
            share_value: self,
            f,
            dependency: (),
        }
    }

    fn into_callback_map_mut<R>(self, f: fn(&mut Self::Value) -> R) -> IntoCallbackMapMut<R, Self>
    where
        Self: Sized + Clone,
    {
        IntoCallbackMapMut {
            share_value: self,
            f,
            dependency: (),
        }
    }

    fn into_callback_map_mut_accept<IN, R>(
        self,
        f: fn(&mut Self::Value, IN) -> R,
    ) -> IntoCallbackMapMutAcceptInput<IN, R, Self>
    where
        Self: Sized + Clone,
    {
        IntoCallbackMapMutAcceptInput {
            share_value: self,
            f,
            dependency: (),
        }
    }
}

impl<S: ?Sized> ShareValueExt for S where S: ShareValue {}

#[derive(Debug, Clone, Copy)]
pub struct IntoCallback<F: Clone + Equivalent, Dependency: Clone + PartialEq, S: Clone + ShareValue>
{
    share_value: S,
    f: F,
    dependency: Dependency,
}

impl<F: Clone + Equivalent, Dependency: Clone + PartialEq, S: Clone + ShareValue> IsCallback
    for IntoCallback<F, Dependency, S>
{
}

impl<F: Clone + Equivalent, Dependency: Clone + PartialEq, S: Clone + ShareValue> PartialEq
    for IntoCallback<F, Dependency, S>
{
    fn eq(&self, other: &Self) -> bool {
        self.f.equivalent(&other.f)
            && self.share_value.equivalent_to(&other.share_value)
            && self.dependency == other.dependency
    }
}

pub type IntoCallbackMap<Out, S> = IntoCallback<fn(&<S as ShareValue>::Value) -> Out, (), S>;
pub type IntoCallbackMapAcceptInput<IN, Out, S> =
    IntoCallback<fn(&<S as ShareValue>::Value, IN) -> Out, (), S>;
pub type IntoCallbackMapFor<Dependency, Out, S> =
    IntoCallback<fn(&<S as ShareValue>::Value, &Dependency) -> Out, Dependency, S>;

pub type IntoCallbackMapMut<Out, S> = IntoCallback<fn(&mut <S as ShareValue>::Value) -> Out, (), S>;
pub type IntoCallbackMapMutAcceptInput<IN, Out, S> =
    IntoCallback<fn(&mut <S as ShareValue>::Value, IN) -> Out, (), S>;
pub type IntoCallbackMapMutFor<Dependency, Out, S> =
    IntoCallback<Hkt21<<S as ShareValue>::Value, Dependency, Out>, Dependency, S>;

impl<R, S: Clone + ShareValue> Callback<()> for IntoCallback<Use<S, R>, (), S> {
    type Output = R;

    fn emit(&self, _: ()) -> Self::Output {
        (self.f.0)(&self.share_value)
    }
}

impl<R, S: Clone + ShareValue> Callback<()> for IntoCallbackMap<R, S> {
    type Output = R;

    fn emit(&self, _: ()) -> Self::Output {
        self.share_value.map(move |state| (self.f)(state))
    }
}

impl<R, S: Clone + ShareValue> Callback<()> for IntoCallbackMapMut<R, S> {
    type Output = R;

    fn emit(&self, _: ()) -> Self::Output {
        self.share_value.map_mut(move |state| (self.f)(state))
    }
}

impl<IN, R, S: Clone + ShareValue> Callback<IN> for IntoCallbackMapMutAcceptInput<IN, R, S> {
    type Output = R;

    fn emit(&self, input: IN) -> Self::Output {
        self.share_value
            .map_mut(move |state| (self.f)(state, input))
    }
}

impl<R, Dependency: Clone + PartialEq, S: Clone + ShareValue> Callback<()>
    for IntoCallbackMapMutFor<Dependency, R, S>
{
    type Output = R;

    fn emit(&self, _: ()) -> Self::Output {
        self.share_value
            .map_mut(|state| (self.f.0)(state, &self.dependency))
    }
}

impl_equivalent!(
    impl_!(A0, R): fn(&A0) -> R,
    impl_!(A0, R): fn(&mut A0) -> R,
    impl_!(A0, A1, R): fn(&mut A0, A1) -> R,
);
