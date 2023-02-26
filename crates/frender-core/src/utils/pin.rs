use std::{ops::DerefMut, pin::Pin};

/// See [`Pin::as_deref_mut`]
pub fn pin_as_deref_mut<P>(this: Pin<&mut Pin<P>>) -> Pin<&mut P::Target>
where
    P: DerefMut,
{
    // SAFETY: See [`Pin::as_deref_mut`]
    unsafe { this.get_unchecked_mut() }.as_mut()
}

pub fn pin_downcast_mut<T: std::any::Any>(
    this: Pin<&mut dyn std::any::Any>,
) -> Option<Pin<&mut T>> {
    // SAFETY: get_unchecked_mut is not used to mutate this
    let this = unsafe { this.get_unchecked_mut() };
    if let Some(this) = this.downcast_mut::<T>() {
        // SAFETY: this comes from pinned pointer
        Some(unsafe { Pin::new_unchecked(this) })
    } else {
        None
    }
}

pub fn pin_project_map_array<T, const N: usize>(
    this: Pin<&mut [T; N]>,
    mut f: impl FnMut(Pin<&mut T>),
) {
    for item in unsafe { this.get_unchecked_mut() }.iter_mut() {
        f(unsafe { Pin::new_unchecked(item) })
    }
}
