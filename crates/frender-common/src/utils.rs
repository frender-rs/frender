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

pub mod pin_project {
    use std::pin::Pin;

    macro_rules! impl_tuple {
        ($($name:ident ($($field:ident),+ $(,)?) ,)+) => {
            $(
                pub fn $name<$($field),+>($name: Pin<&mut ($($field,)+)>) -> ($(Pin<&mut $field>,)+) {
                    // SAFETY: pin projection
                    unsafe {
                        #[allow(non_snake_case)]
                        let ($($field,)+) = Pin::get_unchecked_mut($name);
                        ($(Pin::new_unchecked($field),)+)
                    }
                }
            )+
        };
    }

    impl_tuple!(
        tuple_1(R0,),
        tuple_2(R0, R1),
        tuple_3(R0, R1, R2),
        tuple_4(R0, R1, R2, R3),
        tuple_5(R0, R1, R2, R3, R4),
        tuple_6(R0, R1, R2, R3, R4, R5),
        tuple_7(R0, R1, R2, R3, R4, R5, R6),
        tuple_8(R0, R1, R2, R3, R4, R5, R6, R7),
        tuple_9(R0, R1, R2, R3, R4, R5, R6, R7, R8),
        tuple_10(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9),
        tuple_11(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10),
        tuple_12(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11),
        tuple_13(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12),
    );
}
