#![allow(non_snake_case)]

use crate::{Element, RenderState};

impl RenderState for () {
    #[inline]
    fn unmount(self: std::pin::Pin<&mut Self>) {}

    #[inline]
    fn state_unmount(self: std::pin::Pin<&mut Self>) {}

    #[inline]
    fn poll_csr(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let _ = cx;
        std::task::Poll::Ready(())
    }
}

impl Element for () {
    type CsrState = ();

    #[inline]
    fn into_csr_state(self, _ctx: &mut crate::CsrContext) -> Self::CsrState {}

    fn update_csr_state_maybe_reposition(
        self,
        _: &mut crate::CsrContext,
        _: std::pin::Pin<&mut Self::CsrState>,
        _: bool,
    ) {
    }
}

impl<R0: Element> Element for (R0,) {
    type CsrState = R0::CsrState;

    #[inline]
    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
        R0::into_csr_state(self.0, ctx)
    }

    #[inline]
    fn update_csr_state(
        self,
        ctx: &mut crate::CsrContext,
        state: std::pin::Pin<&mut Self::CsrState>,
    ) {
        R0::update_csr_state(self.0, ctx, state)
    }

    #[inline]
    fn update_csr_state_force_reposition(
        self,
        ctx: &mut crate::CsrContext,
        state: std::pin::Pin<&mut Self::CsrState>,
    ) {
        R0::update_csr_state_force_reposition(self.0, ctx, state)
    }

    fn update_csr_state_maybe_reposition(
        self,
        ctx: &mut crate::CsrContext,
        state: std::pin::Pin<&mut Self::CsrState>,
        force_reposition: bool,
    ) {
        R0::update_csr_state_maybe_reposition(self.0, ctx, state, force_reposition)
    }
}

macro_rules! impl_render_for_tuple {
    ($($name:ident ($($field:ident),+) ,)+) => {
        $(
            ::pin_project_lite::pin_project! {
                #[derive(Default)]
                pub struct $name <$($field),+> {
                    $(#[pin] $field: $field),+
                }
            }

            impl<$($field: RenderState),+> RenderState for $name<$($field),+> {
                fn unmount(self: ::core::pin::Pin<&mut Self>) {
                    let this = self.project();
                    $( this.$field.unmount(); )+
                }

                fn state_unmount(self: std::pin::Pin<&mut Self>) {
                    let this = self.project();
                    $( this.$field.state_unmount(); )+
                }

                #[inline]
                fn poll_csr(
                    self: std::pin::Pin<&mut Self>,
                    ctx: &mut crate::CsrContext,
                    cx: &mut std::task::Context<'_>,
                ) -> std::task::Poll<()> {
                    let this = self.project();
                    match ($($field::poll_csr(this.$field, ctx, cx) ,)+) {
                        #[allow(unused_variables)]
                        ( $(std::task::Poll::Ready($field @ ()),)+ ) => std::task::Poll::Ready(()),
                        _ => std::task::Poll::Pending,
                    }
                }
            }

            impl<$($field: Element),+> Element for ($($field),+) {
                type CsrState = $name <$($field::CsrState),+>;

                #[inline]
                fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
                    let ($($field,)+) = self;

                    $name {$(
                        $field: $field::into_csr_state($field, ctx),
                    )+}
                }

                fn update_csr_state(self, ctx: &mut crate::CsrContext, state: std::pin::Pin<&mut Self::CsrState>) {
                    let state = state.project();
                    let ($($field,)+) = self;
                    $(
                        $field::update_csr_state($field, ctx, state.$field);
                    )+
                }

                fn update_csr_state_force_reposition(
                    self,
                    ctx: &mut crate::CsrContext,
                    state: std::pin::Pin<&mut Self::CsrState>,
                ) {
                    let state = state.project();
                    let ($($field,)+) = self;
                    $(
                        $field::update_csr_state_force_reposition($field, ctx, state.$field);
                    )+
                }

                fn update_csr_state_maybe_reposition(
                    self,
                    ctx: &mut crate::CsrContext,
                    state: std::pin::Pin<&mut Self::CsrState>,
                    force_reposition: bool,
                ) {
                    let state = state.project();
                    let ($($field,)+) = self;
                    $(
                        $field::update_csr_state_maybe_reposition($field, ctx, state.$field, force_reposition);
                    )+
                }
            }
        )+
    };
}

impl_render_for_tuple! {
    Tuple2Rendered (R0, R1),
    Tuple3Rendered (R0, R1, R2),
    Tuple4Rendered (R0, R1, R2, R3),
    Tuple5Rendered (R0, R1, R2, R3, R4),
    Tuple6Rendered (R0, R1, R2, R3, R4, R5),
    Tuple7Rendered (R0, R1, R2, R3, R4, R5, R6),
    Tuple8Rendered (R0, R1, R2, R3, R4, R5, R6, R7),
    Tuple9Rendered (R0, R1, R2, R3, R4, R5, R6, R7, R8),
    Tuple10Rendered(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9),
    Tuple11Rendered(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10),
    Tuple12Rendered(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11),
    Tuple13Rendered(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12),
}
