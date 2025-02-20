use crate::{
    reactive_value::{ReactiveValue, ReusableRendererOfKind},
    value_kind::ValueKind,
};

use super::{CachedNonReactiveValue, State};

impl<T: CachedNonReactiveValue<VK>, VK: ?Sized + ValueKind> ReactiveValue<VK> for T {
    type PinnedState = State<VK, T::Cache>;
    type PinnedRenderInit = super::RenderInit<T::RenderInit>;
    type UnpinnedState = State<VK, T::Cache>;

    crate::impl_reactive_value_with_mixed_unpinned!(
        type ReactiveValueKind = VK;
    );

    fn pinned_render_init(self) -> (Self::PinnedState, Self::PinnedRenderInit) {
        let (cache, render_init) = self.into_cache_and_render_init();
        (State::new(cache), super::RenderInit(render_init))
    }

    fn unpinned_render_init_by_reusing<Out>(
        self,
        renderer: impl ReusableRendererOfKind<VK, Output = Out>,
        State(::core::marker::PhantomData, cache): &mut Self::PinnedState,
    ) -> Out {
        if T::match_cache(&self, cache) {
            renderer.reuse(self.cached_into_provide_value())
        } else {
            self.update_into_cache_and_render(|v| renderer.render(v), cache)
        }
    }

    fn unpinned_render_update<Out>(
        self,
        renderer: impl FnOnce(VK::Value<'_>) -> Out,
        super::State(::core::marker::PhantomData, cache): &mut Self::PinnedState,
    ) -> Option<Out> {
        T::maybe_update_into_cache_and_render(self, renderer, cache)
    }
}
