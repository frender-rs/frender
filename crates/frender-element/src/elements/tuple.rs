#![allow(non_snake_case)]

use crate::{pin_mut_maybe_uninit::PinMutMaybeUninit, Element, RenderHtml, RenderState};

impl<R> RenderState<R> for () {
    #[inline]
    fn unmount(self: std::pin::Pin<&mut Self>, _: &mut R) {}

    #[inline]
    fn state_unmount(self: std::pin::Pin<&mut Self>) {}

    #[inline]
    fn poll_render(
        self: std::pin::Pin<&mut Self>,
        _: &mut R,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        std::task::Poll::Ready(())
    }
}

impl Element for () {
    type RenderState<R: RenderHtml> = ();

    #[cfg(feature = "render_into")]
    fn render_into<'s, Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: PinMutMaybeUninit<'s, Self::RenderState<Renderer>>,
    ) -> std::pin::Pin<&'s mut Self::RenderState<Renderer>> {
        render_state.write(())
    }

    fn render_update_maybe_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
        force_reposition: bool,
    ) {
    }

    fn render_update<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
    }

    fn render_update_force_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
    }
}

impl<E0: Element> Element for (E0,) {
    type RenderState<R: RenderHtml> = E0::RenderState<R>;

    #[cfg(feature = "render_into")]
    fn render_into<'s, Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: PinMutMaybeUninit<'s, Self::RenderState<Renderer>>,
    ) -> std::pin::Pin<&'s mut Self::RenderState<Renderer>> {
        self.0.render_into(renderer, render_state)
    }

    fn render_update_maybe_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
        force_reposition: bool,
    ) {
        self.0
            .render_update_maybe_reposition(renderer, render_state, force_reposition)
    }

    fn render_update<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
        self.0.render_update(renderer, render_state)
    }

    fn render_update_force_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
        self.render_update_force_reposition(renderer, render_state)
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

            impl<$($field: Element),+> Element for ($($field,)+) {
                type RenderState<R: RenderHtml> = ($($field::RenderState<R>,)+);

                #[cfg(feature = "render_into")]
                fn render_into<'s, Renderer: RenderHtml>(
                    self,
                    renderer: &mut Renderer,
                    render_state: PinMutMaybeUninit<'s, Self::RenderState<Renderer>>,
                ) -> std::pin::Pin<&'s mut Self::RenderState<Renderer>> {
                    let ($($field,)+) = self;

                    crate::pin_mut_maybe_uninit::PinMutInitializeWith::pin_mut_initialize_with(
                        render_state,
                        ($(
                            |render_state| $field::render_into($field, renderer, render_state)
                        ,)+)
                    )
                }

                fn render_update_maybe_reposition<Renderer: RenderHtml>(
                    self,
                    renderer: &mut Renderer,
                    render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
                    force_reposition: bool,
                ) {
                    let ($($field,)+) = self;
                    let ($($field_var,)+) = frender_common::utils::pin_project::$name(render_state);
                    $($field::render_update_maybe_reposition($field, renderer, $field_var, force_reposition);)+
                }

                fn render_update<Renderer: RenderHtml>(
                    self,
                    renderer: &mut Renderer,
                    render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
                ) {
                    let ($($field,)+) = self;
                    let ($($field_var,)+) = frender_common::utils::pin_project::$name(render_state);
                    $($field::render_update($field, renderer, $field_var);)+
                }

                fn render_update_force_reposition<Renderer: RenderHtml>(
                    self,
                    renderer: &mut Renderer,
                    render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>,
                ) {
                    let ($($field,)+) = self;
                    let ($($field_var,)+) = frender_common::utils::pin_project::$name(render_state);
                    $($field::render_update_force_reposition($field, renderer, $field_var);)+
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
