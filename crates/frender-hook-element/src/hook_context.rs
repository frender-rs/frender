pub trait HookContext: Sized {
    type ContextData;

    fn get_context_data(this: &Self) -> Self::ContextData;

    fn replace_context_data(this: &mut Self, old_context: Self::ContextData);
}

#[cfg(feature = "dom")]
impl HookContext for frender_csr::Dom {
    type ContextData = frender_csr::NextNodePosition;

    #[inline]
    fn get_context_data(this: &Self) -> Self::ContextData {
        this.next_node_position.clone()
    }

    #[inline]
    fn replace_context_data(this: &mut Self, old_context: Self::ContextData) {
        this.next_node_position = old_context
    }
}
