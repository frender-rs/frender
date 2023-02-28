use frender_dom::props::IntrinsicComponentPollReactive;

pub struct NonReactiveData<T>(pub T);

impl<T> IntrinsicComponentPollReactive for NonReactiveData<T> {
    #[inline(always)]
    fn intrinsic_component_poll_reactive(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        std::task::Poll::Ready(false)
    }
}
