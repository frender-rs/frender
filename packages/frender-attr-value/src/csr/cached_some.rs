use frender_reactive_value::{
    non_reactive::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit as _},
    value_kind::ValueKind,
};

use crate::{values::cached_some::CachedSome, AttrValueKind};

use super::{CsrAttrValue, CsrAttrValueState, UpdateAttrValue};

pub trait AttrValueKindWithReactiveValueKind<VK: ValueKind>: AttrValueKind {
    fn reactive_value_into_attr_value(value: VK::Value<'_>) -> Self::AttrValue<'_>;
}

fn render_some<AK: AttrValueKindWithReactiveValueKind<VK>, VK: ValueKind>(
    updater: impl UpdateAttrValue<Kind = AK>,
) -> impl FnOnce(VK::Value<'_>) {
    |v| {
        updater.set(AK::reactive_value_into_attr_value(v));
    }
}

pub struct State<Cache>(Cache);

impl<Cache> CsrAttrValueState for State<Cache> {}

impl<T: CachedNonReactiveValue<VK>, AK: AttrValueKindWithReactiveValueKind<VK>, VK: ValueKind>
    CsrAttrValue<AK> for CachedSome<T, VK>
{
    type State = State<T::Cache>;

    fn render_init_on_absent_attribute(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
    ) -> Self::State {
        Self::render_init(this, updater)
    }

    fn render_init(this: Self, updater: impl UpdateAttrValue<Kind = AK>) -> Self::State {
        let this = this.0;
        let (mut cache, render_init) = this.into_cache_and_render_init();

        render_init.cached_non_reactive_value_render_init(render_some(updater), &mut cache);

        State(cache)
    }

    fn render_update(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
        State(cache): &mut Self::State,
    ) {
        let this = this.0;
        _ = this.maybe_update_into_cache_and_render(render_some(updater), cache)
    }

    fn render_init_by_reusing_on_absent_attribute(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
        state: &mut Self::State,
    ) {
        Self::render_init_by_reusing(this, updater, state);
    }

    fn render_init_by_reusing(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
        State(cache): &mut Self::State,
    ) {
        let this = this.0;
        this.update_into_cache_and_render(render_some(updater), cache)
    }
}
