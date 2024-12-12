use either::Either;
use frender_common::either::EitherElement;

use crate::{element::CsrElement, HtmlRenderContext};

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

    fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        into_either_element(self).pinned_render_init(render_context, states)
    }

    fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        into_either_element(self).pinned_render_update(render_context, states)
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> crate::element::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        into_either_element(self).unpinned_render_init(render_context)
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        into_either_element(self).unpinned_render_update(render_context, states)
    }
}
