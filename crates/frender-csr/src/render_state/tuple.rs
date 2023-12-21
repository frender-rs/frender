#![allow(non_snake_case)]

use crate::RenderState;

impl<R> RenderState<R> for () {
    #[inline]
    fn unmount(self: std::pin::Pin<&mut Self>, _: &mut R) {}

    #[inline]
    fn state_unmount(self: std::pin::Pin<&mut Self>) {}

    #[inline]
    fn poll_render(self: std::pin::Pin<&mut Self>, _: &mut R, _: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        std::task::Poll::Ready(())
    }
}

macro_rules! impl_render_for_tuple {
    ($($name:ident ($($field_var:ident as $field:ident),+) ,)+) => {
        $(
            impl<R,$($field: RenderState<R>),+> RenderState<R> for ($($field,)+) {
                fn unmount(self: ::core::pin::Pin<&mut Self>, renderer: &mut R) {
                    let ($($field,)+) = frender_common::utils::pin_project::$name(self);
                    $( $field.unmount(renderer); )+
                }

                fn state_unmount(self: std::pin::Pin<&mut Self>) {
                    let ($($field,)+) = frender_common::utils::pin_project::$name(self);
                    $( $field.state_unmount(); )+
                }

                fn poll_render(
                    self: std::pin::Pin<&mut Self>,
                    renderer: &mut R,
                    cx: &mut std::task::Context<'_>,
                ) -> std::task::Poll<()> {
                    let ($($field,)+) = frender_common::utils::pin_project::$name(self);

                    match ($($field::poll_render($field, renderer, cx) ,)+) {
                        #[allow(unused_variables)]
                        ( $(std::task::Poll::Ready($field @ ()),)+ ) => std::task::Poll::Ready(()),
                        _ => std::task::Poll::Pending,
                    }
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
