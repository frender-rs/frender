use frender_dom::props::IntrinsicComponentPollReactive;

// TODO: remove
pub struct NonReactiveData<T>(pub T);

impl<T> IntrinsicComponentPollReactive for NonReactiveData<T> {
    #[inline(always)]
    fn intrinsic_component_poll_reactive(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        std::task::Poll::Ready(())
    }
}
