use crate::RenderHtml;

pub trait BehaviorType {
    type NodeOfBehaviorType<Renderer: ?Sized + RenderHtml>;
}

pub trait UpdateNodeNonReactive<BT: BehaviorType> {
    type State<Renderer: ?Sized + RenderHtml>: Default;

    fn update_node_non_reactive<Renderer: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut Renderer,
        node: &mut BT::NodeOfBehaviorType<Renderer>,
        state: &mut Self::State<Renderer>,
    );
}

impl<BT: BehaviorType> UpdateNodeNonReactive<BT> for () {
    type State<Renderer: ?Sized + RenderHtml> = ();

    fn update_node_non_reactive<Renderer: ?Sized + RenderHtml>(
        //
        _: Self,
        _: &mut Renderer,
        _: &mut BT::NodeOfBehaviorType<Renderer>,
        _: &mut Self::State<Renderer>,
    ) {
    }
}

impl<BT: BehaviorType, A: UpdateNodeNonReactive<BT>, B: UpdateNodeNonReactive<BT>> UpdateNodeNonReactive<BT> for (A, B) {
    type State<Renderer: ?Sized + RenderHtml> = (A::State<Renderer>, B::State<Renderer>);

    fn update_node_non_reactive<Renderer: ?Sized + RenderHtml>(
        //
        (a, b): Self,
        renderer: &mut Renderer,
        node: &mut BT::NodeOfBehaviorType<Renderer>,
        (state_a, state_b): &mut Self::State<Renderer>,
    ) {
        A::update_node_non_reactive(a, renderer, node, state_a);
        B::update_node_non_reactive(b, renderer, node, state_b);
    }
}

pub trait UpdateNodeNonReactivePinned<BT: BehaviorType> {
    type StatePinned<Renderer: ?Sized + RenderHtml>: Default;

    fn update_node_non_reactive_pinned<Renderer: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut Renderer,
        node: &mut BT::NodeOfBehaviorType<Renderer>,
        state: std::pin::Pin<&mut Self::StatePinned<Renderer>>,
    );
}

impl<BT: BehaviorType> UpdateNodeNonReactivePinned<BT> for () {
    type StatePinned<Renderer: ?Sized + RenderHtml> = ();

    fn update_node_non_reactive_pinned<Renderer: ?Sized + RenderHtml>(
        //
        _: Self,
        _: &mut Renderer,
        _: &mut BT::NodeOfBehaviorType<Renderer>,
        _: std::pin::Pin<&mut Self::StatePinned<Renderer>>,
    ) {
    }
}

pin_project_lite::pin_project!(
    #[derive(Default)]
    pub struct TwoState<A, B> {
        #[pin]
        a: A,
        #[pin]
        b: B,
    }
);

impl<BT: BehaviorType, A: UpdateNodeNonReactivePinned<BT>, B: UpdateNodeNonReactivePinned<BT>> UpdateNodeNonReactivePinned<BT> for (A, B) {
    type StatePinned<Renderer: ?Sized + RenderHtml> = TwoState<A::StatePinned<Renderer>, B::StatePinned<Renderer>>;

    fn update_node_non_reactive_pinned<Renderer: ?Sized + RenderHtml>(
        //
        (a, b): Self,
        renderer: &mut Renderer,
        node: &mut BT::NodeOfBehaviorType<Renderer>,
        state: std::pin::Pin<&mut Self::StatePinned<Renderer>>,
    ) {
        let state = state.project();
        A::update_node_non_reactive_pinned(a, renderer, node, state.a);
        B::update_node_non_reactive_pinned(b, renderer, node, state.b);
    }
}
