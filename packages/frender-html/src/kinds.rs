use crate::{html::behavior_type_traits, RenderHtml};

pub enum KindOfNoState {}

impl crate::RenderStateKindPinned for KindOfNoState {
    type RenderState<R: RenderHtml + ?Sized> = ();
}
impl crate::RenderStateKindUnpinned for KindOfNoState {
    type UnpinnedRenderState<R: RenderHtml + ?Sized> = ();
}

impl<ElType: ?Sized + behavior_type_traits::Element> crate::element_types::RenderStateWithPehKind<ElType> for KindOfNoState {
    type RenderStateWithPeh<R: RenderHtml + ?Sized> = ();
    type RenderStateWithPehUnpinned<R: RenderHtml + ?Sized> = ();
}
