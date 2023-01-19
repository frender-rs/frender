use crate::Inheritable;

pub trait MaybeInherit<V> {
    fn into_maybe_inheritable(self) -> Option<Inheritable<V>>;
}

impl<V> MaybeInherit<V> for V {
    #[inline]
    fn into_maybe_inheritable(self) -> Option<Inheritable<V>> {
        Some(Inheritable::Value(self))
    }
}

impl<V> MaybeInherit<V> for bg::Unspecified<V> {
    #[inline]
    fn into_maybe_inheritable(self) -> Option<Inheritable<V>> {
        None
    }
}

impl<V> MaybeInherit<V> for Inheritable<V> {
    #[inline]
    fn into_maybe_inheritable(self) -> Option<Inheritable<V>> {
        Some(self)
    }
}

impl<V> MaybeInherit<V> for crate::Inherit<V> {
    fn into_maybe_inheritable(self) -> Option<Inheritable<V>> {
        Some(Inheritable::Inherit)
    }
}

macro_rules! impl_maybe_inherit_for {
    ($($ty:ty;)*) => {$(
        impl MaybeInherit<$ty> for () {
            #[inline]
            fn into_maybe_inheritable(self) -> Option<Inheritable<$ty>> {
                None
            }
        }
    )*};
}

impl_maybe_inherit_for! {
    bool;
}
