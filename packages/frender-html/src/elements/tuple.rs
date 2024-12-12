#![allow(non_snake_case)]

use std::{pin::Pin, task::Poll};

use crate::{
    element::{
        PinMutRenderInitStates, PinMutRenderInitStatesOfKind, PinnedMutRenderStatesOfKind, PinnedRenderStateKind, PinnedRenderStateKindPollRender, PinnedUiHandleOfKind, RenderStates, UnpinnedMutRenderStatesOfKind,
        UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender, UnpinnedRenderStatesOfKind,
    },
    CsrElement, HtmlRenderContext, RenderHtml,
};

impl<E0: CsrElement> CsrElement for (E0,) {
    type RenderStateKind = E0::RenderStateKind;

    fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        self.0.pinned_render_init(render_context, states)
    }

    fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        self.0.pinned_render_update(render_context, states)
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> crate::element::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        self.0.unpinned_render_init(render_context)
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        self.0.unpinned_render_update(render_context, states)
    }
}

pub struct KindOfStates<TupleOfKinds>(super::Kind<TupleOfKinds>);

macro_rules! impl_render_for_tuple {
    ($($name:ident ($($field_idx:tt as $field:ident),+) ,)+) => {
        $(
            impl<$($field: PinnedRenderStateKind),+> PinnedRenderStateKind for KindOfStates<($($field,)+)> {
                type PinnedUiHandle<R: RenderHtml + ?Sized> = ($($field::PinnedUiHandle<R>,)+);
                type PinnedNonReactiveState<R: RenderHtml + ?Sized> = ($($field::PinnedNonReactiveState<R>,)+);
                type PinnedReactiveState = ($($field::PinnedReactiveState,)+);
            }

            impl<$($field: UnpinnedRenderStateKind),+> UnpinnedRenderStateKind for KindOfStates<($($field,)+)> {
                type UnpinnedUiHandle<R: RenderHtml + ?Sized> = ($($field::UnpinnedUiHandle<R>,)+);
                type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = ($($field::UnpinnedNonReactiveState<R>,)+);
                type UnpinnedReactiveState = ($($field::UnpinnedReactiveState,)+);
            }

            impl<$($field: PinnedRenderStateKindPollRender),+> PinnedRenderStateKindPollRender for KindOfStates<($($field,)+)> {
                fn pinned_poll_render<R: RenderHtml + ?Sized>(
                    //
                    renderer: &mut R,
                    states: PinnedMutRenderStatesOfKind<Self, R>,
                    cx: &mut std::task::Context<'_>,
                ) -> Poll<()> {
                    let ui_handle = states.ui_handle;
                    let non_reactive_state = frender_common::utils::pin_project::$name(states.non_reactive_state);
                    let reactive_state = frender_common::utils::pin_project::$name(states.reactive_state);

                    match ($(
                        $field::pinned_poll_render(renderer, RenderStates {
                            ui_handle: &mut ui_handle.$field_idx,
                            non_reactive_state: non_reactive_state.$field_idx,
                            reactive_state: reactive_state.$field_idx,
                        }, cx)
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
                    states: UnpinnedMutRenderStatesOfKind<Self, R>,
                    cx: &mut std::task::Context<'_>,
                ) -> Poll<()> {
                    match ($(
                        $field::unpinned_poll_render(renderer, RenderStates {
                            ui_handle: &mut states.ui_handle.$field_idx,
                            non_reactive_state: &mut states.non_reactive_state.$field_idx,
                            reactive_state: &mut states.reactive_state.$field_idx,
                        }, cx)
                    ,)+) {
                        #[allow(unused_variables)]
                        ( $(std::task::Poll::Ready($field @ ()),)+ ) => std::task::Poll::Ready(()),
                        _ => std::task::Poll::Pending,
                    }
                }
            }

            impl<$($field: CsrElement),+> CsrElement for ($($field,)+) {
                type RenderStateKind = KindOfStates<($($field::RenderStateKind,)+)>;

                fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
                    //
                    self,
                    render_context: &mut Ctx,
                    states: PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
                ) -> PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
                    let non_reactive_state = frender_common::utils::pin_project::$name(states.non_reactive_state);
                    let reactive_state = frender_common::utils::pin_project::$name(states.reactive_state);

                    ($(
                        self.$field_idx.pinned_render_init(
                            render_context,
                            PinMutRenderInitStates {
                                non_reactive_state: non_reactive_state.$field_idx,
                                reactive_state: reactive_state.$field_idx,
                            },
                        )
                    ,)+)
                }

                fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
                    //
                    self,
                    render_context: &mut Ctx,
                    states: PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
                ) {
                    let non_reactive_state = frender_common::utils::pin_project::$name(states.non_reactive_state);
                    let reactive_state = frender_common::utils::pin_project::$name(states.reactive_state);
                    $(
                        self.$field_idx.pinned_render_update(
                            render_context,
                            RenderStates {
                                ui_handle: &mut states.ui_handle.$field_idx,
                                non_reactive_state: non_reactive_state.$field_idx,
                                reactive_state: reactive_state.$field_idx,
                            },
                        );
                    )+
                }

                fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
                    //
                    self,
                    render_context: &mut Ctx,
                ) -> UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
                    let states = ($(
                        self.$field_idx.unpinned_render_init(render_context),
                    )+);

                    RenderStates {
                        ui_handle: ($(states.$field_idx.ui_handle,)+),
                        non_reactive_state: ($(states.$field_idx.non_reactive_state,)+),
                        reactive_state: ($(states.$field_idx.reactive_state,)+),
                    }
                }

                fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
                    //
                    self,
                    render_context: &mut Ctx,
                    states: UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
                ) {
                    $(
                        self.$field_idx.unpinned_render_update(
                            render_context,
                            RenderStates {
                                ui_handle: &mut states.ui_handle.$field_idx,
                                non_reactive_state: &mut states.non_reactive_state.$field_idx,
                                reactive_state: &mut states.reactive_state.$field_idx,
                            },
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
