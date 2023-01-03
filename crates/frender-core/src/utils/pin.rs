use std::{ops::DerefMut, pin::Pin};

/// See [`Pin::as_deref_mut`]
pub fn pin_as_deref_mut<P>(this: Pin<&mut Pin<P>>) -> Pin<&mut P::Target>
where
    P: DerefMut,
{
    // SAFETY: See [`Pin::as_deref_mut`]
    unsafe { this.get_unchecked_mut() }.as_mut()
}

pub fn pin_project_map_array<T, const N: usize>(
    this: Pin<&mut [T; N]>,
    mut f: impl FnMut(Pin<&mut T>),
) {
    for item in unsafe { this.get_unchecked_mut() }.iter_mut() {
        f(unsafe { Pin::new_unchecked(item) })
    }
}
