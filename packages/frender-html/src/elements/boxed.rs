use crate::CsrElement;

impl<E: CsrElement> CsrElement for Box<E> {
    type RenderStateKind = E::RenderStateKind;
    type RenderInitKind = E::RenderInitKind;

    crate::proxy_csr_element!(|this| *this);
}
