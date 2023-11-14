use std::pin::Pin;

pin_project_lite::pin_project! {
    pub struct ElementPropsStates<ChildrenState, PropsState> {
        #[pin]
        pub children: ChildrenState,
        #[pin]
        pub other_state: PropsState,
    }
}

impl<ChildrenState, PropsState> ElementPropsStates<ChildrenState, PropsState> {
    pub fn pin_project(self: Pin<&mut Self>) -> (Pin<&mut ChildrenState>, Pin<&mut PropsState>) {
        let this = self.project();
        (this.children, this.other_state)
    }
}

impl<R, ChildrenState, PropsState> crate::RenderState<R>
    for ElementPropsStates<ChildrenState, PropsState>
where
    ChildrenState: crate::RenderState<R>,
{
    fn unmount(self: Pin<&mut Self>, renderer: &mut R) {
        self.project().children.unmount(renderer)
    }

    fn state_unmount(self: Pin<&mut Self>) {
        self.project().children.state_unmount()
    }

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        self.project().children.poll_render(renderer, cx)
    }
}
