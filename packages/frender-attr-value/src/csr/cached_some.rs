use frender_reactive_value::{
    non_reactive::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit as _},
    temp_ref::TempRef,
    value_kind::{KindOfTempRef, ValueKind},
};

use crate::{AttrKindOfStr, AttrValue, AttrValueKind};

use super::{CsrAttrValue, UpdateAttrValue};

pub trait AttrValueKindWithReactiveValueKind: AttrValueKind {
    type ReactiveValueKind: ValueKind;
    fn reactive_value_into_attr_value(
        value: <Self::ReactiveValueKind as ValueKind>::Value<'_>,
    ) -> Self::AttrValue<'_>;
}

impl AttrValueKindWithReactiveValueKind for AttrKindOfStr {
    type ReactiveValueKind = KindOfTempRef<str>;

    fn reactive_value_into_attr_value(
        TempRef(value): <Self::ReactiveValueKind as ValueKind>::Value<'_>,
    ) -> Self::AttrValue<'_> {
        value
    }
}

pub(crate) trait ImplCsrAttrValueWithCachedSome {}
pub(crate) trait CsrAttrValueCachedSome<AK: AttrValueKindWithReactiveValueKind>:
    ImplCsrAttrValueWithCachedSome + AttrValue<AK> + CachedNonReactiveValue<AK::ReactiveValueKind>
{
}

fn render_some<AK: AttrValueKindWithReactiveValueKind>(
    updater: impl UpdateAttrValue<Kind = AK>,
) -> impl FnOnce(<AK::ReactiveValueKind as ValueKind>::Value<'_>) {
    |v| {
        updater.set(AK::reactive_value_into_attr_value(v));
    }
}

impl<T: CsrAttrValueCachedSome<AK>, AK: AttrValueKindWithReactiveValueKind> CsrAttrValue<AK> for T {
    type State = T::Cache;

    fn update_attribute_value_into_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
    ) -> Self::State {
        let (mut cache, render_init) = this.into_cache_and_render_init();

        render_init.cached_non_reactive_value_render_init(render_some(updater), &mut cache);

        cache
    }

    fn can_skip_update(this: &Self, cache: &Self::State) -> bool {
        this.match_cache(cache)
    }

    fn force_update_attribute_value_with_state(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
        cache: &mut Self::State,
    ) {
        this.update_into_cache_and_render(render_some(updater), cache)
    }
}
