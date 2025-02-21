use crate::{csr::CsrElement, html::RenderHtml};

impl<E: CsrElement> CsrElement for Box<E> {
    type RenderStateKind = E::RenderStateKind;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = E::PinnedRenderInit<R>;

    crate::proxy_csr_element!(|this| *this);
}
