use crate::RenderState;

pub use SsrElement as Element;

pub trait SsrElement {
    type SsrState: RenderState;

    fn into_ssr_state(self) -> Self::SsrState;
}
