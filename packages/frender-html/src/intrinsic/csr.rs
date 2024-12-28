use frender_common::convert::IntoMut;
use frender_dom::ui_handle::{UiHandle, UnmountedUiHandle};
use pin_project_lite::pin_project;

use crate::dom::component::HasIntrinsicComponentTag;

use crate::element::{PinMutRenderInitStates, PinnedRenderStateKind, PinnedRenderStateKindPollRender, RenderStates, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender};
use crate::element_types::RenderStateKindPollRenderWithParent;
use crate::html::{behavior_type_traits, behaviors};
use crate::intrinsic::Intrinsic;
use crate::update_element::{PinnedNonReactiveRenderStateKind, PinnedRenderWithBehavior, UnpinnedNonReactiveRenderStateKind, UnpinnedRenderWithBehavior};
use crate::{CsrComponent, HtmlRenderContext, RenderHtml};

use crate::CsrElement;

pub struct Kind<
    //
    BT: behavior_type_traits::Element,
    ChildrenKind: RenderStateKindPollRenderWithParent<BT>,
    AttrsKind: UnpinnedNonReactiveRenderStateKind,
    AttrsPinnedKindUnpinned: UnpinnedNonReactiveRenderStateKind,
    AttrsPinnedKindPinned: PinnedNonReactiveRenderStateKind,
>(crate::elements::Kind<(BT, ChildrenKind, AttrsKind, AttrsPinnedKindUnpinned, AttrsPinnedKindPinned)>);

// region: ui handle

pub struct ParentWithChildren<P, C, PA> {
    parent: P,
    children: C,
    parent_attributes: PA,
}

/// The unmounted [`ParentWithChildren`] can only be constructed in [`UiHandle::unmount`],
/// in which case its children doesn't need to be re-mounted.
impl<P: UnmountedUiHandle<R>, C, PA, R: ?Sized> UnmountedUiHandle<R> for ParentWithChildren<P, C, PA>
where
    P::Mounted: behaviors::Element<R>,
{
    type Mounted = ParentWithChildren<P::Mounted, C, PA>;

    fn mount(self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
    where
        R: frender_dom::render::RenderWithContext,
    {
        ParentWithChildren {
            parent: self.parent.mount(render_context),
            children: self.children,
            parent_attributes: self.parent_attributes,
        }
    }
}

/// P must be an [`Element`](behaviors::Element) so that
/// when its children don't need to be unmounted or mounted.
impl<P: behaviors::Element<R>, C, PA, R: ?Sized> UiHandle<R> for ParentWithChildren<P, C, PA> {
    type Unmounted = ParentWithChildren<P::Unmounted, C, PA>;

    fn unmount(self, renderer: &mut R) -> Self::Unmounted {
        ParentWithChildren {
            parent: self.parent.unmount(renderer),
            children: self.children,
            parent_attributes: self.parent_attributes,
        }
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        self.parent.reposition(render_context)
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        self.parent.check_and_move_cursor(render_context)
    }

    fn assert_cursor_if_at_self(&self, render_context: &<R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        self.parent.assert_cursor_if_at_self(render_context)
    }
}

// endregion

// region: pinned

pin_project!(
    #[derive(Default)]
    pub struct ParentWithChildrenNonReactive<C, PA> {
        #[pin]
        children: C,
        #[pin]
        parent_attributes: PA,
    }
);

impl<BT, ChildrenKind, AttrsKind, AttrsPinnedKindUnpinned, AttrsPinnedKindPinned> PinnedRenderStateKind for Kind<BT, ChildrenKind, AttrsKind, AttrsPinnedKindUnpinned, AttrsPinnedKindPinned>
where
    BT: behavior_type_traits::Element,
    ChildrenKind: RenderStateKindPollRenderWithParent<BT>,
    AttrsKind: UnpinnedNonReactiveRenderStateKind,
    AttrsPinnedKindUnpinned: UnpinnedNonReactiveRenderStateKind,
    AttrsPinnedKindPinned: PinnedNonReactiveRenderStateKind,
{
    type PinnedUiHandle<R: RenderHtml + ?Sized> = ParentWithChildren<
        //
        BT::Element<R>,
        ChildrenKind::PinnedUiHandle<R>,
        AttrsKind::UnpinnedNonReactiveState<R>,
    >;
    type PinnedNonReactiveState<R: RenderHtml + ?Sized> = ParentWithChildrenNonReactive<
        //
        ChildrenKind::PinnedNonReactiveState<R>,
        AttrsPinnedKindPinned::PinnedNonReactiveState<R>,
    >;
    type PinnedReactiveState = ChildrenKind::PinnedReactiveState;
}

impl<BT, ChildrenKind, AttrsKind, AttrsPinnedKindUnpinned, AttrsPinnedKindPinned> PinnedRenderStateKindPollRender for Kind<BT, ChildrenKind, AttrsKind, AttrsPinnedKindUnpinned, AttrsPinnedKindPinned>
where
    BT: behavior_type_traits::Element,
    ChildrenKind: RenderStateKindPollRenderWithParent<BT>,
    AttrsKind: UnpinnedNonReactiveRenderStateKind,
    AttrsPinnedKindUnpinned: UnpinnedNonReactiveRenderStateKind,
    AttrsPinnedKindPinned: PinnedNonReactiveRenderStateKind,
{
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        states: crate::element::PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let RenderStates {
            ui_handle: ParentWithChildren { parent, children, parent_attributes: _ },
            non_reactive_state,
            reactive_state,
        } = states;
        ChildrenKind::pinned_poll_render_with_parent(
            renderer,
            parent.into_mut(),
            RenderStates {
                ui_handle: children,
                non_reactive_state: non_reactive_state.project().children,
                reactive_state,
            },
            cx,
        )
    }
}

// endregion

// region: unpinned

impl<BT, ChildrenKind, AttrsKind, AttrsPinnedKindUnpinned, AttrsPinnedKindPinned> UnpinnedRenderStateKind for Kind<BT, ChildrenKind, AttrsKind, AttrsPinnedKindUnpinned, AttrsPinnedKindPinned>
where
    BT: behavior_type_traits::Element,
    ChildrenKind: RenderStateKindPollRenderWithParent<BT>,
    AttrsKind: UnpinnedNonReactiveRenderStateKind,
    AttrsPinnedKindUnpinned: UnpinnedNonReactiveRenderStateKind,
    AttrsPinnedKindPinned: PinnedNonReactiveRenderStateKind,
{
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = ParentWithChildren<
        //
        BT::Element<R>,
        ChildrenKind::UnpinnedUiHandle<R>,
        (),
    >;
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = ParentWithChildrenNonReactive<
        //
        ChildrenKind::UnpinnedNonReactiveState<R>,
        (AttrsKind::UnpinnedNonReactiveState<R>, AttrsPinnedKindUnpinned::UnpinnedNonReactiveState<R>),
    >;
    type UnpinnedReactiveState = ChildrenKind::UnpinnedReactiveState;
}

impl<BT, ChildrenKind, AttrsKind, AttrsPinnedKindUnpinned, AttrsPinnedKindPinned> UnpinnedRenderStateKindPollRender for Kind<BT, ChildrenKind, AttrsKind, AttrsPinnedKindUnpinned, AttrsPinnedKindPinned>
where
    BT: behavior_type_traits::Element,
    ChildrenKind: RenderStateKindPollRenderWithParent<BT>,
    AttrsKind: UnpinnedNonReactiveRenderStateKind,
    AttrsPinnedKindUnpinned: UnpinnedNonReactiveRenderStateKind,
    AttrsPinnedKindPinned: PinnedNonReactiveRenderStateKind,
{
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        states: crate::element::UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let RenderStates {
            ui_handle: ParentWithChildren { parent, children, parent_attributes: _ },
            non_reactive_state: ParentWithChildrenNonReactive {
                children: non_reactive_state,
                parent_attributes: _,
            },
            reactive_state,
        } = states;
        ChildrenKind::unpinned_poll_render_with_parent(
            renderer,
            parent.into_mut(),
            RenderStates {
                ui_handle: children,
                non_reactive_state,
                reactive_state,
            },
            cx,
        )
    }
}

// endregion

impl<
        //
        BT,
        Children,
        Attrs,
        AttrsWithPinnedState,
    > CsrElement for Intrinsic<BT, Children, Attrs, AttrsWithPinnedState>
where
    BT: HasIntrinsicComponentTag + behavior_type_traits::Element,
    BT: CsrComponent<Children>,
    Attrs: UnpinnedRenderWithBehavior<BT>,
    AttrsWithPinnedState: UnpinnedRenderWithBehavior<BT> + PinnedRenderWithBehavior<BT>,
{
    type RenderStateKind = Kind<
        //
        BT,
        BT::ChildrenRenderStateKind,
        Attrs::UnpinnedRenderStateKind,
        AttrsWithPinnedState::UnpinnedRenderStateKind,
        AttrsWithPinnedState::PinnedRenderStateKind,
    >;

    fn pinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> crate::element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let Self {
            //
            type_marker,
            attributes,
            attributes_with_pinned_state,
            children,
        } = self;

        let PinMutRenderInitStates { non_reactive_state, reactive_state } = states;

        let non_reactive_state = non_reactive_state.project();
        let children_non_reactive_state = non_reactive_state.children;
        let parent_attributes_pinned_state = non_reactive_state.parent_attributes;

        let parent = BT::create_and_mount_ui_handle_of_type(render_context);
        let mut parent: BT::Element<Ctx::Renderer> = From::from(parent);

        let renderer = render_context.renderer_mut();
        let parent_attributes;
        let parent_b: &mut BT::OfBehaviorType<Ctx::Renderer> = parent.into_mut();
        {
            parent_attributes = Attrs::unpinned_render_init_with_behavior(attributes, renderer, parent_b);
            AttrsWithPinnedState::pinned_render_init_with_behavior(attributes_with_pinned_state, renderer, parent_b, parent_attributes_pinned_state);
        }

        let children_ui_handle = type_marker.children_pinned_render_init(
            children,
            renderer,
            parent_b,
            PinMutRenderInitStates {
                non_reactive_state: children_non_reactive_state,
                reactive_state,
            },
        );

        ParentWithChildren {
            parent,
            children: children_ui_handle,
            parent_attributes,
        }
    }

    fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        let Self {
            //
            type_marker,
            attributes,
            attributes_with_pinned_state,
            children,
        } = self;

        let RenderStates {
            ui_handle: ParentWithChildren {
                parent,
                children: children_ui_handle,
                parent_attributes,
            },
            non_reactive_state,
            reactive_state,
        } = states;

        let non_reactive_state = non_reactive_state.project();
        let children_non_reactive_state = non_reactive_state.children;
        let parent_attributes_pinned_state = non_reactive_state.parent_attributes;

        let renderer = render_context.renderer_mut();
        let parent_b: &mut BT::OfBehaviorType<Ctx::Renderer> = parent.into_mut();

        {
            Attrs::unpinned_render_update_with_behavior(attributes, renderer, parent_b, parent_attributes);
            AttrsWithPinnedState::pinned_render_init_with_behavior(attributes_with_pinned_state, renderer, parent_b, parent_attributes_pinned_state);
        }

        type_marker.children_pinned_render_update(
            children,
            renderer,
            parent_b,
            RenderStates {
                ui_handle: children_ui_handle,
                non_reactive_state: children_non_reactive_state,
                reactive_state,
            },
        );

        render_context.map_mut_render_context(|render_context| parent.check_and_move_cursor(render_context));
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> crate::element::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        let Self {
            type_marker,
            attributes,
            attributes_with_pinned_state,
            children,
        } = self;

        let parent = BT::create_and_mount_ui_handle_of_type(render_context);
        let mut parent: BT::Element<Ctx::Renderer> = From::from(parent);

        let renderer = render_context.renderer_mut();
        let parent_b: &mut BT::OfBehaviorType<Ctx::Renderer> = parent.into_mut();

        let parent_attributes = {
            (
                Attrs::unpinned_render_init_with_behavior(attributes, renderer, parent_b),
                AttrsWithPinnedState::unpinned_render_init_with_behavior(attributes_with_pinned_state, renderer, parent_b),
            )
        };

        let children_states = type_marker.children_unpinned_render_init(children, renderer, parent_b);

        RenderStates {
            ui_handle: ParentWithChildren {
                parent,
                children: children_states.ui_handle,
                parent_attributes: (),
            },
            non_reactive_state: ParentWithChildrenNonReactive {
                children: children_states.non_reactive_state,
                parent_attributes,
            },
            reactive_state: children_states.reactive_state,
        }
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: crate::element::UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        let Self {
            type_marker,
            attributes,
            attributes_with_pinned_state,
            children,
        } = self;

        let RenderStates {
            ui_handle: ParentWithChildren {
                parent,
                children: children_ui_handle,
                parent_attributes: (),
            },
            non_reactive_state: ParentWithChildrenNonReactive {
                children: children_non_reactive_state,
                parent_attributes: (parent_attributes, parent_attributes_pinned),
            },
            reactive_state,
        } = states;

        let renderer = render_context.renderer_mut();
        let parent_b: &mut BT::OfBehaviorType<Ctx::Renderer> = parent.into_mut();

        {
            Attrs::unpinned_render_update_with_behavior(attributes, renderer, parent_b, parent_attributes);
            AttrsWithPinnedState::unpinned_render_update_with_behavior(attributes_with_pinned_state, renderer, parent_b, parent_attributes_pinned);
        }

        type_marker.children_unpinned_render_update(
            children,
            renderer,
            parent_b,
            RenderStates {
                ui_handle: children_ui_handle,
                non_reactive_state: children_non_reactive_state,
                reactive_state,
            },
        );

        render_context.map_mut_render_context(|render_context| parent.check_and_move_cursor(render_context));
    }
}
