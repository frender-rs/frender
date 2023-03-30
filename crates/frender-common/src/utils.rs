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
