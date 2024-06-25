use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_html::{Element, RenderHtml, RenderState};
use hooks_core::{HookPollNextUpdate, HookUnmount};

pin_project_lite::pin_project!(
    pub struct State<HookData, S, U, C> {
        use_hook_and_cursor_placeholder: StatedUseHookAndCursorPlaceholder<U, C>,
        render_iteration_count: u8,
        #[pin]
        hook_data: HookData,
        #[pin]
        render_state: S,
    }
);

impl<HookData: Default, S: Default, U, C> Default for State<HookData, S, U, C> {
    fn default() -> Self {
        Self {
            use_hook_and_cursor_placeholder: StatedUseHookAndCursorPlaceholder {
                use_hook_and_cursor_placeholder: None,
                mount_state: MountState::Unmounted,
            },
            render_iteration_count: 0,
            hook_data: Default::default(),
            render_state: Default::default(),
        }
    }
}

enum MountState {
    Unmounted,
    StateUnmounted,
    Mounted,
}

struct StatedUseHookAndCursorPlaceholder<U, C> {
    use_hook_and_cursor_placeholder: Option<(U, C)>,
    mount_state: MountState,
}

pub trait UseHookRenderUpdate<HookData> {
    type State<R: RenderHtml + ?Sized>: RenderState<R>;
    fn use_hook_render_update<R: RenderHtml + ?Sized>(
        &mut self,
        hook_data: Pin<&mut HookData>,
        renderer: &mut R,
        render_state: Pin<&mut Self::State<R>>,
    );
}

pub struct UseHookWithRenderState<U>(pub U);

impl<HookData, U: FnMut(Pin<&mut HookData>) -> E, E: Element> UseHookRenderUpdate<HookData>
    for UseHookWithRenderState<U>
{
    type State<R: RenderHtml + ?Sized> = E::RenderState<R>;

    fn use_hook_render_update<R: RenderHtml + ?Sized>(
        &mut self,
        hook_data: Pin<&mut HookData>,
        renderer: &mut R,
        render_state: Pin<&mut Self::State<R>>,
    ) {
        self.0(hook_data).render_update(renderer, render_state)
    }
}

pub struct UseHookWithUnpinnedRenderState<U>(pub U);

impl<HookData, U: FnMut(Pin<&mut HookData>) -> E, E: Element> UseHookRenderUpdate<HookData>
    for UseHookWithUnpinnedRenderState<U>
{
    type State<R: RenderHtml + ?Sized> = E::UnpinnedRenderState<R>;

    fn use_hook_render_update<R: RenderHtml + ?Sized>(
        &mut self,
        hook_data: Pin<&mut HookData>,
        renderer: &mut R,
        render_state: Pin<&mut Self::State<R>>,
    ) {
        self.0(hook_data).unpinned_render_update(renderer, render_state.get_mut())
    }
}

impl<
        HookData: HookPollNextUpdate + HookUnmount + Default,
        U: UseHookRenderUpdate<HookData>,
        R: RenderHtml + ?Sized,
    > RenderState<R> for State<HookData, U::State<R>, U, R::CursorPlaceholder>
{
    fn unmount(self: Pin<&mut Self>, renderer: &mut R) {
        if let MountState::Unmounted = self.use_hook_and_cursor_placeholder.mount_state {
            return;
        }

        let this = self.project();

        if let MountState::Mounted = this.use_hook_and_cursor_placeholder.mount_state {
            this.hook_data.unmount();
        }

        this.render_state.unmount(renderer);

        this.use_hook_and_cursor_placeholder.mount_state = MountState::Unmounted;
    }

    fn state_unmount(self: Pin<&mut Self>) {
        if !matches!(
            self.use_hook_and_cursor_placeholder.mount_state,
            MountState::Mounted
        ) {
            return;
        }

        let this = self.project();
        this.hook_data.unmount();
        this.render_state.state_unmount();
        this.use_hook_and_cursor_placeholder.mount_state = MountState::StateUnmounted;
    }

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let mut this = self.project();

        let (use_hook, placeholder) = match this.use_hook_and_cursor_placeholder {
            StatedUseHookAndCursorPlaceholder {
                use_hook_and_cursor_placeholder: Some(u),
                mount_state: MountState::Mounted,
            } => u,
            _ => return Poll::Ready(()),
        };

        loop {
            let a = this.hook_data.as_mut().poll_next_update(cx);

            #[cfg(debug_assertions)]
            let initial_cursor = renderer.cursor();

            let b = this.render_state.as_mut().poll_render(renderer, cx);

            #[cfg(debug_assertions)]
            assert!(
                renderer.cursor_is_same_as(&initial_cursor),
                "cursor changed in hook element RenderState::poll_render"
            );

            match (a, b) {
                (Poll::Ready(false), Poll::Ready(())) => return Poll::Ready(()),
                (Poll::Ready(true), _) => {
                    renderer.with_render_context(|renderer| {
                        renderer.move_cursor_after_placeholder(placeholder);
                        use_hook.use_hook_render_update(
                            this.hook_data.as_mut(),
                            renderer,
                            this.render_state.as_mut(),
                        );
                    });

                    if *this.render_iteration_count == u8::MAX {
                        *this.render_iteration_count = 0;
                        cx.waker().wake_by_ref();
                        return Poll::Pending;
                    } else {
                        *this.render_iteration_count += 1;
                    }
                }
                _ => return Poll::Pending,
            }
        }
    }
}

pub struct FnHookElement<HookData: HookPollNextUpdate + HookUnmount + Default, U> {
    use_hook: U,
    _phantom: PhantomData<HookData>,
}

impl<HookData: HookPollNextUpdate + HookUnmount + Default, U, E: Element> Element
    for FnHookElement<HookData, U>
where
    U: FnMut(Pin<&mut HookData>) -> E,
    HookData: Unpin,
{
    type RenderState<R: RenderHtml + ?Sized> =
        State<HookData, E::RenderState<R>, UseHookWithRenderState<U>, R::CursorPlaceholder>;

    fn render_update_maybe_reposition<Renderer: RenderHtml + ?Sized>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
        mut force_reposition: bool,
    ) {
        let render_state = render_state.project();
        let StatedUseHookAndCursorPlaceholder {
            use_hook_and_cursor_placeholder,
            mount_state,
        } = render_state.use_hook_and_cursor_placeholder;

        let use_hook = if let Some((use_hook, cp)) = use_hook_and_cursor_placeholder {
            force_reposition = force_reposition || matches!(mount_state, MountState::Unmounted);
            if force_reposition {
                renderer.cursor_placeholder_force_reposition(cp);
            }
            use_hook.0 = self.use_hook;

            use_hook
        } else {
            force_reposition = true;
            let cp = renderer.cursor_placeholder_render();
            let (use_hook, _) =
                use_hook_and_cursor_placeholder.insert((UseHookWithRenderState(self.use_hook), cp));
            use_hook
        };

        (use_hook.0)(render_state.hook_data).render_update_maybe_reposition(
            renderer,
            render_state.render_state,
            force_reposition,
        );

        *mount_state = MountState::Mounted;
    }

    type UnpinnedRenderState<R: RenderHtml + ?Sized> = State<
        HookData,
        E::UnpinnedRenderState<R>,
        UseHookWithUnpinnedRenderState<U>,
        R::CursorPlaceholder,
    >;

    fn unpinned_render_update_maybe_reposition<Renderer: RenderHtml + ?Sized>(
        self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
        mut force_reposition: bool,
    ) {
        let StatedUseHookAndCursorPlaceholder {
            use_hook_and_cursor_placeholder,
            mount_state,
        } = &mut render_state.use_hook_and_cursor_placeholder;

        let use_hook = if let Some((use_hook, cp)) = use_hook_and_cursor_placeholder {
            force_reposition = force_reposition || matches!(mount_state, MountState::Unmounted);
            if force_reposition {
                renderer.cursor_placeholder_force_reposition(cp);
            }
            use_hook.0 = self.use_hook;

            use_hook
        } else {
            force_reposition = true;
            let cp = renderer.cursor_placeholder_render();
            let (use_hook, _) = use_hook_and_cursor_placeholder
                .insert((UseHookWithUnpinnedRenderState(self.use_hook), cp));
            use_hook
        };

        (use_hook.0)(Pin::new(&mut render_state.hook_data))
            .unpinned_render_update_maybe_reposition(
                renderer,
                &mut render_state.render_state,
                force_reposition,
            );

        *mount_state = MountState::Mounted;
    }
}

impl<HookData: HookPollNextUpdate + HookUnmount + Default, U, E: Element> frender_ssr::SsrElement
    for FnHookElement<HookData, U>
where
    U: FnMut(Pin<&mut HookData>) -> E,
{
    type HtmlChildren = E::HtmlChildren;

    fn into_html_children(mut self) -> Self::HtmlChildren {
        let hook_data = HookData::default();
        let hook_data = std::pin::pin!(hook_data); // TODO: compatibility
        (self.use_hook)(hook_data).into_html_children()
    }
}

pub fn new_fn_hook_element<HookData: HookPollNextUpdate + HookUnmount + Default, U, E: Element>(
    use_hook: U,
) -> FnHookElement<HookData, U>
where
    U: FnMut(Pin<&mut HookData>) -> E,
{
    FnHookElement {
        use_hook,
        _phantom: PhantomData,
    }
}
