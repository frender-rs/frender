use frender_dom::csr::{
    render::{RenderContext, RenderWithContext},
    UiHandle, UnmountedUiHandle,
};

pub enum UiHandleMaybe<M, U> {
    BeforeMounted,
    Mounted(M),
    Unmounted(U),
}

impl<M, U> UiHandleMaybe<M, U> {
    /*
    fn mount_unmounted<Ctx: ?Sized + RenderContext>(&mut self, render_context: &mut Ctx) -> &mut M
    where
        U: UnmountedUiHandle<Ctx::Renderer, Mounted = M>,
    {
        let UiHandleMaybe::Unmounted(unmounted) = self.take() else {
            unreachable!()
        };

        *self = render_context.map_mut_render_context(|render_context| {
            Self::Mounted(unmounted.mount(render_context))
        });

        match self {
            UiHandleMaybe::Mounted(mounted) => mounted,
            _ => unreachable!(),
        }
    }
    */

    pub fn take(&mut self) -> Self {
        std::mem::replace(self, Self::BeforeMounted)
    }

    /// Returns `true` if mounted in this call.
    pub fn mount_in_place<Ctx: ?Sized>(&mut self, render_context: &mut Ctx) -> bool
    where
        Ctx: RenderContext,
        U: UnmountedUiHandle<Ctx::Renderer, Mounted = M>,
    {
        match self {
            UiHandleMaybe::Unmounted(_) => {
                let UiHandleMaybe::Unmounted(unmounted) = self.take() else {
                    unreachable!()
                };
                *self = Self::Mounted(
                    render_context
                        .map_mut_render_context(|render_context| unmounted.mount(render_context)),
                );
                true
            }
            _ => false,
        }
    }

    /// Returns `true` if unmounted in this call.
    /// Returns `false` if already unmounted before.
    pub fn unmount_in_place<R: ?Sized>(&mut self, renderer: &mut R) -> bool
    where
        M: UiHandle<R, Unmounted = U>,
    {
        match self {
            UiHandleMaybe::Mounted(_) => {
                let UiHandleMaybe::Mounted(mounted) = self.take() else {
                    unreachable!()
                };
                *self = Self::Unmounted(mounted.unmount(renderer));
                true
            }
            _ => false,
        }
    }
}

pub type UiHandleMaybeMounted<M, R> = UiHandleMaybe<M, <M as UiHandle<R>>::Unmounted>;

pub struct UnmountedUiHandleMaybe<U>(Option<U>);

impl<U, R: ?Sized> UnmountedUiHandle<R> for UnmountedUiHandleMaybe<U>
where
    U: UnmountedUiHandle<R>,
{
    type Mounted = UiHandleMaybe<U::Mounted, U>;

    fn mount(self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
    where
        R: RenderWithContext,
    {
        match self.0 {
            Some(unmounted) => UiHandleMaybe::Mounted(unmounted.mount(render_context)),
            None => UiHandleMaybe::BeforeMounted,
        }
    }
}

impl<M, U, R: ?Sized> UiHandle<R> for UiHandleMaybe<M, U>
where
    M: UiHandle<R, Unmounted = U>,
    U: UnmountedUiHandle<R, Mounted = M>,
{
    type Unmounted = UnmountedUiHandleMaybe<U>;

    fn unmount(self, renderer: &mut R) -> <Self as UiHandle<R>>::Unmounted {
        match self {
            UiHandleMaybe::BeforeMounted => UnmountedUiHandleMaybe(None),
            UiHandleMaybe::Mounted(mounted) => {
                UnmountedUiHandleMaybe(Some(mounted.unmount(renderer)))
            }
            UiHandleMaybe::Unmounted(unmounted) => UnmountedUiHandleMaybe(Some(unmounted)),
        }
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        if let UiHandleMaybe::Mounted(m) = self {
            m.reposition(render_context)
        }
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        if let UiHandleMaybe::Mounted(m) = self {
            m.check_and_move_cursor(render_context);
        }
    }

    fn assert_cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        if let UiHandleMaybe::Mounted(m) = self {
            m.assert_cursor_is_at_self(render_context);
        }
    }
}
