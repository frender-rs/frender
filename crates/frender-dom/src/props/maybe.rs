use super::UpdateValue;

pub trait MaybeUpdateValue<V: ?Sized> {
    type State: Default;

    fn maybe_update_value(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&V),
        remove: impl FnOnce(),
    );
}

impl<V: ?Sized> MaybeUpdateValue<V> for () {
    type State = ();

    #[inline]
    fn maybe_update_value(_: Self, _: &mut Self::State, _: impl FnOnce(&V), _: impl FnOnce()) {}
}

impl<T: UpdateValue<V>, V: ?Sized> MaybeUpdateValue<V> for Option<T> {
    type State = T::State;

    fn maybe_update_value(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&V),
        remove: impl FnOnce(),
    ) {
        match this {
            Some(this) => T::update_value(this, state, update),
            None => {
                remove();
                *state = Default::default();
            }
        }
    }
}

macro_rules! auto_impl_maybe_update {
    ($($ty:ty),* $(,)?) => {$(
        impl<T: UpdateValue<$ty>> MaybeUpdateValue<$ty> for T {
            type State = T::State;

            #[inline]
            fn maybe_update_value(
                this: Self,
                state: &mut Self::State,
                update: impl FnOnce(&$ty),
                _: impl FnOnce(),
            ) {
                T::update_value(this, state, update)
            }
        }
    )*};
}

auto_impl_maybe_update! {
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
    f32, f64,
    bool,
    str,
}
