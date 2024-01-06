#![allow(non_snake_case)]

use crate::{Element, RenderHtml};

impl Element for () {
    type RenderState<PEH: ?Sized, R: RenderHtml + ?Sized> = ();

    #[cfg(feature = "render_into")]
    fn render_into<'s, Renderer: RenderHtml>(
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: PinMutMaybeUninit<'s, Self::RenderState<PEH, Renderer>>,
    ) -> std::pin::Pin<&'s mut Self::RenderState<PEH, Renderer>> {
        render_state.write(())
    }

    fn render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
        force_reposition: bool,
    ) {
    }

    fn render_update<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>)
    where
        Self: Sized,
    {
    }

    fn render_update_force_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>)
    where
        Self: Sized,
    {
    }

    crate::impl_unpinned_render_for_unpin! {}
}

impl<E0: Element> Element for (E0,) {
    type RenderState<PEH: ?Sized, R: RenderHtml + ?Sized> = E0::RenderState<PEH, R>;

    #[cfg(feature = "render_into")]
    fn render_into<'s, Renderer: RenderHtml>(
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: PinMutMaybeUninit<'s, Self::RenderState<PEH, Renderer>>,
    ) -> std::pin::Pin<&'s mut Self::RenderState<PEH, Renderer>> {
        self.0.render_into(peh, renderer, render_state)
    }

    fn render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
        force_reposition: bool,
    ) {
        self.0.render_update_maybe_reposition(peh, renderer, render_state, force_reposition)
    }

    fn render_update<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>)
    where
        Self: Sized,
    {
        self.0.render_update(peh, renderer, render_state)
    }

    fn render_update_force_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>)
    where
        Self: Sized,
    {
        self.0.render_update_force_reposition(peh, renderer, render_state)
    }

    type UnpinnedRenderState<PEH: ?Sized, R: RenderHtml + ?Sized> = E0::UnpinnedRenderState<PEH, R>;

    fn unpinned_render_update<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>) {
        self.0.unpinned_render_update(peh, renderer, render_state)
    }

    fn unpinned_render_update_force_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>) {
        self.0.unpinned_render_update_force_reposition(peh, renderer, render_state)
    }

    fn unpinned_render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
        force_reposition: bool,
    ) {
        self.0.unpinned_render_update_maybe_reposition(peh, renderer, render_state, force_reposition)
    }
}

macro_rules! impl_render_for_tuple {
    ($($name:ident ($($field_var:ident as $field:ident),+) ,)+) => {
        $(
            impl<$($field: Element),+> Element for ($($field,)+) {
                type RenderState<PEH: ?Sized, R: RenderHtml+?Sized> = ($($field::RenderState<PEH, R>,)+);

                #[cfg(feature = "render_into")]
                fn render_into<'s, Renderer: RenderHtml>(
                    self,
                    peh: &mut PEH,
                    renderer: &mut Renderer,
                    render_state: PinMutMaybeUninit<'s, Self::RenderState<PEH, Renderer>>,
                ) -> std::pin::Pin<&'s mut Self::RenderState<PEH, Renderer>> {
                    let ($($field,)+) = self;

                    crate::pin_mut_maybe_uninit::PinMutInitializeWith::pin_mut_initialize_with(
                        render_state,
                        ($(
                            |render_state| $field::render_into($field, peh, renderer, render_state)
                        ,)+)
                    )
                }

                fn render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml+?Sized>(
                    self,
                    peh: &mut PEH,
                    renderer: &mut Renderer,
                    render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
                    force_reposition: bool,
                ) {
                    let ($($field,)+) = self;
                    let ($($field_var,)+) = frender_common::utils::pin_project::$name(render_state);
                    $($field::render_update_maybe_reposition($field, peh, renderer, $field_var, force_reposition);)+
                }

                fn render_update<PEH: ?Sized, Renderer: RenderHtml+?Sized>(
                    self,
                    peh: &mut PEH,
                    renderer: &mut Renderer,
                    render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
                ) {
                    let ($($field,)+) = self;
                    let ($($field_var,)+) = frender_common::utils::pin_project::$name(render_state);
                    $($field::render_update($field, peh, renderer, $field_var);)+
                }

                fn render_update_force_reposition<PEH: ?Sized, Renderer: RenderHtml+?Sized>(
                    self,
                    peh: &mut PEH,
                    renderer: &mut Renderer,
                    render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
                ) {
                    let ($($field,)+) = self;
                    let ($($field_var,)+) = frender_common::utils::pin_project::$name(render_state);
                    $($field::render_update_force_reposition($field, peh, renderer, $field_var);)+
                }

                type UnpinnedRenderState<PEH: ?Sized, R: RenderHtml+?Sized> = ($($field::UnpinnedRenderState<PEH, R>,)+);

                fn unpinned_render_update<PEH: ?Sized, Renderer: RenderHtml+?Sized>(
                    self,
                    peh: &mut PEH,
                    renderer: &mut Renderer,
                    render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
                ) {
                    match self {
                        ($($field,)+) => {
                            match render_state {
                                ($($field_var,)+) => {$(
                                    $field::unpinned_render_update($field, peh, renderer, $field_var);
                                )+}
                            }
                        }
                    }
                }

                fn unpinned_render_update_force_reposition<PEH: ?Sized, Renderer: RenderHtml+?Sized>(
                    self,
                    peh: &mut PEH,
                    renderer: &mut Renderer,
                    render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
                ) {
                    match self {
                        ($($field,)+) => {
                            match render_state {
                                ($($field_var,)+) => {$(
                                    $field::unpinned_render_update_force_reposition($field, peh, renderer, $field_var);
                                )+}
                            }
                        }
                    }
                }

                fn unpinned_render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml+?Sized>(
                    self,
                    peh: &mut PEH,
                    renderer: &mut Renderer,
                    render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
                    force_reposition: bool,
                ) {
                    match self {
                        ($($field,)+) => {
                            match render_state {
                                ($($field_var,)+) => {$(
                                    $field::unpinned_render_update_maybe_reposition($field, peh, renderer, $field_var, force_reposition);
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
