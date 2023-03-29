use std::pin::Pin;

use frender_core::RenderState;

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

#[cfg(feature = "dom")]
mod dom {
    use super::*;
    use frender_csr::props::IntrinsicComponentPollReactive;

    impl<ChildrenState, PropsState> IntrinsicComponentPollReactive
        for ElementPropsStates<ChildrenState, PropsState>
    where
        ChildrenState: RenderState<frender_csr::Dom>,
    {
        fn intrinsic_component_unmount(self: Pin<&mut Self>) {
            self.project().children.unmount()
        }

        #[inline(always)]
        fn intrinsic_component_poll_reactive(
            self: std::pin::Pin<&mut Self>,
            ctx: &mut frender_csr::Dom,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            self.project().children.poll_reactive(ctx, cx)
        }
    }
}
