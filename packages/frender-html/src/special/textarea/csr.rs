use std::marker::PhantomData;

use frender_common::convert::{FromMut as _, IntoMut as _};
use frender_form_control::{
    csr::{FormControlValue, FormControlValueStateKind},
    textarea::CsrTextAreaValue,
    KindOfValue,
};

use crate::{
    cs::textarea,
    csr::{
        behavior_type::BehaviorType,
        component::{CsrComponent, RenderStateKindPollRenderWithParent},
        element::{self, PinnedRenderStateKind, UnpinnedRenderStateKind},
    },
    html::{behavior_type_traits, RenderHtml},
    kinds::RenderInitNothing,
};

enum Never {}
pub struct Kind<K, ET: ?Sized + behavior_type_traits::HtmlTextAreaElement>(Never, PhantomData<K>, PhantomData<ET>);

impl<K: FormControlValueStateKind<KindOfValue>, ET: ?Sized + behavior_type_traits::HtmlTextAreaElement> UnpinnedRenderStateKind for Kind<K, ET> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = ();
    type UnpinnedState<R: RenderHtml + ?Sized> = K::UnpinnedState<ET::HtmlTextAreaElement<R>, R>;
}

impl<K: FormControlValueStateKind<KindOfValue>, ET: ?Sized + behavior_type_traits::HtmlTextAreaElement> PinnedRenderStateKind for Kind<K, ET> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = ();
    type PinnedState<R: RenderHtml + ?Sized> = K::UnpinnedState<ET::HtmlTextAreaElement<R>, R>;
}

impl<K: FormControlValueStateKind<KindOfValue>, ET: ?Sized + behavior_type_traits::HtmlTextAreaElement> RenderStateKindPollRenderWithParent<ET> for Kind<K, ET> {
    fn pinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut <ET as BehaviorType>::OfBehaviorType<R>,
        state: std::pin::Pin<&mut element::PinnedStateOfKind<R, Self>>,
        ui_handle: &mut element::PinnedUiHandleOfKind<R, Self>,
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
        parent: &mut <ET as BehaviorType>::OfBehaviorType<R>,
        state: &mut element::UnpinnedStateOfKind<R, Self>,
        (): &mut element::UnpinnedUiHandleOfKind<R, Self>,
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
    Children: CsrTextAreaValue,
{
    type ChildrenRenderStateKind = Kind<
        //
        <Children::IntoCsrTextAreaValue as FormControlValue<KindOfValue>>::StateKind,
        Self,
    >;
    type ChildrenPinnedRenderInit<R: RenderHtml + ?Sized> = RenderInitNothing;

    fn children_pinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
    ) -> (
        //
        element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
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
        children_reused_state: std::pin::Pin<&mut element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
        children_unmounted_ui_handle: element::PinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) -> element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
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
        children_state: std::pin::Pin<&mut element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
        children_ui_handle: &mut element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
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
        element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) {
        (Children::IntoCsrTextAreaValue::render_init(Children::into_csr_text_area_value(children), renderer, parent.into_mut()), ())
    }

    fn children_unpinned_render_init_by_reusing<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_reused_state: &mut element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        (): element::UnpinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) -> element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        // TODO: render_init or render_update?
        // render_init is correct but render_update might avoid unnecessary rendering
        *children_reused_state = Children::IntoCsrTextAreaValue::render_init(Children::into_csr_text_area_value(children), renderer, parent.into_mut());
        ()
    }

    fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_state: &mut element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        (): &mut element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) {
        Children::IntoCsrTextAreaValue::render_update(Children::into_csr_text_area_value(children), renderer, parent.into(), children_state)
    }
}
