pub use self::cursor_placeholder::{
    CursorPlaceholderWithRenderState, CursorPlaceholderWithRenderStatePinProject,
};

use std::pin::Pin;

#[derive(Debug, Default)]
pub enum MountState {
    #[default]
    Unmounted,
    StateUnmounted,
    Mounted,
}

pin_project_lite::pin_project!(
    #[derive(Debug, Default)]
    pub struct State<HookData, S, T> {
        mount_state: MountState,
        #[pin]
        hook_data: HookData,
        #[pin]
        render_state: S,
        #[pin]
        inner: T,
    }
);

pub struct StatePinProject<'a, HookData, S, T> {
    pub mount_state: &'a mut MountState,
    pub hook_data: Pin<&'a mut HookData>,
    pub render_state: Pin<&'a mut S>,
    pub inner: Pin<&'a mut T>,
}

pub struct StateMutProject<'a, HookData, S, T> {
    pub mount_state: &'a mut MountState,
    pub hook_data: &'a mut HookData,
    pub render_state: &'a mut S,
    pub inner: &'a mut T,
}

impl<HookData, S, T> State<HookData, S, T> {
    pub fn pin_project(self: Pin<&mut Self>) -> StatePinProject<HookData, S, T> {
        let this = self.project();
        StatePinProject {
            mount_state: this.mount_state,
            hook_data: this.hook_data,
            render_state: this.render_state,
            inner: this.inner,
        }
    }

    pub fn as_mut_project(&mut self) -> StateMutProject<HookData, S, T> {
        StateMutProject {
            mount_state: &mut self.mount_state,
            hook_data: &mut self.hook_data,
            render_state: &mut self.render_state,
            inner: &mut self.inner,
        }
    }
}

pub trait MaybeIntoPollNextUpdate<R: ?Sized, HookData, S> {
    type IntoPollNextUpdate<'a>: hooks_core::HookPollNextUpdate + Unpin
    where
        Self: 'a,
        R: 'a,
        HookData: 'a,
        S: 'a;
    fn maybe_into_poll_next_update<'a>(
        self: Pin<&'a mut Self>,
        renderer: &'a mut R,
        hook_data: Pin<&'a mut HookData>,
        render_state: Pin<&'a mut S>,
    ) -> Option<Self::IntoPollNextUpdate<'a>>;
}

mod imp {
    use std::task::Poll;

    use frender_html::{RenderHtml, RenderState};
    use hooks_core::{HookPollNextUpdate as _, HookUnmount};

    use super::{MaybeIntoPollNextUpdate, MountState, State};

    impl<
            HookData: HookUnmount + Default,
            S: RenderState<R>,
            U: MaybeIntoPollNextUpdate<R, HookData, S>,
            R: RenderHtml + ?Sized,
        > RenderState<R> for State<HookData, S, U>
    {
        fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
            if let MountState::Unmounted = self.mount_state {
                return;
            }

            let this = self.project();

            if let MountState::Mounted = this.mount_state {
                this.hook_data.unmount();
            } // else hook_data has been unmounted when state_unmount was called

            this.render_state.unmount(renderer);

            *this.mount_state = MountState::Unmounted;
        }

        fn state_unmount(self: std::pin::Pin<&mut Self>) {
            if !matches!(self.mount_state, MountState::Mounted) {
                return;
            }

            let this = self.project();
            this.hook_data.unmount();
            this.render_state.state_unmount();
            *this.mount_state = MountState::StateUnmounted;
        }

        fn poll_render(
            self: std::pin::Pin<&mut Self>,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            if !matches!(self.mount_state, MountState::Mounted) {
                return Poll::Ready(());
            }

            let this = self.project();

            let mut render_iteration_count = 0;

            let mut inner = if let Some(inner) =
                this.inner
                    .maybe_into_poll_next_update(renderer, this.hook_data, this.render_state)
            {
                inner
            } else {
                return Poll::Ready(());
            };
            loop {
                match std::pin::Pin::new(&mut inner).poll_next_update(cx) {
                    Poll::Ready(true) => {
                        if render_iteration_count == u8::MAX {
                            cx.waker().wake_by_ref();
                            // TODO: warn
                            return Poll::Pending;
                        } else {
                            render_iteration_count += 1;
                        }
                    }
                    Poll::Ready(false) => return Poll::Ready(()),
                    Poll::Pending => return Poll::Pending,
                }
            }
        }
    }
}

mod cursor_placeholder {
    use std::pin::Pin;

    use frender_html::{dom::behaviors::Node, RenderState};

    pin_project_lite::pin_project!(
        pub struct CursorPlaceholderWithRenderState<C, T, S> {
            pub cursor_placeholder_and_data: Option<(C, T)>,
            #[pin]
            pub render_state: S,
        }
    );

    pub struct CursorPlaceholderWithRenderStatePinProject<'a, C, T, S> {
        pub cursor_placeholder_and_data: &'a mut Option<(C, T)>,
        pub render_state: Pin<&'a mut S>,
    }

    impl<C, T, S> CursorPlaceholderWithRenderState<C, T, S> {
        pub fn pin_project(
            self: Pin<&mut Self>,
        ) -> CursorPlaceholderWithRenderStatePinProject<C, T, S> {
            let this = self.project();
            CursorPlaceholderWithRenderStatePinProject {
                cursor_placeholder_and_data: this.cursor_placeholder_and_data,
                render_state: this.render_state,
            }
        }
    }

    impl<C, T, S: Default> Default for CursorPlaceholderWithRenderState<C, T, S> {
        fn default() -> Self {
            Self {
                cursor_placeholder_and_data: None,
                render_state: Default::default(),
            }
        }
    }

    impl<C, T, S, R: ?Sized> RenderState<R> for CursorPlaceholderWithRenderState<C, T, S>
    where
        C: Node<R>,
        S: RenderState<R>,
    {
        fn unmount(self: Pin<&mut Self>, renderer: &mut R) {
            let this = self.project();
            if let Some((ref mut cp, _)) = this.cursor_placeholder_and_data {
                cp.remove_self(renderer)
            }
            this.render_state.unmount(renderer)
        }

        fn state_unmount(self: Pin<&mut Self>) {
            self.project().render_state.state_unmount()
        }

        fn poll_render(
            self: Pin<&mut Self>,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            self.project().render_state.poll_render(renderer, cx)
        }
    }
}
