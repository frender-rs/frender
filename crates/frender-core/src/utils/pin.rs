use std::{ops::DerefMut, pin::Pin};

/// See [`Pin::as_deref_mut`]
pub fn pin_as_deref_mut<P>(this: Pin<&mut Pin<P>>) -> Pin<&mut P::Target>
where
    P: DerefMut,
{
    // SAFETY: See [`Pin::as_deref_mut`]
    unsafe { this.get_unchecked_mut() }.as_mut()
}
