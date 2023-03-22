use frender_core::{RenderState, UpdateRenderState};
use futures_io::AsyncWrite;

use crate::{element::html::HtmlAttrPair, SsrContext};

pub trait IntoSsrData<W: AsyncWrite + Unpin> {
    type Children: UpdateRenderState<SsrContext<W>, State = Self::ChildrenRenderState>;
    type ChildrenRenderState: RenderState<SsrContext<W>>;
    type Attrs: Iterator<Item = HtmlAttrPair<'static>>;

    fn into_ssr_data(this: Self) -> (Self::Children, Self::Attrs);
}
