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
