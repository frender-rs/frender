use std::rc::Rc;

pub trait DynFn: FnOfArgs<Self::ArgsTuple, DynFn = Self> {
    type ArgsTuple;
}

pub trait DynFnMut: FnMutOfArgs<Self::ArgsTuple, DynFnMut = Self> {
    type ArgsTuple;
}

pub trait DynFnOnce: FnOnceOfArgs<Self::ArgsTuple, DynFnOnce = Self> {
    type ArgsTuple;
}

pub trait FnOfArgs<TArgs>: FnMutOfArgs<TArgs> {
    type DynFn: ?Sized + DynFn<ArgsTuple = TArgs>;

    fn into_rc_dyn_fn(self: Rc<Self>) -> Rc<Self::DynFn>
    where
        Self: 'static;
    fn rc_into_box_dyn_fn(self: Rc<Self>) -> Box<Self::DynFn>
    where
        Self: 'static;
    fn into_box_dyn_fn(self: Box<Self>) -> Box<Self::DynFn>
    where
        Self: 'static;
}

pub trait FnMutOfArgs<TArgs>: FnOnceOfArgs<TArgs> {
    type DynFnMut: ?Sized + DynFnMut<ArgsTuple = TArgs>;

    fn into_box_dyn_fn_mut(self: Box<Self>) -> Box<Self::DynFnMut>
    where
        Self: 'static;
}

pub trait FnOnceOfArgs<TArgs> {
    type Output;
    type DynFnOnce: ?Sized + DynFnOnce<ArgsTuple = TArgs>;

    fn into_box_dyn_fn_once(self: Box<Self>) -> Box<Self::DynFnOnce>
    where
        Self: 'static;
}

macro_rules! doit {
    ($(
        ($($var:ident)*)
    )*) => ($(
        impl<$($var,)* TReturn> DynFn for dyn Fn($($var,)*) -> TReturn {
            type ArgsTuple = ($($var,)*);
        }

        impl<$($var,)* TReturn> DynFnMut for dyn FnMut($($var,)*) -> TReturn {
            type ArgsTuple = ($($var,)*);
        }

        impl<$($var,)* TReturn> DynFnOnce for dyn FnOnce($($var,)*) -> TReturn {
            type ArgsTuple = ($($var,)*);
        }

        impl<TFunc, $($var,)* TReturn> FnOnceOfArgs<($($var,)*)> for TFunc
            where TFunc: FnOnce($($var),*) -> TReturn
        {
            type Output = TReturn;
            type DynFnOnce = dyn FnOnce($($var,)*) -> TReturn;

            #[inline]
            fn into_box_dyn_fn_once(self: Box<Self>) -> Box<Self::DynFnOnce>
            where
                Self: 'static,
            {
                self as Box<Self::DynFnOnce>
            }
        }

        impl<$($var,)* TReturn> FnOnceOfArgs<($($var,)*)> for dyn FnOnce($($var),*) -> TReturn {
            type Output = TReturn;
            type DynFnOnce = Self;

            #[inline]
            fn into_box_dyn_fn_once(self: Box<Self>) -> Box<Self::DynFnOnce>
            where
                Self: 'static,
            {
                self
            }
        }

        impl<$($var,)* TReturn> FnOnceOfArgs<($($var,)*)> for dyn FnMut($($var),*) -> TReturn {
            type Output = TReturn;
            type DynFnOnce = dyn FnOnce($($var,)*) -> TReturn;

            #[inline]
            fn into_box_dyn_fn_once(self: Box<Self>) -> Box<Self::DynFnOnce>
            where
                Self: 'static,
            {
                Box::new(self) as Box<Self::DynFnOnce>
            }
        }

        impl<$($var,)* TReturn> FnOnceOfArgs<($($var,)*)> for dyn Fn($($var),*) -> TReturn {
            type Output = TReturn;
            type DynFnOnce = dyn FnOnce($($var,)*) -> TReturn;

            #[inline]
            fn into_box_dyn_fn_once(self: Box<Self>) -> Box<Self::DynFnOnce>
            where
                Self: 'static,
            {
                Box::new(self) as Box<Self::DynFnOnce>
            }
        }

        impl<TFunc, $($var,)* TReturn> FnMutOfArgs<($($var,)*)> for TFunc
            where TFunc: FnMut($($var),*) -> TReturn
        {
            type DynFnMut = dyn FnMut($($var,)*) -> TReturn;

            #[inline]
            fn into_box_dyn_fn_mut(self: Box<Self>) -> Box<Self::DynFnMut>
            where
                Self: 'static,
            {
                self as Box<Self::DynFnMut>
            }
        }

        impl<$($var,)* TReturn> FnMutOfArgs<($($var,)*)> for dyn FnMut($($var),*) -> TReturn {
            type DynFnMut = Self;

            #[inline]
            fn into_box_dyn_fn_mut(self: Box<Self>) -> Box<Self::DynFnMut>
            where
                Self: 'static,
            {
                self
            }
        }

        impl<$($var,)* TReturn> FnMutOfArgs<($($var,)*)> for dyn Fn($($var),*) -> TReturn {
            type DynFnMut = dyn FnMut($($var,)*) -> TReturn;

            #[inline]
            fn into_box_dyn_fn_mut(self: Box<Self>) -> Box<Self::DynFnMut>
            where
                Self: 'static,
            {
                Box::new(self) as Box<Self::DynFnMut>
            }
        }

        impl<TFunc, $($var,)* TReturn> FnOfArgs<($($var,)*)> for TFunc
            where TFunc: Fn($($var),*) -> TReturn
        {
            type DynFn = dyn Fn($($var,)*) -> TReturn;

            #[inline]
            fn into_rc_dyn_fn(self: Rc<Self>) -> Rc<Self::DynFn>
            where
                Self: 'static,
            {
                self as Rc<Self::DynFn>
            }

            #[inline]
            fn rc_into_box_dyn_fn(self: Rc<Self>) -> Box<Self::DynFn>
            where
                Self: 'static,
            {
                #![allow(non_snake_case)]
                Box::new(move |$($var: $var),*| (*self)($($var),*)) as Box<Self::DynFn>
            }

            #[inline]
            fn into_box_dyn_fn(self: Box<Self>) -> Box<Self::DynFn>
            where
                Self: 'static,
            {
                self as Box<Self::DynFn>
            }
        }

        impl<$($var,)* TReturn> FnOfArgs<($($var,)*)> for dyn Fn($($var),*) -> TReturn {
            type DynFn = Self;

            #[inline]
            fn into_rc_dyn_fn(self: Rc<Self>) -> Rc<Self::DynFn>
            where
                Self: 'static,
            {
                self as Rc<Self::DynFn>
            }

            #[inline]
            fn rc_into_box_dyn_fn(self: Rc<Self>) -> Box<Self::DynFn>
            where
                Self: 'static,
            {
                #![allow(non_snake_case)]
                Box::new(move |$($var: $var),*| (*self)($($var),*)) as Box<Self::DynFn>
            }

            #[inline]
            fn into_box_dyn_fn(self: Box<Self>) -> Box<Self::DynFn>
            where
                Self: 'static,
            {
                self as Box<Self::DynFn>
            }
        }

        impl<TFunc: 'static + Fn($($var,)*) -> TReturn, TReturn, $($var,)*> $crate::IntoPropValue<$crate::AnyFn<dyn Fn($($var,)*) -> TReturn>> for TFunc {
            #[inline]
            fn into_prop_value(self) -> $crate::AnyFn<dyn Fn($($var,)*) -> TReturn> {
                $crate::AnyFn::new(self)
            }
        }

        impl<TFunc: 'static + Fn($($var,)*) -> TReturn, TReturn, $($var,)*> $crate::IntoPropValue<Option<$crate::AnyFn<dyn Fn($($var,)*) -> TReturn>>> for TFunc {
            #[inline]
            fn into_prop_value(self) -> Option<$crate::AnyFn<dyn Fn($($var,)*) -> TReturn>> {
                Some($crate::AnyFn::new(self))
            }
        }

        impl<TFunc: 'static + Fn($($var,)*) -> TReturn, TReturn, $($var,)*> $crate::IntoPropValue<Option<$crate::AnyFn<dyn Fn($($var,)*) -> TReturn>>> for Option<TFunc> {
            #[inline]
            fn into_prop_value(self) -> Option<$crate::AnyFn<dyn Fn($($var,)*) -> TReturn>> {
                self.map($crate::IntoPropValue::into_prop_value)
            }
        }

        impl<TFunc: 'static + FnMut($($var,)*) -> TReturn, TReturn, $($var,)*> $crate::IntoPropValue<$crate::AnyFnMut<dyn FnMut($($var,)*) -> TReturn>> for TFunc {
            #[inline]
            fn into_prop_value(self) -> $crate::AnyFnMut<dyn FnMut($($var,)*) -> TReturn> {
                $crate::AnyFnMut::new(self)
            }
        }

        impl<TFunc: 'static + FnMut($($var,)*) -> TReturn, TReturn, $($var,)*> $crate::IntoPropValue<Option<$crate::AnyFnMut<dyn FnMut($($var,)*) -> TReturn>>> for TFunc {
            #[inline]
            fn into_prop_value(self) -> Option<$crate::AnyFnMut<dyn FnMut($($var,)*) -> TReturn>> {
                Some($crate::AnyFnMut::new(self))
            }
        }

        impl<TFunc: 'static + FnMut($($var,)*) -> TReturn, TReturn, $($var,)*> $crate::IntoPropValue<Option<$crate::AnyFnMut<dyn FnMut($($var,)*) -> TReturn>>> for Option<TFunc> {
            #[inline]
            fn into_prop_value(self) -> Option<$crate::AnyFnMut<dyn FnMut($($var,)*) -> TReturn>> {
                self.map($crate::IntoPropValue::into_prop_value)
            }
        }

        impl<TFunc: 'static + FnOnce($($var,)*) -> TReturn, TReturn, $($var,)*> $crate::IntoPropValue<$crate::AnyFnOnce<dyn FnOnce($($var,)*) -> TReturn>> for TFunc {
            #[inline]
            fn into_prop_value(self) -> $crate::AnyFnOnce<dyn FnOnce($($var,)*) -> TReturn> {
                $crate::AnyFnOnce::new(self)
            }
        }

        impl<TFunc: 'static + FnOnce($($var,)*) -> TReturn, TReturn, $($var,)*> $crate::IntoPropValue<Option<$crate::AnyFnOnce<dyn FnOnce($($var,)*) -> TReturn>>> for TFunc {
            #[inline]
            fn into_prop_value(self) -> Option<$crate::AnyFnOnce<dyn FnOnce($($var,)*) -> TReturn>> {
                Some($crate::AnyFnOnce::new(self))
            }
        }

        impl<TFunc: 'static + FnOnce($($var,)*) -> TReturn, TReturn, $($var,)*> $crate::IntoPropValue<Option<$crate::AnyFnOnce<dyn FnOnce($($var,)*) -> TReturn>>> for Option<TFunc> {
            #[inline]
            fn into_prop_value(self) -> Option<$crate::AnyFnOnce<dyn FnOnce($($var,)*) -> TReturn>> {
                self.map($crate::IntoPropValue::into_prop_value)
            }
        }
    )*)
}

doit! {
    ()
    (A)
    (A B)
    (A B C)
    (A B C D)
    (A B C D E)
    (A B C D E F)
    (A B C D E F G)
    (A B C D E F G H)
}
