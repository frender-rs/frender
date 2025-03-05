use std::pin::Pin;

use frender_csr::{
    CsrElement, RenderStateKind,
    experimental::{
        PinnedStateOfKind, PinnedUiHandleOfKind, RenderHtml, UnpinnedStateOfKind,
        UnpinnedUiHandleOfKind,
    },
};

pub trait CsrElementRenderUpdate {
    type RenderStateKind: RenderStateKind;

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    );

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: &mut UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    );
}

impl<E: CsrElement> CsrElementRenderUpdate for E {
    type RenderStateKind = <E as CsrElement>::RenderStateKind;

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        <E as CsrElement>::pinned_render_update(self, renderer, state, ui_handle)
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: &mut UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        <E as CsrElement>::unpinned_render_update(self, renderer, state, ui_handle)
    }
}

pub trait AsMutCsrElementWithValue<V: ?Sized> {
    type ElementWithValueRenderStateKind: RenderStateKind;

    type ElementWithValue<'a>: CsrElementRenderUpdate<
        RenderStateKind = Self::ElementWithValueRenderStateKind,
    >
    where
        Self: 'a,
        V: 'a;

    fn as_mut_csr_element_with_value<'a>(&'a mut self, value: &'a V) -> Self::ElementWithValue<'a>;
}

pub trait MapValueToCsrElement<V: ?Sized>: super::MapValueToElement<V> {
    type OwnedPart;
    type MutPart: AsMutCsrElementWithValue<V>;

    fn into_csr_parts(self) -> (Self::MutPart, Self::OwnedPart);

    type OwnedPartIntoCsrElement<'a>: CsrElement<
        RenderStateKind = <Self::MutPart as AsMutCsrElementWithValue<V>>::ElementWithValueRenderStateKind
    >
    where
        Self: 'a,
        V: 'a;

    fn owned_part_into_csr_element<'a>(
        mut_part: &'a mut Self::MutPart,
        value: &'a V,
        owned_part: Self::OwnedPart,
    ) -> Self::OwnedPartIntoCsrElement<'a>;
}

macro_rules! impl_MapValueToCsrElement_with_Self {
    (
        type Value = $Value:ty;
    ) => {
        type OwnedPart = ();
        type MutPart = Self;

        fn into_csr_parts(self) -> (Self::MutPart, Self::OwnedPart) {
            (self, ())
        }

        type OwnedPartIntoCsrElement<'a> = <Self as $crate::share_value::element::bound::csr::AsMutCsrElementWithValue<$Value>>::ElementWithValue<'a>
        where
            Self: 'a,
            $Value: 'a;

        fn owned_part_into_csr_element<'a>(
            mut_part: &'a mut Self::MutPart,
            value: &'a $Value,
            (): Self::OwnedPart,
        ) -> Self::OwnedPartIntoCsrElement<'a> {
            Self::as_mut_csr_element_with_value(mut_part, value)
        }
    };
}

pub(crate) use impl_MapValueToCsrElement_with_Self;
