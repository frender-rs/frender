use crate::CsrElement;

impl<E: CsrElement> CsrElement for Box<E> {
    type RenderStateKind = E::RenderStateKind;
    type PinnedRenderInit<R: ?Sized + crate::RenderHtml> = E::PinnedRenderInit<R>;

    crate::proxy_csr_element!(|this| *this);
}
