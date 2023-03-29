pub trait MaybeUpdateValue<V> {
    fn maybe_update_value(this: Self, update: impl FnOnce(V), remove: impl FnOnce());
}

impl<V> MaybeUpdateValue<V> for () {
    #[inline(always)]
    fn maybe_update_value(_: Self, _: impl FnOnce(V), _: impl FnOnce()) {}
}

impl<V, T: MaybeUpdateValue<V>> MaybeUpdateValue<V> for Option<T> {
    #[inline(always)]
    fn maybe_update_value(this: Self, update: impl FnOnce(V), remove: impl FnOnce()) {
        match this {
            Some(this) => T::maybe_update_value(this, update, remove),
            None => remove(),
        }
    }
}

macro_rules! auto_impl_update {
    ($($ty:ty),* $(,)?) => {$(
        impl MaybeUpdateValue<$ty> for $ty {
            #[inline(always)]
            fn maybe_update_value(
                this: Self,
                update: impl FnOnce($ty),
                _: impl FnOnce(),
            ) {
                update(this)
            }
        }
    )*};
}

auto_impl_update! {
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
    f32, f64,
    bool,
}
