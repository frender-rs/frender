#![allow(non_snake_case)]

use std::pin::Pin;

use crate::{Element, HtmlRenderContext, RenderHtml, RenderStateKind, RenderStateOfContext, UnpinnedRenderStateOfContext};

pub enum KindOfNoState {}

impl crate::RenderStateKindPinned for KindOfNoState {
    type RenderState<R: RenderHtml + ?Sized> = ();
}
impl crate::RenderStateKindUnpinned for KindOfNoState {
    type UnpinnedRenderState<R: RenderHtml + ?Sized> = ();
}

impl Element for () {
    type RenderStateKind = KindOfNoState;

    fn render_update<Ctx: ?Sized + HtmlRenderContext>(self, _: &mut Ctx, _: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>) {}
    fn render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(self, _: &mut Ctx, _: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>) {}
    fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, _: &mut Ctx, _: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>, _: bool) {}

    crate::impl_unpinned_render_for_unpin! {}
}

impl<E0: Element> Element for (E0,) {
    type RenderStateKind = E0::RenderStateKind;

    fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>, force_reposition: bool) {
        self.0.render_update_maybe_reposition(render_context, render_state, force_reposition)
    }

    fn render_update<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>) {
        self.0.render_update(render_context, render_state)
    }

    fn render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>) {
        self.0.render_update_force_reposition(render_context, render_state)
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>) {
        self.0.unpinned_render_update(render_context, render_state)
    }

    fn unpinned_render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>) {
        self.0.unpinned_render_update_force_reposition(render_context, render_state)
    }

    fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(self, render_context: &mut Ctx, render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>, force_reposition: bool) {
        self.0.unpinned_render_update_maybe_reposition(render_context, render_state, force_reposition)
    }
}

pub struct KindOfStates<TupleOfKinds>(super::Kind<TupleOfKinds>);

macro_rules! impl_render_for_tuple {
    ($($name:ident ($($field_var:ident as $field:ident),+) ,)+) => {
        $(
            impl<$($field: RenderStateKind),+> crate::RenderStateKindPinned for KindOfStates<($($field,)+)> {
                type RenderState< R: RenderHtml+?Sized> = ($($field::RenderState<R>,)+);
            }
            impl<$($field: RenderStateKind),+> crate::RenderStateKindUnpinned for KindOfStates<($($field,)+)> {
                type UnpinnedRenderState< R: RenderHtml+?Sized> = ($($field::UnpinnedRenderState<R>,)+);
            }

            impl<$($field: Element),+> Element for ($($field,)+) {
                type RenderStateKind = KindOfStates<($($field::RenderStateKind,)+)>;

                fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(
                    self,
                    render_context: &mut Ctx,
                    render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>,
                    force_reposition: bool,
                ) {
                    let ($($field,)+) = self;
                    let ($($field_var,)+) = frender_common::utils::pin_project::$name(render_state);
                    $($field::render_update_maybe_reposition($field, render_context, $field_var, force_reposition);)+
                }

                fn render_update<Ctx: ?Sized + HtmlRenderContext>(
                    self,
                    render_context: &mut Ctx,
                    render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>,
                ) {
                    let ($($field,)+) = self;
                    let ($($field_var,)+) = frender_common::utils::pin_project::$name(render_state);
                    $($field::render_update($field, render_context, $field_var);)+
                }

                fn render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(
                    self,
                    render_context: &mut Ctx,
                    render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>,
                ) {
                    let ($($field,)+) = self;
                    let ($($field_var,)+) = frender_common::utils::pin_project::$name(render_state);
                    $($field::render_update_force_reposition($field, render_context, $field_var);)+
                }

                fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
                    self,
                    render_context: &mut Ctx,
                    render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
                ) {
                    match self {
                        ($($field,)+) => {
                            match render_state {
                                ($($field_var,)+) => {$(
                                    $field::unpinned_render_update($field,  render_context, $field_var);
                                )+}
                            }
                        }
                    }
                }

                fn unpinned_render_update_force_reposition<Ctx: ?Sized + HtmlRenderContext>(
                    self,
                    render_context: &mut Ctx,
                    render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
                ) {
                    match self {
                        ($($field,)+) => {
                            match render_state {
                                ($($field_var,)+) => {$(
                                    $field::unpinned_render_update_force_reposition($field,  render_context, $field_var);
                                )+}
                            }
                        }
                    }
                }

                fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(
                    self,
                    render_context: &mut Ctx,
                    render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
                    force_reposition: bool,
                ) {
                    match self {
                        ($($field,)+) => {
                            match render_state {
                                ($($field_var,)+) => {$(
                                    $field::unpinned_render_update_maybe_reposition($field,  render_context, $field_var, force_reposition);
                                )+}
                            }
                        }
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
