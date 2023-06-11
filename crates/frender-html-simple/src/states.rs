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

#[cfg(feature = "csr")]
mod dom {
    use super::*;
    use frender_csr::RenderState;

    impl<ChildrenState, PropsState> RenderState for ElementPropsStates<ChildrenState, PropsState>
    where
        ChildrenState: RenderState,
    {
        fn unmount(self: Pin<&mut Self>) {
            self.project().children.unmount()
        }

        fn state_unmount(self: Pin<&mut Self>) {
            self.project().children.state_unmount()
        }

        #[inline(always)]
        fn poll_csr(
            self: std::pin::Pin<&mut Self>,
            ctx: &mut frender_csr::CsrContext,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            self.project().children.poll_csr(ctx, cx)
        }
    }
}
