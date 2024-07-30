use std::marker::PhantomData;

use crate::{element_types::RenderStateWithPehKind, html::behavior_type_traits, render_state::non_reactive::NonReactiveRenderState, RenderHtml, RenderStateKindPinned, RenderStateKindUnpinned};

pub enum KindOfNoState {}

impl RenderStateKindPinned for KindOfNoState {
    type RenderState<R: RenderHtml + ?Sized> = ();
}
impl RenderStateKindUnpinned for KindOfNoState {
    type UnpinnedRenderState<R: RenderHtml + ?Sized> = ();
}

impl<ElType: ?Sized + behavior_type_traits::Element> RenderStateWithPehKind<ElType> for KindOfNoState {
    type RenderStateWithPeh<R: RenderHtml + ?Sized> = ();
    type RenderStateWithPehUnpinned<R: RenderHtml + ?Sized> = ();
}

pub struct KindOfNonReactive<T: Default>(KindOfNoState, PhantomData<T>);

impl<T: Default> RenderStateKindPinned for KindOfNonReactive<T> {
    type RenderState<R: RenderHtml + ?Sized> = NonReactiveRenderState<T>;
}
impl<T: Default> RenderStateKindUnpinned for KindOfNonReactive<T> {
    type UnpinnedRenderState<R: RenderHtml + ?Sized> = NonReactiveRenderState<T>;
}

impl<T: Default, ElType: ?Sized + behavior_type_traits::Element> RenderStateWithPehKind<ElType> for KindOfNonReactive<T> {
    type RenderStateWithPeh<R: RenderHtml + ?Sized> = NonReactiveRenderState<T>;
    type RenderStateWithPehUnpinned<R: RenderHtml + ?Sized> = NonReactiveRenderState<T>;
}
