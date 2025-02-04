#![allow(non_snake_case)]

use std::{pin::Pin, task::Poll};

use frender_dom::render::RenderWithContext;

use crate::{
    element::{
        CsrElementRenderInitPinned, PinnedRenderInitKind, PinnedRenderInitOfKind, PinnedRenderStateKind, PinnedRenderStateKindPollRender, PinnedStateOfKind, PinnedUiHandleOfKind, PinnedUnmountedUiHandleOfKind,
        UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender, UnpinnedStateOfKind, UnpinnedUiHandleOfKind, UnpinnedUnmountedUiHandleOfKind,
    },
    proxy_csr_element, CsrElement, HtmlRenderContext, RenderHtml,
};

impl<E0: CsrElement> CsrElement for (E0,) {
    type RenderStateKind = E0::RenderStateKind;

    proxy_csr_element!(|this| this.0);
}

pub struct KindOfStates<TupleOfKinds>(super::Kind<TupleOfKinds>);

pub struct RenderInits<TupleOfRenderInits>(pub TupleOfRenderInits);

macro_rules! impl_render_for_tuple {
    ($($name:ident ($($field_idx:tt as $field:ident),+) ,)+) => {
        $(
            impl<Renderer: ?Sized + RenderWithContext, $($field: CsrElementRenderInitPinned<Renderer>),+> CsrElementRenderInitPinned<Renderer> for RenderInits<($($field,)+)> {
                type UiHandle = ($($field::UiHandle,)+);
                type State = ($($field::State,)+);

                fn render_init_pinned(self, render_context: &mut Renderer::RenderContext<'_>, state: Pin<&mut Self::State>) -> Self::UiHandle {
                    let state = frender_common::utils::pin_project::$name(state);
                    ($(
                        $field::render_init_pinned(
                            self.0.$field_idx,
                            render_context,
                            state.$field_idx,
                        )
                    ,)+)
                }
            }

            impl<$($field: PinnedRenderStateKind),+> PinnedRenderStateKind for KindOfStates<($($field,)+)> {
                type PinnedUiHandle<R: RenderHtml + ?Sized> = ($($field::PinnedUiHandle<R>,)+);
                type PinnedState<R: RenderHtml + ?Sized> = ($($field::PinnedState<R>,)+);
            }

            impl<$($field: PinnedRenderInitKind),+> PinnedRenderInitKind for KindOfStates<($($field,)+)> {
                type PinnedRenderInit<R: RenderHtml + ?Sized> = RenderInits<($($field::PinnedRenderInit<R>,)+)>;
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

                fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
                    //
                    self,
                    renderer: &mut Renderer,
                ) -> (
                    //
                    PinnedStateOfKind<Renderer, Self::RenderStateKind>,
                    PinnedRenderInitOfKind<Renderer, Self::RenderStateKind>,
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
    tuple_2 (0 as R0, 1 as R1),
    tuple_3 (0 as R0, 1 as R1, 2 as R2),
    tuple_4 (0 as R0, 1 as R1, 2 as R2, 3 as R3),
    tuple_5 (0 as R0, 1 as R1, 2 as R2, 3 as R3, 4 as R4),
    tuple_6 (0 as R0, 1 as R1, 2 as R2, 3 as R3, 4 as R4, 5 as R5),
    tuple_7 (0 as R0, 1 as R1, 2 as R2, 3 as R3, 4 as R4, 5 as R5, 6 as R6),
    tuple_8 (0 as R0, 1 as R1, 2 as R2, 3 as R3, 4 as R4, 5 as R5, 6 as R6, 7 as R7),
    tuple_9 (0 as R0, 1 as R1, 2 as R2, 3 as R3, 4 as R4, 5 as R5, 6 as R6, 7 as R7, 8 as R8),
    tuple_10(0 as R0, 1 as R1, 2 as R2, 3 as R3, 4 as R4, 5 as R5, 6 as R6, 7 as R7, 8 as R8, 9 as R9),
    tuple_11(0 as R0, 1 as R1, 2 as R2, 3 as R3, 4 as R4, 5 as R5, 6 as R6, 7 as R7, 8 as R8, 9 as R9, 10 as R10),
    tuple_12(0 as R0, 1 as R1, 2 as R2, 3 as R3, 4 as R4, 5 as R5, 6 as R6, 7 as R7, 8 as R8, 9 as R9, 10 as R10, 11 as R11),
    // tuple_13(0 as R0, 1 as R1, 2 as R2, 3 as R3, 4 as R4, 5 as R5, 6 as R6, 7 as R7, 8 as R8, 9 as R9, 10 as R10, 11 as R11, 12 as R12),
}
