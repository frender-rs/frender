use frender_dom::render::RenderContext;
use frender_dom::RenderStateWithParentElementsHandle;

use crate::dom::component::HasIntrinsicComponentTag;

use crate::element_types::RenderStateWithPehKind;
use crate::html::behavior_type_traits;
use crate::{CreateNode, CsrComponent, HtmlRenderContext, RenderHtml, UnpinnedRenderStateOfContext, UpdateNodeNonReactive, UpdateNodeNonReactivePinned};

use crate::{Element, RenderState};

#[derive(Default)]
pub struct ElementAndMounted<E> {
    pub element: E,
    pub mounted: bool,
}

pub struct Kind<C: behavior_type_traits::Element, ChildrenKind: RenderStateWithPehKind<C>, Attrs, EventListeners>(crate::elements::Kind<(C, ChildrenKind, Attrs, EventListeners)>);

impl<C: behavior_type_traits::Element, ChildrenKind: RenderStateWithPehKind<C>, Attrs: UpdateNodeNonReactive<C>, EventListeners: UpdateNodeNonReactive<C> + UpdateNodeNonReactivePinned<C>> crate::RenderStateKindPinned
    for Kind<C, ChildrenKind, Attrs, EventListeners>
{
    type RenderState<R: RenderHtml + ?Sized> = IntrinsicElementRenderState<
        C::Element<R>,
        ElementPropsState<
            //
            <ChildrenKind as RenderStateWithPehKind<C>>::RenderStateWithPeh<R>,
            <Attrs as UpdateNodeNonReactive<C>>::State<R>,
            <EventListeners as UpdateNodeNonReactivePinned<C>>::StatePinned<R>,
        >,
    >;
}
impl<C: behavior_type_traits::Element, ChildrenKind: RenderStateWithPehKind<C>, Attrs: UpdateNodeNonReactive<C>, EventListeners: UpdateNodeNonReactive<C> + UpdateNodeNonReactivePinned<C>> crate::RenderStateKindUnpinned
    for Kind<C, ChildrenKind, Attrs, EventListeners>
{
    type UnpinnedRenderState<R: RenderHtml + ?Sized> = IntrinsicElementRenderState<
        C::Element<R>,
        ElementPropsState<
            //
            <ChildrenKind as RenderStateWithPehKind<C>>::RenderStateWithPehUnpinned<R>,
            (<Attrs as UpdateNodeNonReactive<C>>::State<R>, <EventListeners as UpdateNodeNonReactive<C>>::State<R>),
            (),
        >,
    >;
}

pin_project_lite::pin_project!(
    pub struct IntrinsicElementRenderState<E, S> {
        element_and_mounted: Option<ElementAndMounted<E>>,
        #[pin]
        props_state: S,
    }
);

impl<E, S: Default> Default for IntrinsicElementRenderState<E, S> {
    fn default() -> Self {
        Self {
            element_and_mounted: None,
            props_state: Default::default(),
        }
    }
}

impl<
        //
        R: ?Sized,
        E: crate::html::behaviors::Element<R>,
        S: RenderStateWithParentElementsHandle<E, R>,
    > RenderState<R> for IntrinsicElementRenderState<E, S>
{
    fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
        let this = self.project();
        if let Some(ElementAndMounted { element, mounted }) = this.element_and_mounted {
            if *mounted {
                *mounted = false;
                // renderer.remove_node(element);
                element.remove_self(renderer);
                this.props_state.state_unmount_with_peh(element);
            }
        }
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        let this = self.project();
        match this.element_and_mounted {
            // TODO: Do we need to record whether state_unmounted?
            Some(v) if v.mounted => {
                this.props_state.state_unmount_with_peh(&mut v.element);
            }
            _ => {}
        }
    }

    fn poll_render(self: std::pin::Pin<&mut Self>, renderer: &mut R, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        let this = self.project();

        let element = match this.element_and_mounted {
            Some(v) if v.mounted => &mut v.element,
            _ => return std::task::Poll::Ready(()),
        };

        S::poll_render_with_peh(this.props_state, element, renderer, cx)
    }

    /// children states are not checked
    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        match &self.element_and_mounted {
            Some(ElementAndMounted { element, mounted: true }) => element.check_and_move_cursor_after_self(render_context),
            _ => {}
        }
    }
}

pin_project_lite::pin_project! {
    #[derive(Default)]
    pub struct ElementPropsState<C, A, EL> {
        #[pin]
        children_render_state: C,
        attrs_state: A,
        #[pin]
        event_listeners: EL,
    }
}

impl<
        //
        PEH: ?Sized,
        R: ?Sized,
        C: RenderStateWithParentElementsHandle<PEH, R>,
        A,
        EL,
    > RenderStateWithParentElementsHandle<PEH, R> for ElementPropsState<C, A, EL>
{
    frender_dom::proxy_render_state_with_peh!(|self| -> (PEH, R) { self.project().children_render_state });
}

impl<
        //
        R: ?Sized,
        C: RenderState<R>,
        A,
        EL,
    > RenderState<R> for ElementPropsState<C, A, EL>
{
    fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
        self.project().children_render_state.unmount(renderer)
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        self.project().children_render_state.state_unmount()
    }

    fn poll_render(self: std::pin::Pin<&mut Self>, renderer: &mut R, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        self.project().children_render_state.poll_render(renderer, cx)
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        self.children_render_state.check_and_move_cursor(render_context)
    }
}

impl<
        //
        C: HasIntrinsicComponentTag + crate::html::behavior_type_traits::Element + CreateNode,
        Children,
        Attrs,
        AttrsWithPinnedState,
    > Element for crate::intrinsic::Intrinsic<C, Children, Attrs, AttrsWithPinnedState>
where
    C: CsrComponent<Children>,
    Attrs: UpdateNodeNonReactive<C>,
    AttrsWithPinnedState: UpdateNodeNonReactivePinned<C> + UpdateNodeNonReactive<C>,
    // ssr bounds
    // TODO: remove ssr bounds from csr implementations
    Attrs: crate::dom::component::IntoSpaceAndHtmlAttributesOrEmpty,
    C: crate::dom::component::SsrComponent<Children>,
{
    type RenderStateKind = Kind<C, C::ChildrenRenderStateKind, Attrs, AttrsWithPinnedState>; // TODO: shouldn't be generic over P

    fn render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: std::pin::Pin<&mut crate::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
        force_reposition: bool,
    ) {
        {
            let render_state = render_state.project();

            let props_state = render_state.props_state.project();

            let Self {
                //
                type_marker,
                attributes,
                attributes_with_pinned_state,
                children,
            } = self;

            let element_and_mounted = render_state.element_and_mounted.get_or_insert_with(|| ElementAndMounted {
                element: <C::Element<Ctx::Renderer>>::from(<C as CreateNode>::create_node(render_context.renderer_mut())),
                mounted: false,
            });

            render_context.map_mut_render_context(|render_context| {
                update_element_maybe_reposition(
                    element_and_mounted,
                    render_context,
                    |element, renderer| {
                        let node = frender_common::convert::IntoMut::into_mut(element);
                        Attrs::update_node_non_reactive(attributes, renderer, node, props_state.attrs_state);
                        AttrsWithPinnedState::update_node_non_reactive_pinned(attributes_with_pinned_state, renderer, node, props_state.event_listeners);
                        type_marker.children_render_update(children, element, renderer, props_state.children_render_state)
                    },
                    force_reposition,
                )
            })
        }
    }

    fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: &mut UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
        force_reposition: bool,
    ) {
        let props_state = &mut render_state.props_state;

        let Self {
            type_marker,
            attributes,
            children,
            attributes_with_pinned_state,
        } = self;

        let element_and_mounted = render_state.element_and_mounted.get_or_insert_with(|| ElementAndMounted {
            element: <C as CreateNode>::create_node(render_context.renderer_mut()).into(),
            mounted: false,
        });

        render_context.map_mut_render_context(|render_context| {
            update_element_maybe_reposition(
                element_and_mounted,
                render_context,
                |element, renderer| {
                    let (attrs_state, event_listeners_state) = &mut props_state.attrs_state;
                    let node = frender_common::convert::IntoMut::into_mut(element);
                    Attrs::update_node_non_reactive(attributes, renderer, node, attrs_state);
                    AttrsWithPinnedState::update_node_non_reactive(attributes_with_pinned_state, renderer, node, event_listeners_state);
                    type_marker.children_unpinned_render_update(children, element, renderer, &mut props_state.children_render_state)
                },
                force_reposition,
            )
        })
    }
}

fn update_element_maybe_reposition<E: crate::html::behaviors::Element<R>, R: ?Sized + RenderHtml>(
    element_and_mounted: &mut ElementAndMounted<E>,
    render_context: &mut R::RenderContext<'_>,
    update: impl FnOnce(&mut E, &mut R),
    force_reposition: bool,
) {
    let ElementAndMounted { element, mounted } = element_and_mounted;

    // web_sys::console::log_5(
    //     &"intrinsic::update_element_maybe_reposition".into(),
    //     &"mounted=".into(),
    //     &(*mounted).into(),
    //     &"force_reposition=".into(),
    //     &force_reposition.into(),
    // );

    update(element, render_context.renderer_mut());

    element.readd_self(render_context, force_reposition || !*mounted);
    *mounted = true;
}
