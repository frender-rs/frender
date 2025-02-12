use std::marker::PhantomData;

use frender_common::convert::{FromMut as _, IntoMut as _};
use frender_form_control::value::FormControlValueStateKind;

use crate::{
    cs::textarea,
    element::{PinnedRenderStateKind, UnpinnedRenderStateKind},
    element_types::RenderStateKindPollRenderWithParent,
    form_control::value::FormControlValue,
    html::behavior_type_traits,
    kinds::RenderInitNothing,
    CsrComponent, RenderHtml,
};

enum Never {}
pub struct Kind<K, ET: ?Sized + behavior_type_traits::HtmlTextAreaElement>(Never, PhantomData<K>, PhantomData<ET>);

impl<K: FormControlValueStateKind<str>, ET: ?Sized + behavior_type_traits::HtmlTextAreaElement> UnpinnedRenderStateKind for Kind<K, ET> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = ();
    type UnpinnedState<R: RenderHtml + ?Sized> = K::UnpinnedState<ET::HtmlTextAreaElement<R>, R>;
}

impl<K: FormControlValueStateKind<str>, ET: ?Sized + behavior_type_traits::HtmlTextAreaElement> PinnedRenderStateKind for Kind<K, ET> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = ();
    type PinnedState<R: RenderHtml + ?Sized> = K::UnpinnedState<ET::HtmlTextAreaElement<R>, R>;
}

impl<K: FormControlValueStateKind<str>, ET: ?Sized + behavior_type_traits::HtmlTextAreaElement> RenderStateKindPollRenderWithParent<ET> for Kind<K, ET> {
    fn pinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut <ET as crate::BehaviorType>::OfBehaviorType<R>,
        state: std::pin::Pin<&mut crate::element::PinnedStateOfKind<R, Self>>,
        ui_handle: &mut crate::element::PinnedUiHandleOfKind<R, Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        <Self as RenderStateKindPollRenderWithParent<ET>>::unpinned_poll_render_with_parent(
            //
            renderer,
            parent,
            state.get_mut(),
            ui_handle,
            cx,
        )
    }

    fn unpinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut <ET as crate::BehaviorType>::OfBehaviorType<R>,
        state: &mut crate::element::UnpinnedStateOfKind<R, Self>,
        (): &mut crate::element::UnpinnedUiHandleOfKind<R, Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        K::unpinned_poll_render_form_control_value_state(
            //
            renderer,
            ET::HtmlTextAreaElement::from_mut(parent),
            state,
            cx,
        )
    }
}

impl<Children> CsrComponent<Children> for textarea::Marker
where
    Children: FormControlValue<str>,
{
    type ChildrenRenderStateKind = Kind<Children::StateKind, Self>;
    type ChildrenPinnedRenderInit<R: RenderHtml + ?Sized> = RenderInitNothing;

    fn children_pinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
    ) -> (
        //
        crate::element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        Self::ChildrenPinnedRenderInit<R>,
    ) {
        let (state, ()) = self.children_unpinned_render_init(children, renderer, parent);
        (state, RenderInitNothing)
    }

    fn children_pinned_render_init_by_reusing<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_reused_state: std::pin::Pin<&mut crate::element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
        children_unmounted_ui_handle: crate::element::PinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) -> crate::element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        self.children_unpinned_render_init_by_reusing(
            //
            children,
            renderer,
            parent,
            children_reused_state.get_mut(),
            children_unmounted_ui_handle,
        )
    }

    fn children_pinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_state: std::pin::Pin<&mut crate::element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
        children_ui_handle: &mut crate::element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) {
        self.children_unpinned_render_update(
            //
            children,
            renderer,
            parent,
            children_state.get_mut(),
            children_ui_handle,
        )
    }

    fn children_unpinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
    ) -> (
        //
        crate::element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        crate::element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) {
        (Children::render_init(children, renderer, parent.into_mut()), ())
    }

    fn children_unpinned_render_init_by_reusing<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_reused_state: &mut crate::element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        (): crate::element::UnpinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) -> crate::element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        // TODO: render_init or render_update?
        // render_init is correct but render_update might avoid unnecessary rendering
        *children_reused_state = Children::render_init(children, renderer, parent.into_mut());
        ()
    }

    fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_state: &mut crate::element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        (): &mut crate::element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) {
        Children::render_update(children, renderer, parent.into(), children_state)
    }
}
