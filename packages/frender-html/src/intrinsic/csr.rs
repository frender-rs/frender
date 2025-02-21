use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Poll;

use frender_common::convert::IntoMut;
use frender_common::reactive_value::RenderInitPinned;
use frender_dom::csr::render::RenderWithContext;
use frender_dom::csr::StateUnmount;
use frender_dom::csr::{UiHandle, UnmountedUiHandle};
use pin_project_lite::pin_project;

use crate::dom::HasIntrinsicComponentTag;

use crate::csr::element::{self, HtmlRenderContext, PinnedRenderStateKind, PinnedRenderStateKindPollRender, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender};
use crate::csr::CsrElement;
use crate::element_types::RenderStateKindPollRenderWithParent;
use crate::html::{behavior_type_traits, behaviors};
use crate::intrinsic::Intrinsic;
use crate::update_element::{PinnedNonReactiveRenderStateKind, PinnedRenderWithBehavior, UnpinnedNonReactiveRenderStateKind, UnpinnedRenderWithBehavior};
use crate::{html::RenderHtml, CsrComponent};

enum Never {}
pub struct Kind<
    //
    BT: behavior_type_traits::Element,
    ChildrenKind: RenderStateKindPollRenderWithParent<BT>,
    AttrsKind: UnpinnedNonReactiveRenderStateKind,
    AttrsPinnedKindUnpinned: UnpinnedNonReactiveRenderStateKind,
    AttrsPinnedKindPinned: PinnedNonReactiveRenderStateKind,
>(Never, PhantomData<(BT, ChildrenKind, AttrsKind, AttrsPinnedKindUnpinned, AttrsPinnedKindPinned)>);

// region: ui handle

pub struct ParentWithChildren<P, C> {
    parent: P,
    children: C,
}

/// The unmounted [`ParentWithChildren`] can only be constructed in [`UiHandle::unmount`],
/// in which case its children doesn't need to be re-mounted.
impl<P: UnmountedUiHandle<R>, C, R: ?Sized> UnmountedUiHandle<R> for ParentWithChildren<P, C>
where
    P::Mounted: behaviors::Element<R>,
{
    type Mounted = ParentWithChildren<P::Mounted, C>;

    fn mount(self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
    where
        R: RenderWithContext,
    {
        ParentWithChildren {
            parent: self.parent.mount(render_context),
            children: self.children,
        }
    }
}

/// P must be an [`Element`](behaviors::Element) so that
/// when its children don't need to be unmounted or mounted.
impl<P: behaviors::Element<R>, C, R: ?Sized> UiHandle<R> for ParentWithChildren<P, C> {
    type Unmounted = ParentWithChildren<P::Unmounted, C>;

    fn unmount(self, renderer: &mut R) -> Self::Unmounted {
        ParentWithChildren {
            parent: self.parent.unmount(renderer),
            children: self.children,
        }
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        self.parent.reposition(render_context)
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        self.parent.check_and_move_cursor(render_context)
    }

    fn assert_cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        self.parent.assert_cursor_is_at_self(render_context)
    }
}

// endregion

// region: pinned

pin_project!(
    pub struct ParentWithChildrenState<C, PAU, PAP> {
        #[pin]
        children: C,
        parent_attributes_unpinned: PAU,
        #[pin]
        parent_attributes_pinned: PAP,
    }
);

impl<C: StateUnmount, PAU, PAP> StateUnmount for ParentWithChildrenState<C, PAU, PAP> {
    fn state_unmount(self: Pin<&mut Self>) {
        self.project().children.state_unmount()
    }
}

pub struct ParentWithChildrenStateUnpinned<C, PAU, PAP> {
    children: C,
    parent_attributes_unpinned: PAU,
    parent_attributes_pinned: PAP,
}

impl<C, PAU, PAP> Unpin for ParentWithChildrenStateUnpinned<C, PAU, PAP> {}

impl<C: StateUnmount + Unpin, PAU, PAP> StateUnmount for ParentWithChildrenStateUnpinned<C, PAU, PAP> {
    fn state_unmount(self: Pin<&mut Self>) {
        Pin::new(&mut self.get_mut().children).state_unmount()
    }
}

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
    >;
    type PinnedState<R: RenderHtml + ?Sized> = ParentWithChildrenState<
        //
        ChildrenKind::PinnedState<R>,
        AttrsKind::UnpinnedNonReactiveState<R>,
        AttrsPinnedKindPinned::PinnedNonReactiveState<R>,
    >;
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
        state: Pin<&mut Self::PinnedState<R>>,
        ParentWithChildren { parent, children }: &mut Self::PinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let state = state.project().children;
        ChildrenKind::pinned_poll_render_with_parent(renderer, parent.into_mut(), state, children, cx)
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
    >;
    type UnpinnedState<R: RenderHtml + ?Sized> = ParentWithChildrenStateUnpinned<
        //
        ChildrenKind::UnpinnedState<R>,
        AttrsKind::UnpinnedNonReactiveState<R>,
        AttrsPinnedKindUnpinned::UnpinnedNonReactiveState<R>,
    >;
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
        state: &mut Self::UnpinnedState<R>,
        ParentWithChildren { parent, children }: &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        ChildrenKind::unpinned_poll_render_with_parent(renderer, parent.into_mut(), &mut state.children, children, cx)
    }
}

// endregion

pub struct RenderInit<PB, P, UP, IPAP, IC> {
    _parent_as_behavior_type: PhantomData<PB>,
    _parent: PhantomData<P>,
    unmounted_parent: UP,
    init_parent_attributes_pinned: IPAP,
    init_children: IC,
}

impl<
        //
        PB,
        UP: UnmountedUiHandle<Ctx::Renderer>,
        P: IntoMut<PB> + From<UP::Mounted>,
        PAU,
        IPAP: for<'r, 'p> RenderInitPinned<(&'r mut Ctx::Renderer, &'p mut PB), SPAP, Output = ()>,
        IC: for<'r, 'p> RenderInitPinned<(&'r mut Ctx::Renderer, &'p mut PB), SC, Output = C>,
        C,
        SC,
        SPAP,
        Ctx: ?Sized + HtmlRenderContext,
    > RenderInitPinned<&mut Ctx, ParentWithChildrenState<SC, PAU, SPAP>> for RenderInit<PB, P, UP, IPAP, IC>
{
    type Output = ParentWithChildren<P, C>;
    fn render_init_pinned(self, render_context: &mut Ctx, state: Pin<&mut ParentWithChildrenState<SC, PAU, SPAP>>) -> Self::Output {
        let Self {
            _parent_as_behavior_type: PhantomData,
            _parent,
            unmounted_parent,
            init_parent_attributes_pinned,
            init_children,
        } = self;

        let parent = render_context.map_mut_render_context(|render_context| unmounted_parent.mount(render_context));
        let mut parent: P = From::from(parent);

        let state = state.project();

        let p: &mut PB = parent.into_mut();

        let renderer = render_context.renderer_mut();

        let () = init_parent_attributes_pinned.render_init_pinned((renderer, p), state.parent_attributes_pinned);
        let children = init_children.render_init_pinned((renderer, p), state.children);
        ParentWithChildren { parent, children }
    }
}

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

    type PinnedRenderInit<R: ?Sized + RenderHtml> = RenderInit<
        //
        BT::OfBehaviorType<R>,
        BT::Element<R>,
        BT::UnmountedUiHandle<R>,
        AttrsWithPinnedState::PinnedRenderInitWithBehavior<R>,
        BT::ChildrenPinnedRenderInit<R>,
    >;

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
    ) -> (
        //
        element::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
    ) {
        use frender_dom::csr::ProvideMutMounted as _;

        let Self {
            //
            type_marker,
            attributes,
            attributes_with_pinned_state,
            children,
        } = self;

        let mut unmounted_parent = BT::create_unmounted_ui_handle_of_type(renderer);

        let (init_state, init_ap, init_children) = unmounted_parent.provide_mut_mounted(renderer, |renderer, parent| {
            let parent_b: &mut BT::OfBehaviorType<Renderer> = parent.into_mut();

            let state_au = Attrs::unpinned_render_init_with_behavior(attributes, renderer, parent_b);
            let (state_ap, init_ap) = AttrsWithPinnedState::pinned_render_init_with_behavior(attributes_with_pinned_state, renderer, parent_b);

            let (children, init_children) = type_marker.children_pinned_render_init(children, renderer, parent_b);

            (
                ParentWithChildrenState {
                    children,
                    parent_attributes_unpinned: state_au,
                    parent_attributes_pinned: state_ap,
                },
                init_ap,
                init_children,
            )
        });

        (
            //
            init_state,
            RenderInit {
                _parent_as_behavior_type: PhantomData,
                _parent: PhantomData,
                unmounted_parent,
                init_parent_attributes_pinned: init_ap,
                init_children,
            },
        )
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: Pin<&mut element::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
        unmounted_ui_handle: element::PinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> element::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let Self {
            //
            type_marker,
            attributes,
            attributes_with_pinned_state,
            children,
        } = self;
        let reused_state = reused_state.project();

        let ParentWithChildren { parent, children: mut children_ui_handle } = unmounted_ui_handle;

        let parent = render_context.map_mut_render_context(|render_context| parent.mount(render_context));
        let mut parent: BT::Element<Ctx::Renderer> = From::from(parent);

        let renderer = render_context.renderer_mut();
        let parent_b: &mut BT::OfBehaviorType<Ctx::Renderer> = parent.into_mut();

        Attrs::unpinned_render_update_with_behavior(attributes, renderer, parent_b, reused_state.parent_attributes_unpinned);
        AttrsWithPinnedState::pinned_render_update_with_behavior(attributes_with_pinned_state, renderer, parent_b, reused_state.parent_attributes_pinned);

        type_marker.children_pinned_render_update(children, renderer, parent_b, reused_state.children, &mut children_ui_handle);

        ParentWithChildren { parent, children: children_ui_handle }
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut element::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ParentWithChildren { parent, children: children_ui_handle }: &mut element::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        let Self {
            //
            type_marker,
            attributes,
            attributes_with_pinned_state,
            children,
        } = self;

        let state = state.project();

        let parent_b: &mut BT::OfBehaviorType<Renderer> = parent.into_mut();

        Attrs::unpinned_render_update_with_behavior(attributes, renderer, parent_b, state.parent_attributes_unpinned);
        AttrsWithPinnedState::pinned_render_update_with_behavior(attributes_with_pinned_state, renderer, parent_b, state.parent_attributes_pinned);

        type_marker.children_pinned_render_update(children, renderer, parent_b, state.children, children_ui_handle);
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> (
        //
        element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) {
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

        let parent_attributes_unpinned = Attrs::unpinned_render_init_with_behavior(attributes, renderer, parent_b);
        let parent_attributes_pinned = AttrsWithPinnedState::unpinned_render_init_with_behavior(attributes_with_pinned_state, renderer, parent_b);

        let (state_children, children) = type_marker.children_unpinned_render_init(children, renderer, parent_b);

        (
            ParentWithChildrenStateUnpinned {
                children: state_children,
                parent_attributes_unpinned,
                parent_attributes_pinned,
            },
            ParentWithChildren { parent, children },
        )
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        ParentWithChildrenStateUnpinned {
            children: reused_children,
            parent_attributes_unpinned: reused_pau,
            parent_attributes_pinned: reused_pap,
        }: &mut element::UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        ParentWithChildren { parent, children: mut children_ui_handle }: element::UnpinnedUnmountedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) -> element::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let Self {
            type_marker,
            attributes,
            attributes_with_pinned_state,
            children,
        } = self;

        let parent = render_context.map_mut_render_context(|render_context| parent.mount(render_context));
        let mut parent: BT::Element<Ctx::Renderer> = From::from(parent);

        let renderer = render_context.renderer_mut();
        let parent_b: &mut BT::OfBehaviorType<Ctx::Renderer> = parent.into_mut();

        Attrs::unpinned_render_update_with_behavior(attributes, renderer, parent_b, reused_pau);
        AttrsWithPinnedState::unpinned_render_update_with_behavior(attributes_with_pinned_state, renderer, parent_b, reused_pap);

        type_marker.children_unpinned_render_update(children, renderer, parent_b, reused_children, &mut children_ui_handle);

        ParentWithChildren { parent, children: children_ui_handle }
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        ParentWithChildrenStateUnpinned {
            children: state_children,
            parent_attributes_unpinned: state_pau,
            parent_attributes_pinned: state_pap,
        }: &mut element::UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ParentWithChildren { parent, children: children_ui_handle }: &mut element::UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        let Self {
            type_marker,
            attributes,
            attributes_with_pinned_state,
            children,
        } = self;

        let parent_b: &mut BT::OfBehaviorType<Renderer> = parent.into_mut();

        Attrs::unpinned_render_update_with_behavior(attributes, renderer, parent_b, state_pau);
        AttrsWithPinnedState::unpinned_render_update_with_behavior(attributes_with_pinned_state, renderer, parent_b, state_pap);

        type_marker.children_unpinned_render_update(children, renderer, parent_b, state_children, children_ui_handle);
    }
}
