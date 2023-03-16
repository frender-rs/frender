#[derive(Clone, Copy)]
pub struct NoData;

impl hooks_core::HookPollNextUpdate for NoData {
    #[inline(always)]
    fn poll_next_update(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        std::task::Poll::Ready(false)
    }
}
