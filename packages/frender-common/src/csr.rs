use std::pin::Pin;

pub trait StateUnmount {
    fn state_unmount(self: Pin<&mut Self>);
}

impl<T: StateUnmount> StateUnmount for Option<T> {
    fn state_unmount(self: Pin<&mut Self>) {
        if let Some(this) = self.as_pin_mut() {
            this.state_unmount()
        }
    }
}

impl StateUnmount for () {
    fn state_unmount(self: Pin<&mut Self>) {}
}

macro_rules! impl_render_for_tuple {
    ($($name:ident ($($field_var:ident as $field:ident),+) ,)+) => {
        $(
            impl<$($field: StateUnmount),+> StateUnmount for ($($field,)+) {
                fn state_unmount(self: Pin<&mut Self>) {
                    let ($($field_var,)+) = crate::utils::pin_project::$name(self);
                    $( $field_var.state_unmount(); )+
                }
            }
        )+
    };
}

impl_render_for_tuple! {
    tuple_2 (r0 as R0, r1 as R1),
    tuple_3 (r0 as R0, r1 as R1, r2 as R2),
    tuple_4 (r0 as R0, r1 as R1, r2 as R2, r3 as R3),
    tuple_5 (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4),
    tuple_6 (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5),
    tuple_7 (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6),
    tuple_8 (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6, r7 as R7),
    tuple_9 (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6, r7 as R7, r8 as R8),
    tuple_10(r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6, r7 as R7, r8 as R8, r9 as R9),
    tuple_11(r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6, r7 as R7, r8 as R8, r9 as R9, r10 as R10),
    tuple_12(r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6, r7 as R7, r8 as R8, r9 as R9, r10 as R10, r11 as R11),
    // tuple_13(r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6, r7 as R7, r8 as R8, r9 as R9, r10 as R10, r11 as R11, r12 as R12),
}
