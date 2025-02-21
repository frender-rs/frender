#![allow(non_snake_case)]

use std::{pin::Pin, task::Poll};

use frender_common::reactive_value::RenderInitPinned;

use crate::{
    csr::element::{
        CsrElement, HtmlRenderContext, PinnedRenderStateKind, PinnedRenderStateKindPollRender, PinnedStateOfKind, PinnedUiHandleOfKind, PinnedUnmountedUiHandleOfKind, UnpinnedRenderStateKind,
        UnpinnedRenderStateKindPollRender, UnpinnedStateOfKind, UnpinnedUiHandleOfKind, UnpinnedUnmountedUiHandleOfKind,
    },
    html::RenderHtml,
    proxy_csr_element,
};

impl<E0: CsrElement> CsrElement for (E0,) {
    type RenderStateKind = E0::RenderStateKind;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = E0::PinnedRenderInit<R>;

    proxy_csr_element!(|this| this.0);
}

pub struct KindOfStates<TupleOfKinds>(super::Kind<TupleOfKinds>);

pub struct RenderInits<TupleOfRenderInits>(pub TupleOfRenderInits);

macro_rules! impl_render_for_tuple {
    ($($name:ident ($($field_idx:tt as $field:ident :: $state:ident :: $out:ident),+) ,)+) => {
        $(
            impl<Renderer: ?Sized, $($field: for<'r> RenderInitPinned<&'r mut Renderer, $state, Output = $out>, $state, $out),+> RenderInitPinned<&mut Renderer, ($($state,)+)> for RenderInits<($($field,)+)> {
                type Output = ($($out,)+);

                fn render_init_pinned(self, renderer: &mut Renderer, state: Pin<&mut ($($state,)+)>) -> Self::Output {
                    let state = frender_common::utils::pin_project::$name(state);
                    ($(
                        $field::render_init_pinned(
                            self.0.$field_idx,
                            renderer,
                            state.$field_idx,
                        )
                    ,)+)
                }
            }

            impl<$($field: PinnedRenderStateKind),+> PinnedRenderStateKind for KindOfStates<($($field,)+)> {
                type PinnedUiHandle<R: RenderHtml + ?Sized> = ($($field::PinnedUiHandle<R>,)+);
                type PinnedState<R: RenderHtml + ?Sized> = ($($field::PinnedState<R>,)+);
            }

            impl<$($field: UnpinnedRenderStateKind),+> UnpinnedRenderStateKind for KindOfStates<($($field,)+)> {
                type UnpinnedUiHandle<R: RenderHtml + ?Sized> = ($($field::UnpinnedUiHandle<R>,)+);
                type UnpinnedState<R: RenderHtml + ?Sized> = ($($field::UnpinnedState<R>,)+);
            }

            impl<$($field: PinnedRenderStateKindPollRender),+> PinnedRenderStateKindPollRender for KindOfStates<($($field,)+)> {
                fn pinned_poll_render<R: RenderHtml + ?Sized>(
                    //
                    renderer: &mut R,
                    state: Pin<&mut Self::PinnedState<R>>,
                    ui_handle: &mut Self::PinnedUiHandle<R>,
                    cx: &mut std::task::Context<'_>,
                ) -> Poll<()> {
                    let state = frender_common::utils::pin_project::$name(state);

                    match ($(
                        $field::pinned_poll_render(
                            renderer,
                            state.$field_idx,
                            &mut ui_handle.$field_idx,
                            cx,
                        )
                    ,)+) {
                        #[allow(unused_variables)]
                        ( $(std::task::Poll::Ready($field @ ()),)+ ) => std::task::Poll::Ready(()),
                        _ => std::task::Poll::Pending,
                    }
                }
            }

            impl<$($field: UnpinnedRenderStateKindPollRender),+> UnpinnedRenderStateKindPollRender for KindOfStates<($($field,)+)> {
                fn unpinned_poll_render<R: RenderHtml + ?Sized>(
                    //
                    renderer: &mut R,
                    state: &mut Self::UnpinnedState<R>,
                    ui_handle: &mut Self::UnpinnedUiHandle<R>,
                    cx: &mut std::task::Context<'_>,
                ) -> Poll<()> {
                    match ($(
                        $field::unpinned_poll_render(
                            renderer,
                            &mut state.$field_idx,
                            &mut ui_handle.$field_idx,
                            cx,
                        )
                    ,)+) {
                        #[allow(unused_variables)]
                        ( $(std::task::Poll::Ready($field @ ()),)+ ) => std::task::Poll::Ready(()),
                        _ => std::task::Poll::Pending,
                    }
                }
            }

            impl<$($field: CsrElement),+> CsrElement for ($($field,)+) {
                type RenderStateKind = KindOfStates<($($field::RenderStateKind,)+)>;
                type PinnedRenderInit<R: ?Sized + RenderHtml> = RenderInits<($($field::PinnedRenderInit<R>,)+)>;

                fn pinned_render_init<R: ?Sized + RenderHtml>(
                    self,
                    renderer: &mut R,
                ) -> (
                    //
                    PinnedStateOfKind<R, Self::RenderStateKind>,
                    Self::PinnedRenderInit<R>,
                ) {
                    let res = ($(
                        $field::pinned_render_init(
                            self.$field_idx,
                            renderer,
                        ),
                    )+);

                    (
                        ($(res.$field_idx.0,)+),
                        RenderInits(($(res.$field_idx.1,)+)),
                    )
                }

                fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
                    self,
                    render_context: &mut Ctx,
                    reused_state: Pin<&mut PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
                    unmounted_ui_handle: PinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
                ) -> PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
                    let reused_state = frender_common::utils::pin_project::$name(reused_state);
                    ($(
                        $field::pinned_render_init_by_reusing(
                            self.$field_idx,
                            render_context,
                            reused_state.$field_idx,
                            unmounted_ui_handle.$field_idx,
                        ),
                    )+)
                }

                fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
                    //
                    self,
                    renderer: &mut Renderer,
                    state: Pin<&mut PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
                    ui_handle: &mut PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
                ) {
                    let state = frender_common::utils::pin_project::$name(state);
                    $(
                        $field::pinned_render_update(
                            self.$field_idx,
                            renderer,
                            state.$field_idx,
                            &mut ui_handle.$field_idx,
                        );
                    )+
                }

                fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
                    //
                    self,
                    render_context: &mut Ctx,
                ) -> (
                    //
                    UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
                    UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
                ) {
                    let res = ($(
                        $field::unpinned_render_init(
                            self.$field_idx,
                            render_context,
                        ),
                    )+);
                    (
                        ($(res.$field_idx.0,)+),
                        ($(res.$field_idx.1,)+),
                    )
                }

                fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
                    self,
                    render_context: &mut Ctx,
                    reused_state: &mut UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
                    unmounted_ui_handle: UnpinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
                ) -> UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
                    ($(
                        $field::unpinned_render_init_by_reusing(
                            self.$field_idx,
                            render_context,
                            &mut reused_state.$field_idx,
                            unmounted_ui_handle.$field_idx,
                        ),
                    )+)
                }

                fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
                    //
                    self,
                    renderer: &mut Renderer,
                    state: &mut UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
                    ui_handle: &mut UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
                ) {
                    $(
                        $field::unpinned_render_update(
                            self.$field_idx,
                            renderer,
                            &mut state.$field_idx,
                            &mut ui_handle.$field_idx,
                        );
                    )+
                }
            }
        )+
    };
}

impl_render_for_tuple! {
    tuple_2 (0 as R0::S0::OUT0, 1 as R1::S1::OUT1),
    tuple_3 (0 as R0::S0::OUT0, 1 as R1::S1::OUT1, 2 as R2::S2::OUT2),
    tuple_4 (0 as R0::S0::OUT0, 1 as R1::S1::OUT1, 2 as R2::S2::OUT2, 3 as R3::S3::OUT3),
    tuple_5 (0 as R0::S0::OUT0, 1 as R1::S1::OUT1, 2 as R2::S2::OUT2, 3 as R3::S3::OUT3, 4 as R4::S4::OUT4),
    tuple_6 (0 as R0::S0::OUT0, 1 as R1::S1::OUT1, 2 as R2::S2::OUT2, 3 as R3::S3::OUT3, 4 as R4::S4::OUT4, 5 as R5::S5::OUT5),
    tuple_7 (0 as R0::S0::OUT0, 1 as R1::S1::OUT1, 2 as R2::S2::OUT2, 3 as R3::S3::OUT3, 4 as R4::S4::OUT4, 5 as R5::S5::OUT5, 6 as R6::S6::OUT6),
    tuple_8 (0 as R0::S0::OUT0, 1 as R1::S1::OUT1, 2 as R2::S2::OUT2, 3 as R3::S3::OUT3, 4 as R4::S4::OUT4, 5 as R5::S5::OUT5, 6 as R6::S6::OUT6, 7 as R7::S7::OUT7),
    tuple_9 (0 as R0::S0::OUT0, 1 as R1::S1::OUT1, 2 as R2::S2::OUT2, 3 as R3::S3::OUT3, 4 as R4::S4::OUT4, 5 as R5::S5::OUT5, 6 as R6::S6::OUT6, 7 as R7::S7::OUT7, 8 as R8::S8::OUT8),
    tuple_10(0 as R0::S0::OUT0, 1 as R1::S1::OUT1, 2 as R2::S2::OUT2, 3 as R3::S3::OUT3, 4 as R4::S4::OUT4, 5 as R5::S5::OUT5, 6 as R6::S6::OUT6, 7 as R7::S7::OUT7, 8 as R8::S8::OUT8, 9 as R9::S9::OUT9),
    tuple_11(0 as R0::S0::OUT0, 1 as R1::S1::OUT1, 2 as R2::S2::OUT2, 3 as R3::S3::OUT3, 4 as R4::S4::OUT4, 5 as R5::S5::OUT5, 6 as R6::S6::OUT6, 7 as R7::S7::OUT7, 8 as R8::S8::OUT8, 9 as R9::S9::OUT9, 10 as R10::S10::OUT10),
    tuple_12(0 as R0::S0::OUT0, 1 as R1::S1::OUT1, 2 as R2::S2::OUT2, 3 as R3::S3::OUT3, 4 as R4::S4::OUT4, 5 as R5::S5::OUT5, 6 as R6::S6::OUT6, 7 as R7::S7::OUT7, 8 as R8::S8::OUT8, 9 as R9::S9::OUT9, 10 as R10::S10::OUT10, 11 as R11::S11::OUT11),
    // tuple_13(0 as R0::S0::OUT0, 1 as R1::S1::OUT1, 2 as R2::S2::OUT2, 3 as R3::S3::OUT3, 4 as R4::S4::OUT4, 5 as R5::S5::OUT5, 6 as R6::S6::OUT6, 7 as R7::S7::OUT7, 8 as R8::S8::OUT8, 9 as R9::S9::OUT9, 10 as R10::S10::OUT10, 11 as R11::S11::OUT11, 12 as R12::S12::OUT12),
}
