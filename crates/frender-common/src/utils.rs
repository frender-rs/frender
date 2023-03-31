use std::pin::Pin;

pub fn pin_project_index_slice<T>(this: Pin<&mut [T]>, idx: usize) -> Pin<&mut T> {
    // SAFETY: pin projection. see this issue:
    // https://github.com/rust-lang/rust/issues/104108
    unsafe { Pin::new_unchecked(&mut this.get_unchecked_mut()[idx]) }
}

pub fn pin_project_index_vec<T>(this: Pin<&mut Vec<T>>, idx: usize) -> Pin<&mut T> {
    // SAFETY: pin projection. see this issue:
    // https://github.com/rust-lang/rust/issues/104108
    unsafe { Pin::new_unchecked(&mut this.get_unchecked_mut()[idx]) }
}

/// See [`Pin::as_deref_mut`]
#[must_use = "`self` will be dropped if the result is not used"]
pub fn pin_as_deref_mut<P>(this: Pin<&mut Pin<P>>) -> Pin<&mut P::Target>
where
    P: std::ops::DerefMut,
{
    // SAFETY: See [`Pin::as_deref_mut`]
    unsafe { this.get_unchecked_mut() }.as_mut()
}
