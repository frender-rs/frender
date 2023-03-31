use std::pin::Pin;

pub fn pin_project_map_array<T, const N: usize>(
    this: Pin<&mut [T; N]>,
    mut f: impl FnMut(Pin<&mut T>),
) {
    for item in unsafe { this.get_unchecked_mut() }.iter_mut() {
        f(unsafe { Pin::new_unchecked(item) })
    }
}
