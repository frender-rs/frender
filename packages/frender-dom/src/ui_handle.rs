pub trait UnmountedUiHandle<Renderer: ?Sized> {
    type Mounted: UiHandle<Renderer, Unmounted = Self>;

    fn mount(self, render_context: &mut Renderer::RenderContext<'_>) -> Self::Mounted
    where
        Renderer: crate::render::RenderWithContext;
}

pub trait UiHandle<Renderer: ?Sized> {
    type Unmounted: UnmountedUiHandle<Renderer, Mounted = Self>;

    fn unmount(self, renderer: &mut Renderer) -> Self::Unmounted;

    fn reposition(&mut self, render_context: &mut Renderer::RenderContext<'_>)
    where
        Renderer: crate::render::RenderWithContext;

    fn check_and_move_cursor(&self, render_context: &mut Renderer::RenderContext<'_>)
    where
        Renderer: crate::render::RenderWithContext;

    /// The implementations and callers of this method could
    /// skip this method or just emits a warning.
    /// On debug builds, it's recommended to panic if the assertion fails so that
    /// developers could know the implementation might be wrong.
    fn assert_cursor_if_at_self(&self, render_context: &Renderer::RenderContext<'_>)
    where
        Renderer: crate::render::RenderWithContext;
}

impl<Renderer: ?Sized> UnmountedUiHandle<Renderer> for () {
    type Mounted = ();

    fn mount(self, _: &mut <Renderer>::RenderContext<'_>) -> Self::Mounted
    where
        Renderer: crate::render::RenderWithContext,
    {
    }
}

impl<Renderer: ?Sized> UiHandle<Renderer> for () {
    type Unmounted = ();

    fn unmount(self, _: &mut Renderer) -> Self::Unmounted {}

    fn reposition(&mut self, _: &mut <Renderer>::RenderContext<'_>)
    where
        Renderer: crate::render::RenderWithContext,
    {
    }

    fn check_and_move_cursor(&self, _: &mut <Renderer>::RenderContext<'_>)
    where
        Renderer: crate::render::RenderWithContext,
    {
    }

    fn assert_cursor_if_at_self(&self, _: &<Renderer>::RenderContext<'_>)
    where
        Renderer: crate::render::RenderWithContext,
    {
    }
}

// not impl for (UH,)

macro_rules! impl_for_tuple {
    ($(($($v:ident as $T:ident),*),)*) => {
        $(
            impl<Renderer: ?Sized, $($T: UnmountedUiHandle<Renderer>,)*> UnmountedUiHandle<Renderer> for ($($T,)*) {
                type Mounted = ($(<$T>::Mounted,)*);

                fn mount(self, render_context: &mut Renderer::RenderContext<'_>) -> Self::Mounted
                where
                    Renderer: crate::render::RenderWithContext,
                {
                    let ($($v,)*) = self;
                    ($($v.mount(render_context),)*)
                }
            }

            impl<Renderer: ?Sized, $($T: UiHandle<Renderer>,)*> UiHandle<Renderer> for ($($T,)*) {
                type Unmounted = ($($T::Unmounted,)*);

                fn unmount(self, renderer: &mut Renderer) -> Self::Unmounted {
                    let ($($v,)*) = self;
                    ($($v.unmount(renderer),)*)
                }

                fn reposition(&mut self, render_context: &mut Renderer::RenderContext<'_>)
                where
                    Renderer: crate::render::RenderWithContext,
                {
                    let ($($v,)*) = self;
                    $($v.reposition(render_context);)*
                }

                fn check_and_move_cursor(&self, render_context: &mut Renderer::RenderContext<'_>)
                where
                    Renderer: crate::render::RenderWithContext,
                {
                    let ($($v,)*) = self;
                    $($v.check_and_move_cursor(render_context);)*
                }

                fn assert_cursor_if_at_self(&self, render_context: &<Renderer>::RenderContext<'_>)
                where
                    Renderer: crate::render::RenderWithContext,
                {
                    // just assert cursor at the first ui handle
                    self.0.assert_cursor_if_at_self(render_context)
                }
            }
        )*
    };
}

impl_for_tuple! {
    (r0 as R0, r1 as R1),
    (r0 as R0, r1 as R1, r2 as R2),
    (r0 as R0, r1 as R1, r2 as R2, r3 as R3),
    (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4),
    (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5),
    (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6),
    (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6, r7 as R7),
    (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6, r7 as R7, r8 as R8),
    (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6, r7 as R7, r8 as R8, r9 as R9),
    (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6, r7 as R7, r8 as R8, r9 as R9, r10 as R10),
    (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6, r7 as R7, r8 as R8, r9 as R9, r10 as R10, r11 as R11),
    // (r0 as R0, r1 as R1, r2 as R2, r3 as R3, r4 as R4, r5 as R5, r6 as R6, r7 as R7, r8 as R8, r9 as R9, r10 as R10, r11 as R11, r12 as R12),
}

// region: array

impl<Renderer: ?Sized, UH: UnmountedUiHandle<Renderer>, const N: usize> UnmountedUiHandle<Renderer>
    for [UH; N]
{
    type Mounted = [UH::Mounted; N];

    fn mount(self, render_context: &mut <Renderer>::RenderContext<'_>) -> Self::Mounted
    where
        Renderer: crate::render::RenderWithContext,
    {
        self.map(|this| this.mount(render_context))
    }
}

impl<Renderer: ?Sized, UH: UiHandle<Renderer>, const N: usize> UiHandle<Renderer> for [UH; N] {
    type Unmounted = [UH::Unmounted; N];

    fn unmount(self, renderer: &mut Renderer) -> Self::Unmounted {
        self.map(|this| this.unmount(renderer))
    }

    fn reposition(&mut self, render_context: &mut <Renderer>::RenderContext<'_>)
    where
        Renderer: crate::render::RenderWithContext,
    {
        self.iter_mut()
            .for_each(|this| this.reposition(render_context))
    }

    fn check_and_move_cursor(&self, render_context: &mut <Renderer>::RenderContext<'_>)
    where
        Renderer: crate::render::RenderWithContext,
    {
        self.iter()
            .for_each(|this| this.check_and_move_cursor(render_context));
    }

    fn assert_cursor_if_at_self(&self, render_context: &<Renderer>::RenderContext<'_>)
    where
        Renderer: crate::render::RenderWithContext,
    {
        // just assert cursor at the first ui handle
        if let Some(first) = self.first() {
            first.assert_cursor_if_at_self(render_context);
        }
    }
}

// endregion
