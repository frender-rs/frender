// mod use_render;

use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_html::{Element, RenderHtml, RenderState};
use hooks_core::{HookPollNextUpdate, HookUnmount};

pin_project_lite::pin_project!(
    #[derive(Default)]
    pub struct State<HookData, S, U> {
        use_hook: U,
        render_iteration_count: u8,
        #[pin]
        hook_data: HookData,
        #[pin]
        render_state: S,
    }
);

impl<HookData: HookPollNextUpdate + HookUnmount + Default, U, E: Element, R: RenderHtml>
    RenderState<R> for State<HookData, E::RenderState<R>, Option<U>>
where
    U: FnMut(Pin<&mut HookData>) -> E,
{
    fn unmount(self: Pin<&mut Self>, renderer: &mut R) {
        let this = self.project();
        this.hook_data.unmount();
        this.render_state.unmount(renderer);
    }

    fn state_unmount(self: Pin<&mut Self>) {
        let this = self.project();
        this.hook_data.unmount();
        this.render_state.state_unmount();
    }

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let mut this = self.project();

        let use_hook = if let Some(use_hook) = this.use_hook {
            use_hook
        } else {
            return Poll::Ready(());
        };

        loop {
            let a = this.hook_data.as_mut().poll_next_update(cx);
            let b = this.render_state.as_mut().poll_render(renderer, cx);

            match (a, b) {
                (Poll::Ready(false), Poll::Ready(())) => return Poll::Ready(()),
                (Poll::Ready(true), _) => {
                    let element = use_hook(this.hook_data.as_mut());

                    element.render_update(renderer, this.render_state.as_mut());

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
{
    type RenderState<R: RenderHtml> = State<HookData, E::RenderState<R>, Option<U>>;

    fn render_update_maybe_reposition<Renderer: RenderHtml>(
        mut self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
        force_reposition: bool,
    ) {
        let render_state = render_state.project();
        (self.use_hook)(render_state.hook_data).render_update_maybe_reposition(
            renderer,
            render_state.render_state,
            force_reposition,
        );
        *render_state.use_hook = Some(self.use_hook);
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