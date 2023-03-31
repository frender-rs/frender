use crate::RenderState;

pub trait Element {
    type SsrState: RenderState;

    fn into_ssr_state(self) -> Self::SsrState;
}
