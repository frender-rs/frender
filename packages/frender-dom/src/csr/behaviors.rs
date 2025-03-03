use frender_style::css_style_declaration::CssStyleDeclaration;

use crate::csr::{render::RenderWithContext, render_from::str::ValueForStr};

#[cfg(feature = "web")]
mod inner_html_web;
#[cfg(feature = "web")]
mod inner_text_web;

pub trait SetInnerHtmlFromStr<Renderer: ?Sized> {
    fn set_inner_html_from_str(&mut self, renderer: &mut Renderer, value: impl ValueForStr);
}

pub trait SetInnerTextFromStr<Renderer: ?Sized> {
    fn set_inner_text_from_str(&mut self, renderer: &mut Renderer, value: impl ValueForStr);
}

// TODO: replace RenderHtml::$tag() with NodeRenderSelf
pub trait NodeRenderSelf<Renderer: ?Sized + RenderWithContext> {
    /// Should create the node,
    /// add the node to dom at the cursor,
    /// and move the cursor after the node.
    fn render_self(render_context: &mut Renderer::RenderContext<'_>) -> Self;
}

pub trait NodeWithRenderContextAfterSelf<Renderer: ?Sized + RenderWithContext> {
    fn with_render_context_after_self<Res>(
        &mut self,
        renderer: &mut Renderer,
        f: impl FnOnce(&mut Renderer::RenderContext<'_>) -> Res,
    ) -> Res;
}

// TODO: use UiHandle
pub trait Node<Renderer: ?Sized> {
    fn log_self(&self, renderer: &mut Renderer);

    fn warn_self_with_message(&self, renderer: &mut Renderer, message: &str);

    /// Should move the node if `force_reposition`,
    /// and move cursor after the node.
    fn readd_self(
        &mut self,
        render_context: &mut Renderer::RenderContext<'_>,
        force_reposition: bool,
    ) where
        Renderer: RenderWithContext;

    fn cursor_is_at_self(&self, render_context: &Renderer::RenderContext<'_>) -> bool
    where
        Renderer: RenderWithContext;

    fn check_and_move_cursor_after_self(&self, render_context: &mut Renderer::RenderContext<'_>)
    where
        Renderer: RenderWithContext;

    fn remove_self(&mut self, renderer: &mut Renderer);
}

pub trait Element<Renderer: ?Sized>: Node<Renderer> + SetInnerHtmlFromStr<Renderer> {
    fn set_attribute(&mut self, renderer: &mut Renderer, name: &str, value: &str);
    fn remove_attribute(&mut self, renderer: &mut Renderer, name: &str);

    /// This kind of method of behavior traits have the same name `as_node_ref`
    /// so that callers can call it with `$TraitName::as_node_ref` in macros.
    fn as_node_ref(&self) -> &(dyn 'static + crate::node_ref::traits::Element);
}

pub trait HtmlElement<Renderer: ?Sized>: Element<Renderer> + SetInnerTextFromStr<Renderer> {
    fn as_node_ref(&self) -> &(dyn 'static + crate::node_ref::traits::HtmlElement);
}

pub trait ElementWithClassList<Renderer: ?Sized>: Element<Renderer> {
    type ClassList<'a>: frender_dom_tokens::DomTokenList
    where
        Self: 'a,
        Renderer: 'a;
    fn class_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::ClassList<'a>;
}

pub trait ElementWithRelList<Renderer: ?Sized>: Element<Renderer> {
    type RelList<'a>: frender_dom_tokens::DomTokenList
    where
        Self: 'a,
        Renderer: 'a;
    fn rel_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::RelList<'a>;
}

pub trait ElementWithStyle<Renderer: ?Sized>: Element<Renderer> {
    type Style<'a>: CssStyleDeclaration
    where
        Self: 'a,
        Renderer: 'a;

    fn style<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::Style<'a>;
}

pub trait ElementWithChildren<Renderer: ?Sized> {
    fn with_render_context_at_first_child_of_self<R>(
        &mut self,
        renderer: &mut Renderer,
        f: impl FnOnce(&mut Renderer::RenderContext<'_>) -> R,
    ) -> R
    where
        Renderer: RenderWithContext;
}

#[cfg(feature = "web")]
mod web;
