// mod use_render;

use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_element::{Element, RenderState};
use frender_html::RenderHtml;
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

#[derive(Default)]
pub struct UnpinnedState<HookData, S, U> {
    use_hook: U,
    render_iteration_count: u8,
    hook_data: HookData,
    render_state: S,
}

impl<HookData, S, U> Unpin for UnpinnedState<HookData, S, U> {}

impl<HookData: HookPollNextUpdate + HookUnmount + Default, U, E: Element, R: RenderHtml>
    RenderState<R> for UnpinnedState<HookData, E::UnpinnedRenderState<R>, Option<U>>
where
    U: FnMut(Pin<&mut HookData>) -> E,
    HookData: Unpin,
{
    fn unmount(self: Pin<&mut Self>, renderer: &mut R) {
        let this = self.get_mut();
        Pin::new(&mut this.hook_data).unmount();
        RenderState::<_>::unmount(Pin::new(&mut this.render_state), renderer);
    }

    fn state_unmount(self: Pin<&mut Self>) {
        let this = self.get_mut();
        Pin::new(&mut this.hook_data).unmount();
        Pin::new(&mut this.render_state).state_unmount();
    }

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let this = self.get_mut();

        let use_hook = if let Some(use_hook) = &mut this.use_hook {
            use_hook
        } else {
            return Poll::Ready(());
        };

        loop {
            let a = Pin::new(&mut this.hook_data).poll_next_update(cx);
            let b = Pin::new(&mut this.render_state).poll_render(renderer, cx);

            match (a, b) {
                (Poll::Ready(false), Poll::Ready(())) => return Poll::Ready(()),
                (Poll::Ready(true), _) => {
                    let element = use_hook(Pin::new(&mut this.hook_data));

                    element.unpinned_render_update(renderer, &mut this.render_state);

                    if this.render_iteration_count == u8::MAX {
                        this.render_iteration_count = 0;
                        cx.waker().wake_by_ref();
                        return Poll::Pending;
                    } else {
                        this.render_iteration_count += 1;
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

    type UnpinnedRenderState<R: RenderHtml> =
        UnpinnedState<HookData, E::UnpinnedRenderState<R>, Option<U>>;

    fn unpinned_render_update_maybe_reposition<Renderer: RenderHtml>(
        mut self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
        force_reposition: bool,
    ) {
        (self.use_hook)(Pin::new(&mut render_state.hook_data))
            .unpinned_render_update_maybe_reposition(
                renderer,
                &mut render_state.render_state,
                force_reposition,
            );
        render_state.use_hook = Some(self.use_hook);
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
