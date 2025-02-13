use either::Either;
use frender_common::either::EitherElement;

use crate::{element::CsrElement, proxy_csr_element};

fn into_either_element<L, R>(this: Either<L, R>) -> EitherElement<L, R> {
    match this {
        Either::Left(this) => EitherElement::A(this),
        Either::Right(this) => EitherElement::B(this),
    }
}

impl<L, R> CsrElement for Either<L, R>
where
    L: CsrElement,
    R: CsrElement,
{
    type RenderStateKind = super::Kind<L::RenderStateKind, R::RenderStateKind>;
    type PinnedRenderInit<Renderer: ?Sized + crate::RenderHtml> = super::RenderInit<
        //
        L::PinnedRenderInit<Renderer>,
        R::PinnedRenderInit<Renderer>,
    >;
    proxy_csr_element!(|this| into_either_element(this));
}
