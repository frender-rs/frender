mod update_value;
pub use update_value::*;

mod maybe;
pub use maybe::*;

macro_rules! auto_impl_update {
    ($($ty:ty),* $(,)?) => {$(
        impl<T: UpdateValueWithState<$ty>> MaybeUpdateValueWithState<$ty> for T {
            type State = T::State;

            #[inline]
            fn maybe_update_value_with_state(
                this: Self,
                state: &mut Self::State,
                update: impl FnOnce(&$ty),
                _: impl FnOnce(),
            ) {
                T::update_value_with_state(this, state, update)
            }
        }
    )*};
}

auto_impl_update! {
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
    f32, f64,
    bool,
    str,
}
