use std::pin::Pin;

use crate::RenderState;

pin_project_lite::pin_project!(
    #[derive(Debug, Default)]
    pub struct CompoundState<S, T> {
        #[pin]
        pub reactive: S,
        pub non_reactive: T,
    }
);

impl<S, T> CompoundState<S, T> {
    pub fn pin_project(self: Pin<&mut Self>) -> CompoundState<Pin<&mut S>, &mut T> {
        let this = self.project();
        CompoundState {
            reactive: this.reactive,
            non_reactive: this.non_reactive,
        }
    }
}

impl<PEH: ?Sized, R: ?Sized, S: RenderState<PEH, R>, T> RenderState<PEH, R>
    for CompoundState<S, T>
{
    fn unmount(self: Pin<&mut Self>, peh: &mut PEH, renderer: &mut R) {
        self.project().reactive.unmount(peh, renderer)
    }

    fn state_unmount(self: Pin<&mut Self>) {
        self.project().reactive.state_unmount()
    }

    fn poll_render(
        self: Pin<&mut Self>,
        peh: &mut PEH,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        self.project().reactive.poll_render(peh, renderer, cx)
    }
}
