use frender_core::{RenderState, UpdateRenderState};
use futures_io::AsyncWrite;

use crate::{element::html::HtmlAttrPair, Ssr};

pub trait IntoSsrData<W: AsyncWrite + ?Sized> {
    type Children: UpdateRenderState<Ssr<W>, State = Self::ChildrenRenderState>;
    type ChildrenRenderState: RenderState<Ssr<W>>;
    type Attrs: Iterator<Item = HtmlAttrPair<'static>>;

    fn into_ssr_data(this: Self) -> (Self::Children, Self::Attrs);
}
