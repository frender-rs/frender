pub trait RenderTextFrom<Text, V: ?Sized> {
    // let text = dom_ctx.document.create_text_node(&data);
    // dom_ctx
    //     .next_node_position
    //     .add_node(Cow::Owned(text.clone().into()));
    fn render_text_from(&mut self, v: &V) -> Text;
    fn update_text_from(&mut self, text: &mut Text, v: &V);
}

pub trait RemoveNode<Node> {
    fn remove_node(&mut self, node: &mut Node);
}

pub trait ReaddNode<Node> {
    // let node = Cow::Owned(self.node.clone().into());
    // if force_reposition || self.unmounted {
    //     dom_ctx.next_node_position.add_node(node)
    // } else {
    //     dom_ctx.next_node_position.set_as_insert_after(node);
    // }
    fn readd_node(&mut self, node: &mut Node, force_reposition: bool);
}

pub trait MarkPositionAtFirstChild<Node> {
    fn mark_position_at_first_child(&mut self, node: &mut Node);
}

pub trait MarkPositionAfter<Node> {
    fn mark_position_after(&mut self, node: &mut Node);
}

pub trait RenderHtml:
    RenderTextFrom<Self::Text, str>
    + RenderTextFrom<Self::Text, i8>
    + RenderTextFrom<Self::Text, u8>
    + RenderTextFrom<Self::Text, i16>
    + RenderTextFrom<Self::Text, u16>
    + RenderTextFrom<Self::Text, i32>
    + RenderTextFrom<Self::Text, u32>
    + RenderTextFrom<Self::Text, i64>
    + RenderTextFrom<Self::Text, u64>
    + RenderTextFrom<Self::Text, i128>
    + RenderTextFrom<Self::Text, u128>
    + RenderTextFrom<Self::Text, isize>
    + RenderTextFrom<Self::Text, usize>
    + RenderTextFrom<Self::Text, f32>
    + RenderTextFrom<Self::Text, f64>
    + RenderTextFrom<Self::Text, bool>
    + RenderTextFrom<Self::Text, char>
    + ReaddNode<Self::Text>
    + RemoveNode<Self::Text>
    + MarkPositionAfter<Self::Text>
    + ReaddNode<Self::Element>
    + RemoveNode<Self::Element>
    + MarkPositionAfter<Self::Element>
    + MarkPositionAtFirstChild<Self::Element>
{
    type Text;
    type Element;
}
