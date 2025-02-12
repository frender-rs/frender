//! # Non-reactive values
//!
//! ## Notable non-reactive values
//!
//! ### Copied
//!
//! `Uncached<T>: ReactiveValue<T>` where `T: 'static + Copy`
//!
//! `T: ReactiveValue<T>` where `T: 'static + PartialEq + Copy`, `type Cache = T`
//!
//! ### Cloned
//!
//! `Uncached<T>: ReactiveValue<KindOfOwned<T>>` where `T: 'static + Clone`
//!
//! `T: ReactiveValue<KindOfOwned<T>>` where `T: 'static + PartialEq + Clone`, `type Cache = T`
//!
//! ### Ref
//!
//! `Uncached<&T>: ReactiveValue<KindOfRef<T>>` where `T: ?Sized`
//!
//! `&T: ReactiveValue<KindOfRef<T>>` where `T: ?Sized + PartialEq + ToOwned`, `type Cache = T::Owned`
//!
//! ### Refed
//!
//! `Uncached<Refed<T>>: ReactiveValue<KindOfRef<T>>`
//!
//! `Refed<T>: ReactiveValue<KindOfRef<T>>` where `T: PartialEq`, `type Cache = T`
//!
//! `Uncached<Rc<T>>` proxies `Uncached<Refed<Rc<T>>>`,
//! `Rc<T>` proxies `Refed<Rc<T>>`.
//!
//! `Uncached<Arc<T>>` proxies `Uncached<Refed<Arc<T>>>`,
//! `Arc<T>` proxies `Refed<Arc<T>>`.
//!
//! ### Cow<'static, T>
//!
//! `Uncached<Cow<'static, T>>: ReactiveValue<KindOfStaticRefOrTempOwned<T>>` where `T: ?Sized + 'static + ToOwned`
//! `Cow<'static, T>: ReactiveValue<KindOfStaticRefOrTempOwned<T>>` where `T: ?Sized + 'static + ToOwned + PartialEq`, `type Cache = Self`
//!
//! `Uncached<&Cow<'static, T>>: ReactiveValue<KindOfStaticRefOrTempOwned<T>>` where `T: ?Sized + 'static + ToOwned`
//! `&Cow<'static, T>: ReactiveValue<KindOfStaticRefOrTempOwned<T>>` where `T: ?Sized + 'static + ToOwned + PartialEq`, `type Cache = Cow<'static, T>`
//!
//! `Uncached<StaticRefOrTempOwned<'_, T>>: ReactiveValue<KindOfStaticRefOrTempOwned<T>>` where `T: ?Sized + 'static + ToOwned`
//! `StaticRefOrTempOwned<'_, T>: ReactiveValue<KindOfStaticRefOrTempOwned<T>>` where `T: ?Sized + 'static + ToOwned + PartialEq`, `type Cache = Cow<'static, T>`
//!
//! ### CsrStr
//!
//! `S: ReactiveValue<str>` where [`S: CsrStr`], [`type Cache = S::StaticStrCache`]
//!
//! [`S: CsrStr`]: crate::strings::CsrStr
//! [`type Cache = S::StaticStrCache`]: crate::strings::CsrStr::StaticStrCache

use std::{marker::PhantomData, pin::Pin, task::Poll};

use crate::{csr::StateUnmount, value_kind::ValueKind};

use super::{
    ProvideValueOfKind, ReactiveValueRenderInitPinned, ReactiveValueState, RenderInitPinned,
};

mod cached;

pub mod clone_as_cache_if_cache_miss;
pub mod clone_as_value_if_cache_miss;
pub mod copied;
pub mod cow_static;
pub mod csr_str;
pub mod owned;
pub mod r#ref;
pub mod refed;
pub mod temp_str;
pub mod uncached;

pub trait UncachedNonReactiveValue<VK: ?Sized + ValueKind> {
    type UncachedIntoProvideValue: ProvideValueOfKind<VK>;
    fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue;
}

pub trait CachedNonReactiveValue<VK: ?Sized + ValueKind> {
    type Cache;
    type RenderInit: CachedNonReactiveValueRenderInit<VK, Self::Cache>;

    type CachedIntoProvideValue: ProvideValueOfKind<VK>;
    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue;

    fn match_cache(&self, cache: &Self::Cache) -> bool;
    fn not_match_cache(&self, cache: &Self::Cache) -> bool {
        !self.match_cache(cache)
    }

    fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit);

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(VK::Value<'_>) -> Out,
        cache: &mut Self::Cache,
    ) -> Out;
}

pub trait CachedNonReactiveValueRenderInit<VK: ?Sized + ValueKind, Cache> {
    fn cached_non_reactive_value_render_init<Out>(
        self,
        renderer: impl FnOnce(VK::Value<'_>) -> Out,
        cache: &mut Cache,
    ) -> Out;
}

pub struct State<VK: ?Sized, Cache>(pub PhantomData<VK>, pub Cache);

impl<VK: ?Sized, Cache> State<VK, Cache> {
    pub fn new(cache: Cache) -> Self {
        Self(PhantomData, cache)
    }
}

impl<VK: ?Sized, Cache> Unpin for State<VK, Cache> {}
impl<VK: ?Sized, Cache> StateUnmount for State<VK, Cache> {
    fn state_unmount(self: Pin<&mut Self>) {}
}
impl<VK: ?Sized + ValueKind, Cache> ReactiveValueState for State<VK, Cache> {
    type ReactiveValueKind = VK;

    fn poll_render(
        self: Pin<&mut Self>,
        _: impl FnMut(VK::Value<'_>),
        _: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Poll::Ready(())
    }
}

pub struct RenderInit<T>(pub T);

impl<
        T: CachedNonReactiveValueRenderInit<VK, Cache>,
        R: FnOnce(VK::Value<'_>) -> Out,
        Out,
        VK: ?Sized + ValueKind,
        Cache,
    > RenderInitPinned<R, State<VK, Cache>> for RenderInit<T>
{
    type Output = Out;
    fn render_init_pinned(
        self,
        renderer: R,
        state: std::pin::Pin<&mut State<VK, Cache>>,
    ) -> Self::Output {
        let State(::core::marker::PhantomData, cache) = state.get_mut();

        self.0
            .cached_non_reactive_value_render_init(renderer, cache)
    }
}

impl<T: CachedNonReactiveValueRenderInit<VK, Cache>, VK: ?Sized + ValueKind, Cache>
    ReactiveValueRenderInitPinned<VK, State<VK, Cache>> for RenderInit<T>
{
    type RenderInitPinned<R: FnOnce(<VK as ValueKind>::Value<'_>) -> Out, Out> = Self;
}

#[derive(Debug)]
pub struct Uncached<T>(pub T);

pub struct CloneIfCacheMiss<T>(pub T);
