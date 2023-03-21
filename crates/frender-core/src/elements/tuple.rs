#![allow(non_snake_case)]

use crate::{RenderState, UpdateRenderState};

impl<Ctx> RenderState<Ctx> for () {
    #[inline]
    fn unmount(self: std::pin::Pin<&mut Self>) {}

    #[inline]
    fn poll_reactive(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut Ctx,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let _ = cx;
        std::task::Poll::Ready(())
    }
}

impl<Ctx> UpdateRenderState<Ctx> for () {
    type State = ();

    #[inline]
    fn initialize_render_state(self, _ctx: &mut Ctx) -> Self::State {}

    #[inline]
    fn update_render_state(self, _: &mut Ctx, _: std::pin::Pin<&mut Self::State>) {}
}

impl<Ctx, R0: UpdateRenderState<Ctx>> UpdateRenderState<Ctx> for (R0,) {
    type State = R0::State;

    #[inline]
    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        R0::initialize_render_state(self.0, ctx)
    }

    #[inline]
    fn update_render_state(self, ctx: &mut Ctx, state: std::pin::Pin<&mut Self::State>) {
        R0::update_render_state(self.0, ctx, state)
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

            impl<Ctx, $($field: RenderState<Ctx>),+> RenderState<Ctx> for $name<$($field),+> {
                fn unmount(self: ::core::pin::Pin<&mut Self>) {
                    let this = self.project();
                    $( this.$field.unmount(); )+
                }

                #[inline]
                fn poll_reactive(
                    self: std::pin::Pin<&mut Self>,
                    ctx: &mut Ctx,
                    cx: &mut std::task::Context<'_>,
                ) -> std::task::Poll<()> {
                    let this = self.project();
                    match ($($field::poll_reactive(this.$field, ctx, cx) ,)+) {
                        #[allow(unused_variables)]
                        ( $(std::task::Poll::Ready($field @ ()),)+ ) => std::task::Poll::Ready(()),
                        _ => std::task::Poll::Pending,
                    }
                }
            }

            impl<Ctx, $($field: UpdateRenderState<Ctx>),+> UpdateRenderState<Ctx> for ($($field),+) {
                type State = $name <$($field::State),+>;

                #[inline]
                fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
                    let ($($field,)+) = self;

                    $name {$(
                        $field: $field::initialize_render_state($field, ctx),
                    )+}
                }

                #[inline]
                fn update_render_state(self, ctx: &mut Ctx, state: std::pin::Pin<&mut Self::State>) {
                    let state = state.project();
                    let ($($field,)+) = self;
                    $(
                        $field::update_render_state($field, ctx, state.$field);
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
