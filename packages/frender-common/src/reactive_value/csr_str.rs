use std::{marker::PhantomData, pin::Pin, task::Poll};

use crate::{
    csr::StateUnmount, reactive_value::ProvideValueOfKind, strings::CsrStr, IntoStaticStrCache,
    TempStr, ToAsRefStr,
};

use super::{
    ReactiveValue, ReactiveValueKind, ReactiveValueRenderInitPinned, ReactiveValueState,
    ReusableRendererOfKind,
};

impl ReactiveValueKind for str {
    type Value<'a> = TempStr<&'a str>;
}

#[derive(Debug, Default)]
pub struct State<C>(pub C);

impl<C> Unpin for State<C> {}

impl<C> StateUnmount for State<C> {
    fn state_unmount(self: Pin<&mut Self>) {}
}

impl<C> ReactiveValueState for State<C> {
    type ReactiveValueKind = str;

    /// The string has been rendered by [`ReactiveValue::<str>::`]
    /// so there are no more updates.
    fn poll_render(
        self: Pin<&mut Self>,
        _: impl FnMut(<Self::ReactiveValueKind as super::ReactiveValueKind>::Value<'_>),
        _: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Poll::Ready(())
    }
}

pub struct RenderInit<Cache>(PhantomData<Cache>);

impl<Cache: ToAsRefStr> ReactiveValueRenderInitPinned<str> for RenderInit<Cache> {
    type State = State<Cache>;

    fn render_init_pinned<Out>(
        self,
        renderer: impl FnOnce(TempStr<&str>) -> Out,
        state: Pin<&mut Self::State>,
    ) -> Out {
        let State(cache) = state.get_mut();
        renderer(TempStr(cache.to_as_ref_str().as_ref()))
    }
}

/// [`CsrStr`] are non reactive
impl<T: CsrStr> ReactiveValue<str> for T {
    super::impl_reactive_value_unpinned_with_pinned!(
        type ReactiveValueKind = str;
    );

    type PinnedState = State<T::StaticStrCache>;
    type PinnedRenderInit = RenderInit<T::StaticStrCache>;

    fn pinned_render_init(self) -> (Self::PinnedState, Self::PinnedRenderInit) {
        (
            State(self.into_into_static_str_cache().into_static_str_cache()),
            RenderInit(PhantomData),
        )
    }

    fn pinned_render_init_by_reusing<Out>(
        self,
        renderer: impl ReusableRendererOfKind<str, Output = Out>,
        reused_state: Pin<&mut Self::PinnedState>,
    ) -> Out {
        let State(cache) = reused_state.get_mut();
        if self.match_static_str_cache(cache) {
            struct Provide<'a, C>(&'a C);

            impl<C: ToAsRefStr> ProvideValueOfKind<str> for Provide<'_, C> {
                fn provide_value_of_kind<Out>(
                    self,
                    f: impl FnOnce(<str as super::ReactiveValueKind>::Value<'_>) -> Out,
                ) -> Out {
                    f(TempStr(self.0.to_as_ref_str().as_ref()))
                }
            }
            renderer.reuse(Provide(cache))
        } else {
            self.into_into_static_str_cache()
                .update_into_static_str_cache(cache);
            renderer.render(TempStr(cache.to_as_ref_str().as_ref()))
        }
    }

    fn pinned_render_update<Out>(
        self,
        renderer: impl FnOnce(<str as super::ReactiveValueKind>::Value<'_>) -> Out,
        state: Pin<&mut Self::PinnedState>,
    ) -> Option<Out> {
        let State(cache) = state.get_mut();
        if self.match_static_str_cache(cache) {
            None
        } else {
            self.into_into_static_str_cache()
                .update_into_static_str_cache(cache);
            Some(renderer(TempStr(cache.to_as_ref_str().as_ref())))
        }
    }
}
