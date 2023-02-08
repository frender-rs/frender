use std::borrow::Cow;

use frender_core::StaticText;

pub trait MaybeUpdateValueByRef<V: ?Sized> {
    fn maybe_update_value_by_ref(this: &Self, update: impl FnOnce(&V), remove: impl FnOnce());
}

impl<V: ?Sized> MaybeUpdateValueByRef<V> for () {
    #[inline(always)]
    fn maybe_update_value_by_ref(_: &Self, _: impl FnOnce(&V), _: impl FnOnce()) {}
}

impl<V: ?Sized, T: ?Sized + MaybeUpdateValueByRef<V>> MaybeUpdateValueByRef<V> for &T {
    #[inline(always)]
    fn maybe_update_value_by_ref(this: &Self, update: impl FnOnce(&V), remove: impl FnOnce()) {
        T::maybe_update_value_by_ref(this, update, remove)
    }
}

impl<V: ?Sized, T: MaybeUpdateValueByRef<V>> MaybeUpdateValueByRef<V> for Option<T> {
    #[inline(always)]
    fn maybe_update_value_by_ref(this: &Self, update: impl FnOnce(&V), remove: impl FnOnce()) {
        match this {
            Some(this) => T::maybe_update_value_by_ref(this, update, remove),
            None => remove(),
        }
    }
}

macro_rules! auto_impl_update_value_by_ref {
    ($(
        $v_ty:ty : $($for_ty:ty),*;
    )*) => {$($(
        impl MaybeUpdateValueByRef<$v_ty> for $for_ty {
            #[inline(always)]
            fn maybe_update_value_by_ref(
                this: &Self,
                update: impl FnOnce(&str),
                _: impl FnOnce(),
            ) {
                update(this)
            }
        }
    )*)*};
}

auto_impl_update_value_by_ref! {
    str: str, String, Cow<'_, str>, StaticText<String>, StaticText<&'static str>, StaticText<Cow<'static, str>>;
}
