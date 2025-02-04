use crate::CsrElement;

impl<E: CsrElement> CsrElement for Box<E> {
    type RenderStateKind = E::RenderStateKind;

    crate::proxy_csr_element!(|this| *this);
}
