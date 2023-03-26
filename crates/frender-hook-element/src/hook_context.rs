pub trait HookContext: Sized {
    type ContextData; // TODO: rename

    fn get_context_data(this: &Self) -> Self::ContextData;

    fn replace_context_data(this: &mut Self, old_context: Self::ContextData);
}

#[cfg(feature = "dom")]
impl HookContext for frender_dom::Dom {
    type ContextData = frender_dom::NextNodePosition;

    #[inline]
    fn get_context_data(this: &Self) -> Self::ContextData {
        this.next_node_position.clone()
    }

    #[inline]
    fn replace_context_data(this: &mut Self, old_context: Self::ContextData) {
        this.next_node_position = old_context
    }
}

#[cfg(feature = "ssr")]
impl<W: frender_ssr::AsyncWrite + Unpin> HookContext for frender_ssr::SsrContext<W> {
    type ContextData = ();

    #[inline]
    fn get_context_data(_: &Self) -> Self::ContextData {}

    fn replace_context_data(_: &mut Self, _: Self::ContextData) {}
}
