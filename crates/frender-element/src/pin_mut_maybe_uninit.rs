use std::mem::MaybeUninit;

use std::pin::Pin;

pub struct PinMutMaybeUninit<'a, T> {
    pub(crate) maybe_uninit: Pin<&'a mut MaybeUninit<T>>,
}

impl<'a, T> PinMutMaybeUninit<'a, T> {
    pub fn write(self, value: T) -> Pin<&'a mut T> {
        // SAFETY: maybe_uninit will not be used to mutate the value
        let maybe_uninit = unsafe { self.maybe_uninit.get_unchecked_mut() };
        let value = maybe_uninit.write(value);
        // SAFETY: constructed from pinned reference
        unsafe { Pin::new_unchecked(value) }
    }
}

pub trait PinMutInitializeWith<I>: Sized {
    fn pin_mut_initialize_with<'a>(
        this: PinMutMaybeUninit<'a, Self>,
        initialize: I,
    ) -> Pin<&'a mut Self>;
}

macro_rules! imp {
    ( $($t:tt),+ $(,)?) => {
        imp! { @append [] $($t)* }
    };
    (@impl $(($t:ident, $i:ident, $idx:tt))+) => {
        impl<
                $($t,)+
                $($i: for<'a> FnOnce(PinMutMaybeUninit<'a, $t>) -> Pin<&'a mut $t>,)+
            > PinMutInitializeWith<($($i,)+)> for ($($t,)+)
        {
            fn pin_mut_initialize_with<'a>(
                this: PinMutMaybeUninit<'a, Self>,
                initialize: ($($i,)+),
            ) -> Pin<&'a mut Self> {
                let maybe_uninit = unsafe { this.maybe_uninit.get_unchecked_mut() };
                {
                    let maybe_uninit = maybe_uninit.as_mut_ptr();

                    $(
                        let _: Pin<&mut $t> = (initialize.$idx)(PinMutMaybeUninit {
                            maybe_uninit: unsafe {
                                Pin::new_unchecked(
                                    &mut *(std::ptr::addr_of_mut!((*maybe_uninit).$idx)
                                        as *mut MaybeUninit<$t>),
                                )
                            },
                        });
                    )+
                }

                unsafe { Pin::new_unchecked(maybe_uninit.assume_init_mut()) }
            }
        }
    };
    (@append [$($ta:tt)*] ) => {};
    (@append [$($ta:tt)*] $t0:tt $($t:tt)* ) => {
        imp! { @impl    $($ta)* $t0 }
        imp! { @append [$($ta)* $t0] $($t)* }
    };
}

imp!(
    (T0, I0, 0),
    (T1, I1, 1),
    (T2, I2, 2),
    (T3, I3, 3),
    (T4, I4, 4),
    (T5, I5, 5),
    (T6, I6, 6),
    (T7, I7, 7),
    (T8, I8, 8),
    (T9, I9, 9),
    (T10, I10, 10),
    (T11, I11, 11),
    (T12, I12, 12),
);
