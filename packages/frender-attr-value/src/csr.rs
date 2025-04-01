use crate::AttrValueKind;

pub trait UpdateAttrValue {
    type Kind: ?Sized + AttrValueKind;

    fn set(self, value: <Self::Kind as AttrValueKind>::AttrValue<'_>);

    fn remove(self);
}

pub trait CsrAttrValueState {
    /// Returning `true` implies that the attribute is absent.
    /// Returning `false` implies that the attribute might be absent.
    ///
    /// This method is to optimize `impl CsrAttrValue for Option, Either`
    ///
    /// Always returning `false` is correct.
    fn attribute_is_known_as_absent(&self) -> bool {
        false
    }
}

/// Unlike CsrStyle and CsrDomTokens, AttrValue doesn't allow chaining.
/// Thus, [`CsrAttrValue`] doesn't have method `remove_with_state`.
pub trait CsrAttrValue<AK: AttrValueKind>: Sized {
    type State: CsrAttrValueState;

    fn render_init_on_absent_attribute(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
    ) -> Self::State;

    fn render_init(this: Self, updater: impl UpdateAttrValue<Kind = AK>) -> Self::State;

    fn render_init_by_reusing_on_absent_attribute(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
        state: &mut Self::State,
    );

    fn render_init_by_reusing(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
        state: &mut Self::State,
    );

    fn render_update(this: Self, updater: impl UpdateAttrValue<Kind = AK>, state: &mut Self::State);
}

pub(crate) mod cached_some;

pub(crate) mod const_some;
